//! UPnP-IGD and NAT-PMP port mapping client

use std::net::SocketAddr;
use std::time::Duration;
use tracing::{debug, error, info, warn};

/// UPnP Port Mapping Protocol Configuration
#[derive(Debug, Clone)]
pub struct UpnpConfig {
    pub enabled: bool,
    pub lease_duration_secs: u32,
    pub timeout: Duration,
}

impl Default for UpnpConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            lease_duration_secs: 7200,
            timeout: Duration::from_millis(2000),
        }
    }
}

/// UPnP IGD Client
pub struct UpnpClient {
    config: UpnpConfig,
}

impl UpnpClient {
    pub fn new(config: UpnpConfig) -> Self {
        Self { config }
    }

    /// Build SSDP M-SEARCH discovery packet for IGD
    pub fn build_ssdp_discover_payload() -> &'static str {
        "M-SEARCH * HTTP/1.1\r\n\
         HOST: 239.255.255.250:1900\r\n\
         MAN: \"ssdp:discover\"\r\n\
         MX: 2\r\n\
         ST: urn:schemas-upnp-org:device:InternetGatewayDevice:1\r\n\
         \r\n"
    }

    /// Build SOAP XML payload for AddPortMapping
    pub fn build_soap_add_port_mapping(
        external_port: u16,
        internal_ip: &str,
        internal_port: u16,
        protocol: &str,
        description: &str,
        lease_duration: u32,
    ) -> String {
        format!(
            "<?xml version=\"1.0\"?>\r\n\
             <s:Envelope xmlns:s=\"http://schemas.xmlsoap.org/soap/envelope/\" \
             s:encodingStyle=\"http://schemas.xmlsoap.org/soap/encoding/\">\r\n\
             <s:Body>\r\n\
             <u:AddPortMapping xmlns:u=\"urn:schemas-upnp-org:service:WANIPConnection:1\">\r\n\
               <NewRemoteHost></NewRemoteHost>\r\n\
               <NewExternalPort>{}</NewExternalPort>\r\n\
               <NewProtocol>{}</NewProtocol>\r\n\
               <NewInternalPort>{}</NewInternalPort>\r\n\
               <NewInternalClient>{}</NewInternalClient>\r\n\
               <NewEnabled>1</NewEnabled>\r\n\
               <NewPortMappingDescription>{}</NewPortMappingDescription>\r\n\
               <NewLeaseDuration>{}</NewLeaseDuration>\r\n\
             </u:AddPortMapping>\r\n\
             </s:Body>\r\n\
             </s:Envelope>",
            external_port, protocol, internal_port, internal_ip, description, lease_duration
        )
    }

    /// Build SOAP XML payload for DeletePortMapping
    pub fn build_soap_delete_port_mapping(external_port: u16, protocol: &str) -> String {
        format!(
            "<?xml version=\"1.0\"?>\r\n\
             <s:Envelope xmlns:s=\"http://schemas.xmlsoap.org/soap/envelope/\" \
             s:encodingStyle=\"http://schemas.xmlsoap.org/soap/encoding/\">\r\n\
             <s:Body>\r\n\
             <u:DeletePortMapping xmlns:u=\"urn:schemas-upnp-org:service:WANIPConnection:1\">\r\n\
               <NewRemoteHost></NewRemoteHost>\r\n\
               <NewExternalPort>{}</NewExternalPort>\r\n\
               <NewProtocol>{}</NewProtocol>\r\n\
             </u:DeletePortMapping>\r\n\
             </s:Body>\r\n\
             </s:Envelope>",
            external_port, protocol
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upnp_soap_payload_formatting() {
        let payload = UpnpClient::build_soap_add_port_mapping(
            51820,
            "192.168.1.100",
            51820,
            "UDP",
            "oxide-mesh-net",
            7200,
        );

        assert!(payload.contains("<NewExternalPort>51820</NewExternalPort>"));
        assert!(payload.contains("<NewInternalClient>192.168.1.100</NewInternalClient>"));
        assert!(payload.contains("<NewProtocol>UDP</NewProtocol>"));

        let delete_payload = UpnpClient::build_soap_delete_port_mapping(51820, "UDP");
        assert!(delete_payload.contains("<NewExternalPort>51820</NewExternalPort>"));
        assert!(delete_payload.contains("<NewProtocol>UDP</NewProtocol>"));
    }
}
