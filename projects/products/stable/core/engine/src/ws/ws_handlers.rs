// projects/products/core/engine/src/ws/ws_handlers.rs
use crate::{
    EngineState, WS_IDLE_TIMEOUT, WS_MAX_MESSAGE_BYTES, WS_PING_EVERY,
    ws::{BackendRegistration, route_command, ws_event_error, ws_event_ok},
};
use common_json::{JsonSerializable, from_json_str, from_value};
use futures_util::{SinkExt, StreamExt};
use protocol::{Command, Event, Metadata};
use tracing::{info, warn};
use warp::ws::{Message, WebSocket};

// -------------------------
// Client-facing constants
// -------------------------
const ERR_UNAUTHORIZED: &str = "Unauthorized";
const ERR_MSG_TOO_LARGE: &str = "Message too large";
const ERR_INVALID_JSON: &str = "Invalid Command JSON";
const ERR_SERIALIZATION: &str = "Failed to serialize event";

// HTTP + app codes (stable contract with the front)
const HTTP_UNAUTHORIZED: u16 = 401;
const CODE_UNAUTHORIZED: i32 = 2000;

const HTTP_TOO_LARGE: u16 = 413;
const CODE_TOO_LARGE: i32 = 2101;

const HTTP_BAD_JSON: u16 = 400;
const CODE_BAD_JSON: i32 = 2100;

const HTTP_SERIALIZE: u16 = 500;
const CODE_SERIALIZE: i32 = 2200;

// -------------------------
// Small helpers (no magic)
// -------------------------
#[inline]
fn safe_json(ev: &impl JsonSerializable) -> String {
    ev.to_json_string().unwrap_or_else(|_| "{}".to_string())
}

#[inline]
async fn send_event(tx: &mut (impl SinkExt<Message> + Unpin), ev: &impl JsonSerializable) -> bool {
    tx.send(Message::text(safe_json(ev))).await.is_ok()
}

#[inline]
fn meta_now() -> Metadata {
    // If your protocol::Metadata doesn't have now(), replace with default()
    // and consider adding Metadata::now() in protocol for better correlation.
    Metadata::now()
}

