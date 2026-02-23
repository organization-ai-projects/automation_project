// projects/products/unstable/platform_versioning/ui/src/ui_app.rs
use crate::auth_view::AuthView;
use crate::repo_list_view::RepoListView;

/// Entry point for the platform-versioning UI.
///
/// The UI is a terminal-based application that communicates with the backend
/// HTTP API. A future version may provide a web frontend.
pub struct UiApp {
    auth: AuthView,
    repo_list: RepoListView,
}

impl UiApp {
    fn new() -> Self {
        Self {
            auth: AuthView::default(),
            repo_list: RepoListView::default(),
        }
    }
}

/// Starts the UI application.
pub fn run() -> anyhow::Result<()> {
    let _app = UiApp::new();
    tracing::info!("Platform Versioning UI started (terminal mode)");
    tracing::info!("Connect to backend at http://127.0.0.1:8080");
    Ok(())
}
