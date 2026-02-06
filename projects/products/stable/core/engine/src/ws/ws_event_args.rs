// projects/products/stable/core/engine/src/ws/ws_event_args.rs
use protocol::{EventType, EventVariant, Metadata, Payload};

use crate::ws::WsEventErrorDetails;

// Update WsEventArgs to use Payload directly
pub(crate) struct WsEventArgs {
    pub(crate) meta: Metadata,
    pub(crate) name: String,
    pub(crate) event_type: EventType,
    pub(crate) variant: EventVariant,
    pub(crate) payload: Option<Payload>,
    pub(crate) error_details: Option<WsEventErrorDetails>,
}

impl WsEventArgs {
    pub(crate) fn new(
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
