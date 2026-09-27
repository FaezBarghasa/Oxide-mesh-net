//! Dynamic TCP MSS Clamping implementation for IPv4 and IPv6 packets
//!
//! Enforces path MTU constraints on TCP connections crossing TUN/overlay boundaries
//! by intercepting TCP SYN and SYN-ACK packets, rewriting the Maximum Segment Size (MSS)
//! option, and performing RFC 1624 incremental one's complement checksum updates.

use crate::error::{Result, TunError};

/// RFC 1624 Incremental 16-bit One's Complement Checksum Update
///
/// Formula: HC' = ~(~HC + ~m + m')
/// where:
/// - HC is the original checksum
/// - m is the original 16-bit word being replaced
/// - m' is the new 16-bit word
#[inline]
pub fn update_checksum_16(old_checksum: u16, old_val: u16, new_val: u16) -> u16 {
    let hc = (!old_checksum) as u32;
    let m = (!old_val) as u32;
    let m_prime = new_val as u32;

    let mut sum = hc + m + m_prime;
    while (sum >> 16) > 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }

    let res = !(sum as u16);
    // In one's complement TCP checksum, 0x0000 is represented as 0xFFFF
    if res == 0 { 0xFFFF } else { res }
}

/// Dynamic TCP MSS Clamper
///
/// Checks if an IPv4 or IPv6 packet is a TCP SYN or SYN-ACK with an MSS option exceeding `max_mss`.
/// If so, rewrites the MSS option in-place and updates the TCP checksum incrementally.
///
/// Returns `Ok(true)` if clamped, `Ok(false)` if no clamping was needed or packet was not a TCP SYN.
pub fn clamp_tcp_mss(packet: &mut [u8], max_mss: u16) -> Result<bool> {
    if packet.is_empty() {
        return Ok(false);
    }

    let version = (packet[0] >> 4) & 0x0F;
    match version {
        4 => clamp_ipv4_tcp_mss(packet, max_mss),
        6 => clamp_ipv6_tcp_mss(packet, max_mss),
        _ => Ok(false),
    }
}

fn clamp_ipv4_tcp_mss(packet: &mut [u8], max_mss: u16) -> Result<bool> {
    if packet.len() < 20 {
        return Ok(false);
    }

    let ihl = ((packet[0] & 0x0F) as usize) * 4;
    if ihl < 20 || packet.len() < ihl {
        return Ok(false);
    }

    let protocol = packet[9];
    if protocol != 6 {
        // Not TCP
        return Ok(false);
    }

    let total_len = u16::from_be_bytes([packet[2], packet[3]]) as usize;
    if packet.len() < total_len || total_len < ihl + 20 {
        return Ok(false);
    }

    let tcp_slice = &mut packet[ihl..total_len];
    clamp_tcp_segment(tcp_slice, max_mss)
}

fn clamp_ipv6_tcp_mss(packet: &mut [u8], max_mss: u16) -> Result<bool> {
    if packet.len() < 40 {
        return Ok(false);
    }

    let payload_len = u16::from_be_bytes([packet[4], packet[5]]) as usize;
    if packet.len() < 40 + payload_len {
        return Ok(false);
    }

    let mut next_header = packet[6];
    let mut offset = 40;

    // Follow IPv6 extension headers to find TCP (protocol 6)
    while offset < 40 + payload_len {
        match next_header {
            6 => {
                // TCP reached
                let tcp_slice = &mut packet[offset..40 + payload_len];
                return clamp_tcp_segment(tcp_slice, max_mss);
            }
            0 | 43 | 60 => {
                // Hop-by-Hop (0), Routing (43), Destination Options (60)
                if offset + 2 > packet.len() {
                    return Ok(false);
                }
                next_header = packet[offset];
                let ext_len = ((packet[offset + 1] as usize) + 1) * 8;
                offset += ext_len;
            }
            44 => {
                // Fragment header (fixed 8 bytes)
                if offset + 8 > packet.len() {
                    return Ok(false);
                }
                next_header = packet[offset];
                offset += 8;
            }
            _ => {
                // Other protocol or unrecognized extension header
                return Ok(false);
            }
        }
    }

    Ok(false)
}

