// projects/products/core/engine/src/engine_state.rs
use std::sync::Arc;

use security::TokenService;
use tokio::sync::RwLock;

use crate::Registry;
use crate::runtime::backend_registry::BackendRegistry;

#[derive(Clone)]
pub struct EngineState {
    pub registry: Arc<RwLock<Registry>>,
    pub token_service: Arc<TokenService>,
    pub backend_registry: Arc<RwLock<BackendRegistry>>, // Nouveau champ pour les backends connectés
}

impl EngineState {
    pub fn new(registry: Registry, token_service: TokenService) -> Self {
        Self {
            registry: Arc::new(RwLock::new(registry)),
            token_service: Arc::new(token_service),
            backend_registry: Arc::new(RwLock::new(BackendRegistry::new())),
        }
    }
}
