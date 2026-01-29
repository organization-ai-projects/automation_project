// projects/products/core/engine/src/routes/http_forwarder.rs
//! Forward HTTP requests to backends via WS Commands through route_command.
//!
//! This module adapts HTTP request/response to the Command/Event protocol.

use protocol::protocol_id::ProtocolId;
use protocol::{Command, CommandType, Event, Metadata, Payload};
use std::str::FromStr;
use tokio::sync::oneshot;
use tokio::time::Duration;

use crate::engine_state::EngineState;

const DEFAULT_BACKEND_TIMEOUT_MS: u64 = 5_000;

pub(crate) fn accounts_product_id() -> Result<ProtocolId, String> {
    let raw = std::env::var("ENGINE_ACCOUNTS_PRODUCT_ID")
        .or_else(|_| std::env::var("ACCOUNTS_PRODUCT_ID"))
        .map_err(|_| "ACCOUNTS_PRODUCT_ID env var is required".to_string())?;
    ProtocolId::from_str(&raw).map_err(|e| format!("Invalid ACCOUNTS_PRODUCT_ID: {e}"))
}

pub(crate) fn payload_from<T: serde::Serialize>(value: T, payload_type: Option<&str>) -> Payload {
    let payload_value = common_json::to_value(&value).ok();
    Payload {
        payload_type: payload_type.map(|s| s.to_string()),
        payload: payload_value,
    }
}

/// Forward a request to a backend via the WS routing system.
/// Creates a Command, routes it, and extracts the response.
pub(crate) async fn forward_to_backend(
    product_id: &ProtocolId,
    action: &str,
    payload: Payload,
    state: &EngineState,
) -> Result<Event, String> {
    let metadata = Metadata::now();
    let request_id = metadata.request_id;

    let (tx, rx) = oneshot::channel();
    {
        let mut pending = state.pending_requests.write().await;
        pending.insert(request_id, tx);
    }

    let sender = {
        let registry = state.backend_registry.read().await;
        registry.sender_for_product(product_id)
    }
    .ok_or_else(|| "Backend not available".to_string())?;

    let cmd = Command {
        metadata,
        command_type: CommandType::StartJob,
        action: Some(action.to_string()),
        payload: Some(payload),
    };

    let cmd_json = common_json::to_string(&cmd).map_err(|e| e.to_string())?;
    if sender.send(cmd_json).is_err() {
        let mut pending = state.pending_requests.write().await;
        pending.remove(&request_id);
        return Err("Backend connection closed".to_string());
    }

    match tokio::time::timeout(Duration::from_millis(DEFAULT_BACKEND_TIMEOUT_MS), rx).await {
        Ok(Ok(event)) => Ok(event),
        Ok(Err(_)) => Err("Backend response channel closed".to_string()),
        Err(_) => {
            let mut pending = state.pending_requests.write().await;
            pending.remove(&request_id);
            Err("Backend request timed out".to_string())
        }
    }
}
