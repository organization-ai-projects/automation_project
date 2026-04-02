mod app;
mod diagnostics;
mod public_api;
mod screens;
mod transport;
mod widgets;

use crate::app::app_state::AppState;
use crate::app::controller::Controller;
use crate::app::screen::Screen;
use crate::diagnostics::error::UiError;
use crate::screens::editor_screen::EditorScreen;
use crate::screens::run_screen::RunScreen;
use crate::screens::test_screen::TestScreen;
use crate::screens::transcript_screen::TranscriptScreen;
use crate::transport::backend_process::BackendProcess;
use crate::transport::ipc_client::IpcClient;

fn main() -> Result<(), UiError> {
    let args: Vec<String> = std::env::args().collect();
    let backend_bin = args
        .iter()
        .position(|a| a == "--backend")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| "state_machine_lab_backend".to_string());

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
        Screen::Editor => EditorScreen::render(state),
        Screen::Run => RunScreen::render(state),
        Screen::Test => TestScreen::render(state),
        Screen::Transcript => TranscriptScreen::render(state),
    }
}
