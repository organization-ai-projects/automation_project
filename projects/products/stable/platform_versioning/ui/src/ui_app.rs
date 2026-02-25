// projects/products/stable/platform_versioning/ui/src/ui_app.rs
use crate::admin_panel::AdminPanel;
use crate::auth_view::AuthView;
use crate::diff_display_entry::DiffDisplayEntry;
use crate::diff_entry_kind::DiffEntryKind;
use crate::diff_view::DiffView;
use crate::issue_summary::IssueSummary;
use crate::permission_entry::PermissionEntry;
use crate::ref_entry::RefEntry;
use crate::repo_detail_view::RepoDetailView;
use crate::repo_list_view::RepoListView;
use crate::repo_summary::RepoSummary;
use crate::role_view::RoleView;
use crate::tree_browser::TreeBrowser;
use crate::tree_browser_entry::TreeBrowserEntry;

/// Entry point for the platform-versioning UI.
///
/// The UI is a terminal-based application that communicates with the backend
/// HTTP API. A future version may provide a web frontend.
pub struct UiApp {
    auth: AuthView,
    repo_list: RepoListView,
    repo_detail: RepoDetailView,
    tree_browser: TreeBrowser,
    diff_view: DiffView,
    admin_panel: AdminPanel,
}

impl UiApp {
    fn new() -> Self {
        Self {
            auth: AuthView::default(),
            repo_list: RepoListView::default(),
            repo_detail: RepoDetailView::default(),
            tree_browser: TreeBrowser::default(),
            diff_view: DiffView::default(),
            admin_panel: AdminPanel::default(),
        }
    }

    fn load_initial_state(&mut self) {
        self.auth.login("session-token".to_string());

        self.repo_list.set_repos(vec![RepoSummary {
            id: "sample-repo".to_string(),
            name: "Sample Repo".to_string(),
            description: Some("Local demo state".to_string()),
        }]);

        self.repo_detail.open_repo("sample-repo".to_string());
        self.repo_detail.refs = vec![RefEntry {
            name: "heads/main".to_string(),
            commit_id: "0123456789abcdef".to_string(),
        }];
        self.repo_detail.recent_commits = vec!["Initial commit".to_string()];

        self.tree_browser.navigate(
            "src",
            vec![TreeBrowserEntry {
                name: "main.rs".to_string(),
                is_dir: false,
                object_id: "deadbeef".to_string(),
            }],
        );
        self.tree_browser.go_up(vec![]);

        self.diff_view.load(
            "aaaa1111".to_string(),
            "bbbb2222".to_string(),
            vec![DiffDisplayEntry {
                path: "README.md".to_string(),
                kind: DiffEntryKind::Modified,
                binary: false,
            }],
        );

        // Refresh auth state once to exercise both paths in runtime flow.
        self.auth.logout();
        self.auth.login("session-token".to_string());

        // Initialise admin governance panel for an admin session.
        self.admin_panel = AdminPanel::for_role(RoleView::Admin);
        self.admin_panel.permission_panel.load(
            vec![PermissionEntry {
                subject: "alice".to_string(),
                repo_id: "sample-repo".to_string(),
                permission: "read".to_string(),
            }],
            true,
        );
        self.admin_panel
            .permission_panel
            .add_entry(PermissionEntry {
                subject: "bob".to_string(),
                repo_id: "sample-repo".to_string(),
                permission: "write".to_string(),
            });
        self.admin_panel
            .permission_panel
            .remove_entry("bob", "sample-repo");

        self.admin_panel.issue_panel.load(
            vec![IssueSummary {
                id: "issue-1".to_string(),
                title: "Implement feature X".to_string(),
                repo_id: Some("sample-repo".to_string()),
                assignee_count: 1,
            }],
            true,
        );
        self.admin_panel.issue_panel.select("issue-1".to_string());
        self.admin_panel.issue_panel.deselect();

        self.admin_panel
            .slice_panel
            .load("issue-1".to_string(), vec!["src".to_string()], true);
        self.admin_panel.slice_panel.add_path("tests".to_string());
        self.admin_panel.slice_panel.remove_path("tests");
    }
}

/// Starts the UI application.
pub fn run() -> anyhow::Result<()> {
    let mut app = UiApp::new();
    app.load_initial_state();
    tracing::info!("Platform Versioning UI started (terminal mode)");
    tracing::info!("Connect to backend at http://127.0.0.1:8080");
    tracing::info!(
        "Loaded {} repos, {} refs, {} diff entries",
        app.repo_list.repos.len(),
        app.repo_detail.refs.len(),
        app.diff_view.entries.len()
    );
    tracing::info!(
        "Admin panel: role={}, {} permission entries, {} issues, {} slice paths",
        app.admin_panel.role.label(),
        app.admin_panel.permission_panel.entries.len(),
        app.admin_panel.issue_panel.issues.len(),
        app.admin_panel.slice_panel.allowed_paths.len(),
    );
    tracing::info!(
        "Admin visibility: permissions={}, slices={}",
        app.admin_panel.show_permission_panel(),
        app.admin_panel.show_slice_panel(),
    );
    Ok(())
}
