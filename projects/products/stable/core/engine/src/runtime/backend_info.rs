// projects/products/stable/core/engine/src/runtime/backend_info.rs
use protocol::protocol_id::ProtocolId;

/// Information about a registered backend service
#[derive(Debug, Clone)]
pub(crate) struct BackendInfo {
    pub(crate) product_id: ProtocolId,
    pub(crate) instance_id: ProtocolId,
    pub(crate) capabilities: Vec<String>,
    pub(crate) routes: Vec<String>,
}
