//! projects/products/stable/accounts/backend/src/backend_hello.rs
use protocol::protocol_id::ProtocolId;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BackendHello {
    pub product_id: ProtocolId,
    pub instance_id: ProtocolId,
    pub capabilities: Vec<String>,
    pub routes: Vec<String>,
}
