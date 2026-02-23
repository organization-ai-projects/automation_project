// projects/products/unstable/platform_ide/ui/src/ui_app.rs
use crate::auth_view::AuthView;
use crate::change_submit_view::ChangeSubmitView;
use crate::diff_view::{DiffLineEntry, DiffLineKind, DiffView};
use crate::editor_view::EditorView;
use crate::issue_list_view::{IssueEntry, IssueListView};
use crate::offline_controls::OfflineControls;
use crate::slice_explorer::SliceExplorer;
use crate::verification_view::{FindingEntry, VerificationView};

/// The platform IDE UI application.
///
/// Wires together all view components. In a future version this will drive
/// a terminal TUI or web frontend.
pub struct UiApp {
    auth: AuthView,
    issue_list: IssueListView,
    slice_explorer: SliceExplorer,
    editor: EditorView,
    diff_view: DiffView,
    change_submit: ChangeSubmitView,
    verification: VerificationView,
    offline_controls: OfflineControls,
}

impl UiApp {
    fn new() -> Self {
        Self {
            auth: AuthView::default(),
            issue_list: IssueListView::default(),
            slice_explorer: SliceExplorer::default(),
            editor: EditorView::default(),
            diff_view: DiffView::default(),
            change_submit: ChangeSubmitView::default(),
            verification: VerificationView::default(),
            offline_controls: OfflineControls::default(),
        }
    }

    fn load_initial_state(&mut self) {
        // Auth
        self.auth.login("session-token".to_string(), "alice".to_string());

        // Issue list (only visible issues)
        self.issue_list.set_issues(vec![IssueEntry {
            id: "issue-42".to_string(),
            name: "Feature: add login".to_string(),
            description: Some("Local demo state".to_string()),
        }]);

        // Slice explorer (only allowed paths)
        self.slice_explorer.load(
            "issue-42".to_string(),
            vec!["src/main.rs".to_string(), "README.md".to_string()],
        );

        // Editor
        self.editor.open(
            "src/main.rs".to_string(),
            "fn main() {}".to_string(),
        );
        self.editor.edit("fn main() { println!(\"hello\"); }".to_string());
        self.slice_explorer.mark_dirty("src/main.rs");

        // Diff view
        self.diff_view.load(
            "src/main.rs".to_string(),
            vec![
                DiffLineEntry {
                    kind: DiffLineKind::Removed,
                    content: "fn main() {}".to_string(),
                },
                DiffLineEntry {
                    kind: DiffLineKind::Added,
                    content: "fn main() { println!(\"hello\"); }".to_string(),
                },
            ],
        );

        // Change submission
        self.change_submit.stage(
            vec!["src/main.rs".to_string()],
            "feat: add hello".to_string(),
        );
        self.change_submit.on_submitted("deadbeef1234".to_string());

        // Verification results (only safe summaries, no forbidden paths)
        self.verification.load(
            true,
            vec![FindingEntry {
                severity: "info".to_string(),
                summary: "All checks passed.".to_string(),
                path: None,
                line: None,
            }],
        );

        // Offline controls — hidden unless platform approves
        self.offline_controls.hide();
    }
}

/// Starts the IDE UI.
pub fn run() -> anyhow::Result<()> {
    let mut app = UiApp::new();
    app.load_initial_state();
    tracing::info!("Platform IDE UI started (terminal mode)");
    tracing::info!("Connect backend at http://127.0.0.1:8080");
    tracing::info!(
        "Loaded {} issues, {} slice entries, diff has_changes={}",
        app.issue_list.issues.len(),
        app.slice_explorer.entries.len(),
        app.diff_view.has_changes(),
    );
    Ok(())
}
