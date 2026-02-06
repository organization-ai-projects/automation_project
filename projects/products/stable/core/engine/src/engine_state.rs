// projects/products/stable/core/engine/src/engine_state.rs
use std::sync::Arc;

use security::TokenService;
use tokio::sync::RwLock;

use crate::Registry;
use crate::runtime::BackendRegistry;
use protocol::{Event, ProtocolId};
use std::collections::HashMap;
use tokio::sync::oneshot;

#[derive(Clone)]
pub(crate) struct EngineState {
    pub(crate) registry: Arc<RwLock<Registry>>,
    pub(crate) token_service: Arc<TokenService>,
    pub(crate) backend_registry: Arc<RwLock<BackendRegistry>>,
    pub(crate) pending_requests: Arc<RwLock<HashMap<ProtocolId, oneshot::Sender<Event>>>>,
}

impl EngineState {
    pub(crate) fn new(registry: Registry, token_service: TokenService) -> Self {
        Self {
            registry: Arc::new(RwLock::new(registry)),
            token_service: Arc::new(token_service),
            backend_registry: Arc::new(RwLock::new(BackendRegistry::new())),
            pending_requests: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
