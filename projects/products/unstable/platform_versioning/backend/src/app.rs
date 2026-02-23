use std::path::PathBuf;
use std::sync::Arc;

use crate::auth::TokenVerifier;
use crate::errors::PvError;
use crate::http::Server;
use crate::repos::RepoStore;

/// Configuration for the platform-versioning backend server.
pub struct AppConfig {
    /// TCP address to bind (e.g. `"0.0.0.0:8080"`).
    pub bind_addr: String,
    /// Root directory for all persistent data.
    pub data_dir: PathBuf,
    /// Signing secret for auth tokens (min 32 bytes).
    pub token_secret: Vec<u8>,
    /// Optional one-time bootstrap secret for initial token issuance.
    pub bootstrap_secret: Option<String>,
}

impl AppConfig {
    /// Reads configuration from environment variables with sensible defaults.
    pub fn from_env() -> Result<Self, PvError> {
        let bind_addr =
            std::env::var("PV_BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
        let data_dir = std::env::var("PV_DATA_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("./pv_data"));
        let token_secret = std::env::var("PV_TOKEN_SECRET")
            .unwrap_or_else(|_| "CHANGE_ME_CHANGE_ME_CHANGE_ME_32X".to_string())
            .into_bytes();
        let bootstrap_secret = std::env::var("PV_BOOTSTRAP_SECRET")
            .ok()
            .filter(|s| !s.trim().is_empty());
        Ok(Self {
            bind_addr,
            data_dir,
            token_secret,
            bootstrap_secret,
        })
    }
}

/// Starts the platform-versioning backend server.
pub async fn run(config: AppConfig) -> Result<(), PvError> {
    let repo_store = Arc::new(RepoStore::open(&config.data_dir)?);
    let token_verifier = Arc::new(TokenVerifier::new(config.token_secret)?);

    let server = Server::bind(
        &config.bind_addr,
        repo_store,
        token_verifier,
        config.bootstrap_secret,
    )
    .await?;

    let addr = server.local_addr()?;
    tracing::info!("Platform Versioning backend listening on {addr}");

    server.serve().await
}
