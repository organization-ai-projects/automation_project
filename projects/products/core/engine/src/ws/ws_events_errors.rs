// projects/products/core/engine/src/ws/ws_events_errors.rs
use protocol::{Event, EventType, EventVariant, Metadata};

use crate::ws::{WsEventArgs, WsEventErrorDetails, ws_event};
use common::custom_uuid::Id128;

// Update ws_event_error to use the new error details struct
pub fn ws_event_error(
    meta: &Metadata,
    http_code: u16,
    code: i32,
    msg: impl Into<String> + Clone,
) -> Event {
    let error_details = WsEventErrorDetails {
        http_code,
        code,
        message: msg.clone().into(),
    };

    let args = WsEventArgs {
        meta: meta.clone(),
        name: "Error".to_string(),
        event_type: EventType::Error,
        variant: EventVariant::Error {
            id: Id128::from_hex(&meta.to_key()).expect("Invalid Id128 format"),
            message: msg.into(),
        },
        payload: None,
        error_details: Some(error_details),
    };

    ws_event(args)
}
