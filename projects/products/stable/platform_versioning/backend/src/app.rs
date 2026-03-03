use std::sync::Arc;

use crate::app_config::AppConfig;
use crate::auth::{AuditLog, AuthorizationService, TokenVerifier};
use crate::errors::PvError;
use crate::http::Server;
use crate::issues::IssueStore;
use crate::repos::RepoStore;

/// Application entry object for the backend runtime.
pub struct App {
    config: AppConfig,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    /// Starts the platform-versioning backend server.
    pub async fn run(self) -> Result<(), PvError> {
        let repo_store = Arc::new(RepoStore::open(&self.config.data_dir)?);
        let token_verifier = Arc::new(TokenVerifier::new(self.config.token_secret)?);
        let audit_log = Arc::new(AuditLog::new());
        let auth_svc = Arc::new(AuthorizationService::new(token_verifier, audit_log));
        let issue_store = Arc::new(IssueStore::new());

        let server = Server::bind(
            &self.config.bind_addr,
            repo_store,
            auth_svc,
            issue_store,
            self.config.bootstrap_secret,
        )
        .await?;

        let addr = server.local_addr()?;
        tracing::info!("Platform Versioning backend listening on {addr}");

        server.serve().await
    }
}