pub(crate) async fn ws_handle(socket: WebSocket, state: EngineState, jwt: String) {
    let (mut tx, mut rx) = socket.split();
    let (out_tx, mut out_rx) = tokio::sync::mpsc::unbounded_channel::<String>();

    // --------
    // Auth
    // --------
    let token = match state.token_service.verify(&jwt) {
        Ok(t) => {
            info!(subject_id = %t.subject_id, role = ?t.role, "WS auth success");
            t
        }
        Err(e) => {
            warn!(error = %e, "WS auth failed");

            let ev = ws_event_error(
                &Metadata::default(), // no command metadata at this stage
                HTTP_UNAUTHORIZED,
                CODE_UNAUTHORIZED,
                format!("{ERR_UNAUTHORIZED}: {e}"),
            );

            let _ = send_event(&mut tx, &ev).await;
            let _ = tx.send(Message::close()).await;
            return;
        }
    };

    let perms = token.role.permissions();
    info!(
        subject_id = %token.subject_id,
        role = ?token.role,
        permissions = ?perms,
        "WS authenticated"
    );

    // timers
    let mut ping = tokio::time::interval(WS_PING_EVERY);
    let mut last_activity = tokio::time::Instant::now();

    loop {
        tokio::select! {
            _ = ping.tick() => {
                // Close idle connections
                if last_activity.elapsed() > WS_IDLE_TIMEOUT {
                    warn!(subject_id = %token.subject_id, "WS idle timeout, closing connection");
                    let _ = tx.send(Message::close()).await;
                    break;
                }

                // Keep-alive
                if tx.send(Message::ping(Vec::new())).await.is_err() {
                    info!(subject_id = %token.subject_id, "WS connection closed during ping");
                    break;
                }
            }

            msg = out_rx.recv() => {
                if let Some(text) = msg
                    && tx.send(Message::text(text)).await.is_err() {
                        info!(subject_id = %token.subject_id, "WS connection closed during backend send");
                        break;
                    }
            }

            msg = rx.next() => {
                let msg = match msg {
                    Some(Ok(m)) => m,
                    Some(Err(e)) => {
                        warn!(subject_id = %token.subject_id, error = %e, "WS error receiving message");
                        break;
                    }
                    None => {
                        info!(subject_id = %token.subject_id, "WS connection closed by client");
                        break;
                    }
                };

                if msg.is_close() {
                    info!(subject_id = %token.subject_id, "WS close frame received");
                    break;
                }

                // Any frame counts as activity
                last_activity = tokio::time::Instant::now();

                // Respond to client ping
                if msg.is_ping() {
                    if tx.send(Message::pong(Vec::new())).await.is_err() {
                        info!(subject_id = %token.subject_id, "WS connection closed during pong");
                        break;
                    }
                    continue;
                }
                if msg.is_pong() {
                    continue;
                }

                // Ignore binary
                if msg.is_binary() {
                    warn!(subject_id = %token.subject_id, "WS binary message ignored");
                    continue;
                }

                // Size limit (bytes) before parsing
                let size = msg.as_bytes().len();
                if size > WS_MAX_MESSAGE_BYTES {
                    warn!(subject_id = %token.subject_id, bytes = size, "WS message too large");
                    let ev = ws_event_error(&meta_now(), HTTP_TOO_LARGE, CODE_TOO_LARGE, ERR_MSG_TOO_LARGE);
                    let _ = send_event(&mut tx, &ev).await;
                    continue;
                }

                // UTF-8 text
                let text = match msg.to_str() {
                    Ok(t) => t,
                    Err(_) => {
                        warn!(subject_id = %token.subject_id, "WS invalid UTF-8 message");
                        continue;
                    }
                };

                // Event -> pending HTTP forwarders
                if let Ok(event) = from_json_str::<Event>(text) {
                    let request_id = event.metadata.request_id;
                    if let Some(tx) = state.pending_requests.write().await.remove(&request_id) {
                        let _ = tx.send(event);
                    }
                    continue;
                }

                // JSON -> Command
                let cmd: Command = match from_json_str(text) {
                    Ok(c) => c,
                    Err(e) => {
                        warn!(subject_id = %token.subject_id, error = %e, "WS invalid JSON");
                        let ev = ws_event_error(
                            &meta_now(),
                            HTTP_BAD_JSON,
                            CODE_BAD_JSON,
                            format!("{ERR_INVALID_JSON}: {e}")
                        );
                        let _ = send_event(&mut tx, &ev).await;
                        continue;
                    }
                };

                // Correlate to command
                let meta = cmd.metadata.clone();

                // Backend registration shortcut (capture sender)
                if cmd.action.as_deref() == Some("backend.hello") {
                    let payload_value = match cmd.payload.as_ref().and_then(|p| p.payload.as_ref()) {
                        Some(v) => v.clone(),
                        None => {
                            let ev = ws_event_error(&meta, HTTP_BAD_JSON, CODE_BAD_JSON, "Missing payload");
                            let _ = send_event(&mut tx, &ev).await;
                            continue;
                        }
                    };

                    let registration: BackendRegistration = match from_value(payload_value) {
                        Ok(r) => r,
                        Err(e) => {
                            let ev = ws_event_error(&meta, HTTP_BAD_JSON, CODE_BAD_JSON, format!("Invalid payload: {e}"));
                            let _ = send_event(&mut tx, &ev).await;
                            continue;
                        }
                    };

                    let mut backends = state.backend_registry.write().await;
                    backends.register_with_sender(
                        registration.product_id,
                        registration.instance_id,
                        registration.capabilities,
                        registration.routes,
                        out_tx.clone(),
                    );
                    info!(
                        registered_backends = backends.count(),
                        "Backend registration accepted"
                    );

                    let ev = ws_event_ok(&meta, "BackendRegistered");
                    let _ = send_event(&mut tx, &ev).await;
                    continue;
                }

                // Route
                let ev = route_command(cmd, &state, &token, perms).await;

                // Serialize event, fallback if it fails
                let out = ev.to_json_string().unwrap_or_else(|e| {
                    warn!(subject_id = %token.subject_id, error = %e, "WS failed to serialize event");
                    let fallback = ws_event_error(&meta, HTTP_SERIALIZE, CODE_SERIALIZE, ERR_SERIALIZATION);
                    fallback.to_json_string().unwrap_or_else(|_| "{\"name\":\"Error\"}".to_string())
                });

                if tx.send(Message::text(out)).await.is_err() {
                    info!(subject_id = %token.subject_id, "WS connection closed during send");
                    break;
                }
            }
        }
    }

    info!(subject_id = %token.subject_id, "WS connection closed");
}
