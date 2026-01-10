use std::collections::HashMap;

// Backend registry implementation

#[derive(Debug)]
pub struct BackendInfo {
    pub product_id: String,
    pub instance_id: String,
    pub capabilities: Vec<String>,
    pub routes: Vec<String>,
}

pub struct BackendRegistry {
    backends: HashMap<String, BackendInfo>, // Key: instance_id
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
