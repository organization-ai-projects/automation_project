mod app;
mod diagnostics;
mod public_api;
mod screens;
mod transport;
mod widgets;

use crate::app::app_state::AppState;
use crate::app::controller::Controller;
use crate::app::screen::Screen;
use crate::diagnostics::ui_error::UiError;
use crate::screens::dsl_screen::DslScreen;
use crate::screens::inspect_screen::InspectScreen;
use crate::screens::replay_screen::ReplayScreen;
use crate::screens::report_screen::ReportScreen;
use crate::screens::run_screen::RunScreen;
use crate::transport::backend_process::BackendProcess;
use crate::transport::ipc_client::IpcClient;

fn main() -> Result<(), UiError> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    let backend_bin = args
        .iter()
        .position(|arg| arg == "--backend")
        .and_then(|index| args.get(index + 1))
        .cloned()
        .unwrap_or_else(|| "universal_model_engine_backend".to_string());

    let process = BackendProcess::spawn(&backend_bin)?;
    let ipc = IpcClient::new(process);
    let mut controller = Controller::new(ipc);

    render_screen(&controller.state);
    public_api::run_headless(&mut controller)?;
    render_screen(&controller.state);
    Ok(())
}

fn render_screen(state: &AppState) {
    match state.current_screen {
        Screen::Dsl => DslScreen::render(state),
        Screen::Run => RunScreen::render(state),
        Screen::Inspect => InspectScreen::render(state),
        Screen::Replay => ReplayScreen::render(state),
        Screen::Report => ReportScreen::render(state),
    }
}
