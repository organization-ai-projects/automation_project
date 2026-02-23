// projects/products/unstable/platform_versioning/ui/src/main.rs
mod admin_panel;
mod auth_view;
mod diff_display_entry;
mod diff_entry_kind;
mod diff_view;
mod issue_panel;
mod issue_summary;
mod permission_entry;
mod permission_panel;
mod ref_entry;
mod repo_detail_view;
mod repo_list_view;
mod repo_summary;
mod role_view;
mod slice_panel;
mod tree_browser;
mod tree_browser_entry;
mod ui_app;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    ui_app::run()
}
