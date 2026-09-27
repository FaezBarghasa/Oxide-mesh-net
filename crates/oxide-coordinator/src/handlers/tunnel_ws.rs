//! Actix Web WebSocket Reverse-Tunnel Fallback Handler (/ws/v1/telemetry)
//!
//! Provides seamless fallback data transport disguised as standard browser telemetry
//! with authentic HTTP headers and binary WebSocket packet encapsulation.

use actix_web::{HttpRequest, HttpResponse, web};
use actix_ws::Message;
use futures_util::StreamExt;
use oxide_protocol::wire::{PreParseVerdict, WirePacket, pre_parse_packet};
use tracing::{debug, info, warn};

/// WebSocket Fallback Tunnel endpoint disguised as telemetry stream
pub async fn ws_telemetry_tunnel(
    req: HttpRequest,
    body: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    debug!(
        "WSS Telemetry reverse-tunnel established from {}",
        req.peer_addr().map(|a| a.to_string()).unwrap_or_default()
    );

    actix_rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Binary(bin) => {
                    // Pre-parse the encapsulated wire frame
                    match pre_parse_packet(&bin) {
                        PreParseVerdict::ValidData {
                            packet_id,
                            payload_len,
                        } => {
                            debug!(
                                "WSS Tunnel received valid data frame (ID: {}, Len: {})",
                                packet_id, payload_len
                            );
                            // Inbound tunnel payload forwarding
                            if let Ok(_wire) = WirePacket::from_bytes(&bin) {
                                // Handled by coordinator packet broker
                            }
                        }
                        PreParseVerdict::ValidControl {
                            packet_id,
                            payload_len,
                        } => {
                            debug!(
                                "WSS Tunnel received control frame (ID: {}, Len: {})",
                                packet_id, payload_len
                            );
                        }
                        PreParseVerdict::JunkIgnored => {
                            debug!("WSS Tunnel dropped junk frame");
                        }
                        PreParseVerdict::Malformed => {
                            warn!("WSS Tunnel dropped malformed frame");
                        }
                    }
                }
                Message::Text(txt) => {
                    // Handle telemetry ping or heartbeat
                    if txt.contains("ping") {
                        let _ = session
                            .text(r#"{"status":"ok","type":"telemetry_ack"}"#)
                            .await;
                    }
                }
                Message::Ping(bytes) => {
                    let _ = session.pong(&bytes).await;
                }
                Message::Close(reason) => {
                    info!("WSS Tunnel closed by peer: {:?}", reason);
                    break;
                }
                _ => {}
            }
        }
    });

    Ok(response)
}

/// Configure tunnel routes
pub fn configure_tunnel_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ws/v1/telemetry").route(web::get().to(ws_telemetry_tunnel)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, test};

    #[actix_rt::test]
    async fn test_ws_telemetry_tunnel_route_registration() {
        let app = test::init_service(App::new().configure(configure_tunnel_routes)).await;

        let req = test::TestRequest::get()
            .uri("/ws/v1/telemetry")
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Non-websocket upgrade GET returns 400 Bad Request
        assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
    }
}
