use std::sync::Arc;

use axum::Router;
use tokio::net::TcpListener;

use crate::auth::AuthorizationService;
use crate::errors::PvError;
use crate::issues::IssueStore;
use crate::repos::RepoStore;

/// HTTP server for the platform-versioning backend.
///
/// All routes are mounted under the versioned prefix `/v1/`.
pub struct Server {
    listener: TcpListener,
    router: Router,
}

impl Server {
    /// Binds to `addr` and creates the server.
    pub async fn bind(
        addr: &str,
        repo_store: Arc<RepoStore>,
        auth_svc: Arc<AuthorizationService>,
        issue_store: Arc<IssueStore>,
        bootstrap_secret: Option<String>,
    ) -> Result<Self, PvError> {
        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| PvError::Internal(format!("bind {addr}: {e}")))?;

        let router =
            crate::routes::build_router(repo_store, auth_svc, issue_store, bootstrap_secret);

        Ok(Self { listener, router })
    }

    /// Returns the local address the server is bound to.
    pub fn local_addr(&self) -> Result<std::net::SocketAddr, PvError> {
        self.listener
            .local_addr()
            .map_err(|e| PvError::Internal(format!("local_addr: {e}")))
    }

    /// Starts serving requests. This future runs until the process exits.
    pub async fn serve(self) -> Result<(), PvError> {
        axum::serve(self.listener, self.router)
            .await
            .map_err(|e| PvError::Internal(format!("serve: {e}")))
    }
}
