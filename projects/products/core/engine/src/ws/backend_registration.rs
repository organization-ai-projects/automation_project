// projects/products/core/engine/src/ws/backend_registration.rs
use serde::Deserialize;

/// Backend service registration payload
#[derive(Debug, Deserialize)]
pub struct BackendRegistration {
    pub product_id: String,
    pub instance_id: String,
    pub capabilities: Vec<String>,
    pub routes: Vec<String>,
}
