// projects/products/stable/core/engine/src/runtime/backend_registry_impl.rs
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;

use crate::BackendConnection;
use protocol::protocol_id::ProtocolId;
use tracing::info;

use super::BackendInfo;

/// Registry for managing connected backend services
pub(crate) struct BackendRegistry {
    backends: HashMap<String, BackendConnection>,
    by_product: HashMap<String, String>,
}

impl BackendRegistry {
    pub(crate) fn new() -> Self {
        Self {
            backends: HashMap::new(),
            by_product: HashMap::new(),
        }
    }

    pub(crate) fn register_with_sender(
        &mut self,
        product_id: ProtocolId,
        instance_id: ProtocolId,
        capabilities: Vec<String>,
        routes: Vec<String>,
        sender: UnboundedSender<String>,
    ) {
        let backend_info = BackendInfo {
            product_id,
            instance_id,
            capabilities,
            routes,
        };
        info!(
            product_id = %backend_info.product_id,
            instance_id = %backend_info.instance_id,
            capabilities = backend_info.capabilities.len(),
            routes = backend_info.routes.len(),
            "Backend registered"
        );
        self.backends
            .insert(instance_id.to_hex(), BackendConnection { sender });
        self.by_product
            .insert(product_id.to_hex(), instance_id.to_hex());
    }

    pub(crate) fn sender_for_product(
        &self,
        product_id: &ProtocolId,
    ) -> Option<UnboundedSender<String>> {
        let instance_id = self.by_product.get(&product_id.to_hex())?;
        self.backends.get(instance_id).map(|b| b.sender.clone())
    }

    /// Count the number of registered backends
    pub(crate) fn count(&self) -> usize {
        self.backends.len()
    }
}

impl Default for BackendRegistry {
    fn default() -> Self {
        Self::new()
    }
}
