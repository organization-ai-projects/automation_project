mod app;
mod diagnostics;
mod public_api;
mod screens;
mod transport;
mod widgets;

use crate::app::action::Action;
use crate::app::app_state::{AppState, Screen};
use crate::app::controller::Controller;
use crate::diagnostics::error::UiError;
use crate::screens::bundle_screen::BundleScreen;
use crate::screens::graph_screen::GraphScreen;
use crate::screens::input_screen::InputScreen;
use crate::screens::render_screen::RenderScreen;
use crate::transport::backend_process::BackendProcess;
use crate::transport::ipc_client::IpcClient;

fn main() -> Result<(), UiError> {
    tracing_subscriber::fmt::init();
    tracing::info!("artifact-factory-ui starting");

    let args: Vec<String> = std::env::args().collect();
    let backend_bin = args
        .iter()
        .position(|a| a == "--backend")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| "artifact-factory-backend".to_string());

    let input_paths: Vec<String> = args
        .iter()
        .position(|a| a == "--inputs")
        .map(|i| {
            args[i + 1..]
                .iter()
                .take_while(|a| !a.starts_with("--"))
                .cloned()
                .collect()
        })
        .unwrap_or_default();

    run_headless(&backend_bin, input_paths)
}

/// Drive the backend through the full pipeline and print results.
fn run_headless(backend_bin: &str, input_paths: Vec<String>) -> Result<(), UiError> {
    tracing::info!(backend = %backend_bin, "spawning backend process");

    let process = BackendProcess::spawn(backend_bin)?;
    let ipc = IpcClient::new(process);
    let mut controller = Controller::new(ipc);

    // Step 1: Load inputs
    if !input_paths.is_empty() {
        controller.dispatch(Action::LoadInputs(input_paths))?;
    }
    render_screen(&controller.state);

    // Step 2: Analyze
    controller.dispatch(Action::Analyze)?;
    render_screen(&controller.state);

    // Step 3: Render docs
    controller.dispatch(Action::RenderDocs)?;
    render_screen(&controller.state);

    // Step 4: Build bundle
    controller.dispatch(Action::BuildBundle)?;

    // Step 5: Get bundle info
    controller.dispatch(Action::GetBundle)?;
    render_screen(&controller.state);

    controller.dispatch(Action::Quit)?;
    Ok(())
}

fn render_screen(state: &AppState) {
    match state.current_screen {
        Screen::Input => InputScreen::render(state),
        Screen::Graph => GraphScreen::render(state),
        Screen::Render => RenderScreen::render(state),
        Screen::Bundle => BundleScreen::render(state),
    }
}
