use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::controller::Controller;
use crate::app::screen::Screen;
use crate::diagnostics::ui_error::UiError;
use crate::screens::bundle_screen::BundleScreen;
use crate::screens::graph_screen::GraphScreen;
use crate::screens::input_screen::InputScreen;
use crate::screens::render_screen::RenderScreen;
use crate::transport::backend_process::BackendProcess;
use crate::transport::ipc_client::IpcClient;

pub fn run_headless(backend_bin: &str, input_paths: Vec<String>) -> Result<(), UiError> {
    tracing::info!(backend = %backend_bin, "spawning backend process");

    let process = BackendProcess::spawn(backend_bin)?;
    let ipc = IpcClient::new(process);
    let mut controller = Controller::new(ipc);

    if !input_paths.is_empty() {
        controller.dispatch(Action::LoadInputs(input_paths))?;
    }
    render_screen(&controller.state);

    controller.dispatch(Action::Analyze)?;
    render_screen(&controller.state);

    controller.dispatch(Action::RenderDocs)?;
    render_screen(&controller.state);

    controller.dispatch(Action::BuildBundle)?;
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
