// projects/products/stable/core/engine/src/ws/backend_registration.rs
use protocol::protocol_id::ProtocolId;
use serde::Deserialize;

/// Backend service registration payload
#[derive(Debug, Deserialize)]
pub(crate) struct BackendRegistration {
    pub(crate) product_id: ProtocolId,
    pub(crate) instance_id: ProtocolId,
    pub(crate) capabilities: Vec<String>,
    pub(crate) routes: Vec<String>,
}
