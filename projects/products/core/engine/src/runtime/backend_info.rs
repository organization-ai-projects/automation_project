// projects/products/core/engine/src/runtime/backend_info.rs

/// Information about a registered backend service
#[derive(Debug, Clone)]
pub struct BackendInfo {
    pub product_id: String,
    pub instance_id: String,
    pub capabilities: Vec<String>,
    pub routes: Vec<String>,
}
