// projects/products/unstable/protocol_builder/backend/src/schema/endpoint_spec.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointSpec {
    pub name: String,
    pub request: String,
    pub response: String,
}
