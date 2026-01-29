// projects/products/core/engine/src/runtime/backend_registry_impl.rs
use std::collections::HashMap;

use super::BackendInfo;

/// Registry for managing connected backend services
pub struct BackendRegistry {
    backends: HashMap<String, BackendInfo>,
}

impl BackendRegistry {
    pub fn new() -> Self {
        Self {
            backends: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        product_id: String,
        instance_id: String,
        capabilities: Vec<String>,
        routes: Vec<String>,
    ) {
        let backend_info = BackendInfo {
            product_id,
            instance_id: instance_id.clone(),
            capabilities,
            routes,
        };
        self.backends.insert(instance_id, backend_info);
    }
}

impl Default for BackendRegistry {
    fn default() -> Self {
        Self::new()
    }
}
