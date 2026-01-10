// projects/products/core/engine/src/ws/ws_events.rs
use protocol::{Event, EventType, EventVariant, Metadata, Payload};

use crate::ws::WsEventArgs;

#[inline]
fn non_empty_data(args: &WsEventArgs) -> String {
    args.payload
        .as_ref()
        .and_then(|p| p.payload.as_ref())
        .and_then(|v| {
            if v.is_object() || v.is_array() {
                Some(v.to_string())
            } else {
                None
            }
        })
        .or_else(|| {
            args.error_details
                .as_ref()
                .map(|e| e.message.clone())
                .filter(|s| !s.trim().is_empty())
        })
        .unwrap_or_else(|| "{}".to_string())
}

#[inline]
fn best_message(args: &WsEventArgs) -> Option<String> {
    args.error_details
        .as_ref()
        .filter(|ed| !ed.message.trim().is_empty())
        .map(|ed| ed.message.clone())
        .or_else(|| {
            args.payload
                .as_ref()
                .and_then(|p| p.payload.as_ref())
                .and_then(|v| {
                    if let Some(obj) = v.as_object() {
                        obj.get("message")
                            .and_then(|m| m.as_str())
                            .map(|s| s.to_string())
                    } else {
                        v.as_str().map(|s| s.to_string())
                    }
                })
                .filter(|s| !s.trim().is_empty())
        })
}

#[inline]
fn payload_json(payload_type: impl Into<String>, value: serde_json::Value) -> Payload {
    Payload {
        payload_type: Some(payload_type.into()),
        payload: Some(value),
    }
}

pub fn ws_event(args: WsEventArgs) -> Event {
    let data = non_empty_data(&args);
    let message = best_message(&args);

    Event {
        name: args.name,
        event_type: args.event_type,
        data,
        metadata: args.meta,
        payload: args.payload,
        level: None,
        message,
        pct: None,
        variant: args.variant,
    }
}

/// Standard OK: ACK + payload "ok" (ensures data is never empty).
pub fn ws_event_ok(meta: &Metadata, name: &str) -> Event {
    ws_event_ok_payload(
        meta,
        name,
        "ack",
        serde_json::Value::String("ok".to_string()),
    )
}

/// OK with typed payload.
pub fn ws_event_ok_payload(
    meta: &Metadata,
    name: &str,
    payload_type: &str,
    payload: serde_json::Value,
) -> Event {
    ws_event(WsEventArgs::new(
        meta.clone(),
        name,
        EventType::Acknowledgment,
        EventVariant::Acknowledged { id: meta.to_key() },
        Some(payload_json(payload_type, payload)),
        None,
    ))
}

// Contract:
// - `payload` is the source of truth.
// - `data` is always non-empty, using object/array from `payload` or `{}` as fallback.
// - `message` prioritizes `error_details` and falls back to `payload.message` if available.
