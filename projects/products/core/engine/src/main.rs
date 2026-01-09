// projects/products/core/engine/src/main.rs
use std::sync::Arc;

use tokio::sync::RwLock;
use tracing::{info, warn};

use engine::{BackendRegistry, EngineState, Registry, config::EngineConfig, routes};
use security::TokenService;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = EngineConfig::from_env().expect("Failed to load engine configuration");

    // Secret JWT
    let token_service =
        TokenService::new_hs256(&config.jwt_secret).expect("ENGINE_JWT_SECRET invalid");

    // Registry auto
    let registry = Registry::load(&config.projects_dir).unwrap_or_else(|e| {
        warn!(
            projects_dir = %config.projects_dir.display(),
            error = %e,
            "Failed to scan projects dir. Starting with empty registry."
        );
        Registry::default()
    });

    info!(
        projects_count = registry.projects.len(),
        projects_dir = %config.projects_dir.display(),
        "Registry loaded"
    );

    // State
    let state = EngineState {
        registry: Arc::new(RwLock::new(registry)),
        token_service: Arc::new(token_service),
        backend_registry: Arc::new(RwLock::new(BackendRegistry::new())),
    };

    // Routes
    let routes = routes::build_routes(state, config.cors);

    info!(host = %config.host, port = config.port, "Engine listening");
    info!(
        "WebSocket endpoint: ws://{}:{}/ws?token=<JWT>",
        config.host, config.port
    );
    info!(
        "Health check: http://{}:{}/health",
        config.host, config.port
    );
    info!(
        "Login endpoint: POST http://{}:{}/auth/login",
        config.host, config.port
    );
    info!(
        "Projects list: GET http://{}:{}/projects",
        config.host, config.port
    );

    warp::serve(routes).run((config.host, config.port)).await;
}
