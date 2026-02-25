// projects/products/stable/platform_ide/ui/src/main.rs
mod auth_view;
mod change_submit_view;
mod diff_view;
mod editor_view;
mod issue_list_view;
mod offline_controls;
mod slice_explorer;
mod ui_app;
mod verification_view;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    ui_app::run()
}
