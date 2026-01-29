// projects/products/core/engine/src/main.rs
use std::{path::PathBuf, sync::Arc};

use anyhow::Context;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

use accounts_backend::AccountManager;
use engine::{BackendRegistry, EngineState, Registry, engine_config::EngineConfig, routes};
use security::TokenService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = EngineConfig::from_env().context("Failed to load engine configuration")?;

    // Secret JWT
    let token_service =
        TokenService::new_hs256(&config.jwt_secret).context("ENGINE_JWT_SECRET invalid")?;

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

    // Accounts storage
    let data_dir = std::env::var("ACCOUNTS_DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| AccountManager::default_data_dir());
    let account_manager = AccountManager::load(data_dir)
        .await
        .context("Failed to load accounts store")?;

    // Bootstrap claim (appliance setup)
    match engine::ensure_owner_claim() {
        Ok(state) if state.setup_mode => {
            if let Some(expires_at) = state.expires_at {
                info!(
                    claim_path = %state.claim_path.display(),
                    expires_at_unix = expires_at,
                    "Setup mode enabled: owner claim available"
                );
            } else {
                info!(
                    claim_path = %state.claim_path.display(),
                    "Setup mode enabled: owner claim available"
                );
            }
        }
        Ok(_) => {
            info!("Setup mode disabled: owner claim already consumed");
        }
        Err(e) => {
            error!(error = %e, "Failed to initialize owner claim");
        }
    }

    // State
    let state = EngineState {
        registry: Arc::new(RwLock::new(registry)),
        token_service: Arc::new(token_service),
        backend_registry: Arc::new(RwLock::new(BackendRegistry::new())),
        account_manager: Arc::new(account_manager),
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
    Ok(())
}
