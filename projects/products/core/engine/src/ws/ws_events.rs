// projects/products/core/engine/src/ws/ws_events.rs
use common_json::{Json, JsonMap, number_i64, number_u64, to_json_string};
use protocol::{Event, EventType, EventVariant, Metadata, Payload};

use crate::ws::WsEventArgs;
use common::custom_uuid::Id128;

#[inline]
fn non_empty_data(args: &WsEventArgs, payload: &Option<Payload>) -> String {
    payload
        .as_ref()
        .and_then(|p| p.payload.as_ref())
        .and_then(|v| {
            if v.is_object() || v.is_array() {
                Some(to_json_string(v).unwrap_or_else(|_| "{}".to_string()))
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
fn payload_json(payload_type: impl Into<String>, value: Json) -> Payload {
    Payload {
        payload_type: Some(payload_type.into()),
        payload: Some(value),
    }
}

pub(crate) fn ws_event(args: WsEventArgs) -> Event {
    let payload = match &args.payload {
        Some(p) => Some(p.clone()),
        None => args.error_details.as_ref().map(|ed| {
            let mut map = JsonMap::new();
            map.insert("http_code".to_string(), number_u64(ed.http_code as u64));
            map.insert("code".to_string(), number_i64(ed.code as i64));
            map.insert("message".to_string(), Json::String(ed.message.clone()));
            payload_json("engine/error", Json::Object(map))
        }),
    };
    let data = non_empty_data(&args, &payload);
    let message = best_message(&args);

    Event {
        name: args.name,
        event_type: args.event_type,
        data,
        metadata: args.meta,
        payload,
        level: None,
        message,
        pct: None,
        variant: args.variant,
    }
}

/// Standard OK: ACK + payload "ok" (ensures data is never empty).
pub(crate) fn ws_event_ok(meta: &Metadata, name: &str) -> Event {
    ws_event_ok_payload(meta, name, "ack", Json::String("ok".to_string()))
}

/// OK with typed payload.
pub(crate) fn ws_event_ok_payload(
    meta: &Metadata,
    name: &str,
    payload_type: &str,
    payload: Json,
) -> Event {
    ws_event(WsEventArgs::new(
        meta.clone(),
        name,
        EventType::Acknowledgment,
        EventVariant::Acknowledged {
            id: Id128::from_hex(&meta.to_key()).expect("Invalid Id128 format"),
        },
        Some(payload_json(payload_type, payload)),
        None,
    ))
}

// Contract:
// - `payload` is the source of truth.
// - `data` is always non-empty, using object/array from `payload` or `{}` as fallback.
// - `message` prioritizes `error_details` and falls back to `payload.message` if available.
