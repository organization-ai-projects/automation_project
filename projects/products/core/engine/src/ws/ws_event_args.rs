// projects/products/core/engine/src/ws/ws_event_args.rs
use protocol::{EventType, EventVariant, Metadata, Payload};

use crate::ws::WsEventErrorDetails;

// Update WsEventArgs to use Payload directly
pub struct WsEventArgs {
    pub meta: Metadata,
    pub name: String,
    pub event_type: EventType,
    pub variant: EventVariant,
    pub payload: Option<Payload>,
    pub error_details: Option<WsEventErrorDetails>,
}

impl WsEventArgs {
    pub fn new(
        meta: Metadata,
        name: impl Into<String>,
        event_type: EventType,
        variant: EventVariant,
        payload: Option<Payload>,
        error_details: Option<WsEventErrorDetails>,
    ) -> Self {
        Self {
            meta,
            name: name.into(),
            event_type,
            variant,
            payload,
            error_details,
        }
    }
}
