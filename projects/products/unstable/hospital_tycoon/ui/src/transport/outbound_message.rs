// projects/products/unstable/hospital_tycoon/ui/src/transport/outbound_message.rs
use crate::transport::outbound_request::OutboundRequest;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct OutboundMessage {
    pub id: u64,
    pub request: OutboundRequest,
}
