// projects/products/core/engine/src/main.rs
mod bootstrap;
mod const_values;
mod cors_config;
mod engine_config;
mod engine_state;
mod registry;
mod requires;
mod routes;
mod runtime;
mod ws;

pub(crate) use bootstrap::{
    BootstrapError, consume_claim, ensure_owner_claim, setup_complete, validate_claim,
};
pub(crate) use const_values::*;
pub(crate) use cors_config::CorsConfig;
pub(crate) use engine_state::EngineState;
pub(crate) use registry::Registry;
pub(crate) use requires::{require_permission, require_project_exists};
pub(crate) use runtime::*;

use anyhow::Context;
use security::TokenService;
use tracing::{error, info, warn};

use crate::engine_config::EngineConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = EngineConfig::from_env().context("Failed to load engine configuration")?;
    if config.allow_insecure_secret {
        warn!("ENGINE_ALLOW_INSECURE_SECRET=1 is set; default secret may be used (INSECURE)");
    }

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

    if let Ok(cache_path) = std::env::var("ENGINE_REGISTRY_CACHE_PATH")
        && let Err(e) = registry.save_cache(&cache_path)
    {
        warn!(
            cache_path = %cache_path,
            error = %e,
            "Failed to write registry cache"
        );
    }

    // Bootstrap claim (appliance setup)
    match crate::ensure_owner_claim() {
        Ok(state) if state.setup_mode => {
            if let Some(expires_at) = state.expires_at {
                info!(
                    claim_path = %state.claim_path.display(),
                    used_marker_path = %state.used_marker_path.display(),
                    expires_at_unix = expires_at,
                    "Setup mode enabled: owner claim available"
                );
            } else {
                info!(
                    claim_path = %state.claim_path.display(),
                    used_marker_path = %state.used_marker_path.display(),
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
    let state = EngineState::new(registry, token_service);

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