fn clamp_tcp_segment(tcp: &mut [u8], max_mss: u16) -> Result<bool> {
    if tcp.len() < 20 {
        return Ok(false);
    }

    // Flags are at offset 13 (SYN is bit 1: 0x02)
    let flags = tcp[13];
    if (flags & 0x02) == 0 {
        // Not a SYN or SYN-ACK
        return Ok(false);
    }

    let data_offset = ((tcp[12] >> 4) as usize) * 4;
    if data_offset < 20 || tcp.len() < data_offset {
        return Ok(false);
    }

    if data_offset == 20 {
        // No TCP options present
        return Ok(false);
    }

    let mut opt_idx = 20;
    let mut modified = false;

    while opt_idx < data_offset {
        let kind = tcp[opt_idx];
        match kind {
            0 => {
                // End of option list
                break;
            }
            1 => {
                // NOP
                opt_idx += 1;
            }
            2 => {
                // Maximum Segment Size Option (Length = 4)
                if opt_idx + 4 > data_offset {
                    return Err(TunError::Internal("Malformed TCP MSS option length".into()));
                }
                let len = tcp[opt_idx + 1] as usize;
                if len != 4 {
                    return Err(TunError::Internal(format!(
                        "Invalid TCP MSS option len: {}",
                        len
                    )));
                }

                let current_mss = u16::from_be_bytes([tcp[opt_idx + 2], tcp[opt_idx + 3]]);
                if current_mss > max_mss {
                    let old_checksum = u16::from_be_bytes([tcp[16], tcp[17]]);
                    let new_checksum = update_checksum_16(old_checksum, current_mss, max_mss);

                    // Rewrite MSS in option
                    let new_mss_bytes = max_mss.to_be_bytes();
                    tcp[opt_idx + 2] = new_mss_bytes[0];
                    tcp[opt_idx + 3] = new_mss_bytes[1];

                    // Rewrite TCP Checksum
                    let checksum_bytes = new_checksum.to_be_bytes();
                    tcp[16] = checksum_bytes[0];
                    tcp[17] = checksum_bytes[1];

                    modified = true;
                }

                opt_idx += 4;
            }
            _ => {
                // Other options: kind (1 byte), length (1 byte), data (length - 2 bytes)
                if opt_idx + 1 >= data_offset {
                    break;
                }
                let len = tcp[opt_idx + 1] as usize;
                if len < 2 {
                    // Invalid option length to avoid infinite loop
                    break;
                }
                opt_idx += len;
            }
        }
    }

    Ok(modified)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compute_inet_checksum(data: &[u8]) -> u16 {
        let mut sum = 0u32;
        let mut chunks = data.chunks_exact(2);
        for chunk in &mut chunks {
            sum += u16::from_be_bytes([chunk[0], chunk[1]]) as u32;
        }
        if let Some(&last) = chunks.remainder().first() {
            sum += (last as u32) << 8;
        }
        while (sum >> 16) > 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }
        let res = !(sum as u16);
        if res == 0 { 0xFFFF } else { res }
    }

    #[test]
    fn test_rfc1624_checksum_incremental_update() {
        // Test RFC 1624 algebra correctness
        let mut data = vec![
            0x12, 0x34, 0x56, 0x78, // Arbitrary data
            0x05, 0xB4, // MSS = 1460 (0x05B4)
            0xAB, 0xCD,
        ];
        let original_checksum = compute_inet_checksum(&data);

        // Update 0x05B4 -> 0x04DC (1244)
        let old_val = 0x05B4;
        let new_val = 0x04DC;
        data[4] = 0x04;
        data[5] = 0xDC;

        let expected_checksum = compute_inet_checksum(&data);
        let updated_checksum = update_checksum_16(original_checksum, old_val, new_val);

        assert_eq!(updated_checksum, expected_checksum);
    }

    #[test]
    fn test_clamp_ipv4_tcp_syn() {
        // Minimal IPv4 TCP SYN packet with MSS=1460 option
        let mut packet = vec![
            // IPv4 Header (20 bytes)
            0x45, 0x00, 0x00, 0x2C, // IPv4, IHL=5, Total Length=44
            0x00, 0x01, 0x00, 0x00, // ID, Flags/Frag
            0x40, 0x06, 0x00, 0x00, // TTL=64, Protocol=6 (TCP), Header Checksum=0
            10, 0, 0, 1, // Src IP: 10.0.0.1
            10, 0, 0, 2, // Dst IP: 10.0.0.2
            // TCP Header (24 bytes: 20 base + 4 option)
            0x04, 0xD2, 0x00, 0x50, // Src Port: 1234, Dst Port: 80
            0x00, 0x00, 0x00, 0x01, // Seq: 1
            0x00, 0x00, 0x00, 0x00, // Ack: 0
            0x60, 0x02, 0x72, 0x10, // Data Offset=6 (24 bytes), Flags=SYN (0x02), Window
            0x1A, 0x2B, 0x00, 0x00, // Checksum (dummy), Urgent Pointer
            0x02, 0x04, 0x05, 0xB4, // Option: Kind=2 (MSS), Len=4, Value=1460 (0x05B4)
        ];

        let clamped = clamp_tcp_mss(&mut packet, 1300).unwrap();
        assert!(clamped);

        // Check new MSS is 1300 (0x0514)
        assert_eq!(packet[40], 0x02);
        assert_eq!(packet[41], 0x04);
        assert_eq!(packet[42], 0x05);
        assert_eq!(packet[43], 0x14);
    }

    #[test]
    fn test_clamp_unnecessary_mss() {
        let mut packet = vec![
            // IPv4 Header (20 bytes)
            0x45, 0x00, 0x00, 0x2C, 0x00, 0x01, 0x00, 0x00, 0x40, 0x06, 0x00, 0x00, 10, 0, 0, 1, 10,
            0, 0, 2, // TCP Header (24 bytes)
            0x04, 0xD2, 0x00, 0x50, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x60, 0x02,
            0x72, 0x10, 0x1A, 0x2B, 0x00, 0x00, 0x02, 0x04, 0x05, 0x00, // MSS = 1280 (0x0500)
        ];

        // Max MSS is 1300, packet MSS is 1280, so clamp should return false
        let clamped = clamp_tcp_mss(&mut packet, 1300).unwrap();
        assert!(!clamped);
        assert_eq!(packet[42], 0x05);
        assert_eq!(packet[43], 0x00);
    }
}
