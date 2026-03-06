use crate::app::app_state::AppState;
use crate::app::controller::Controller;
use crate::app::screen::Screen;
use crate::diagnostics::ui_error::UiError;
use crate::screens::replay_screen::ReplayScreen;
use crate::screens::run_screen::RunScreen;
use crate::screens::snapshot_screen::SnapshotScreen;
use crate::screens::validate_screen::ValidateScreen;
use crate::transport::backend_process::BackendProcess;
use crate::widgets::status_widget::StatusWidget;
use std::path::Path;

fn execute_with_screen(
    screen: Screen,
    build_request: impl FnOnce() -> crate::transport::request::Request,
) -> Result<i32, UiError> {
    let process = BackendProcess::new();
    let client = crate::transport::client::Client::new(process);
    let state = AppState::new(client.backend_bin().to_string());
    let mut controller = Controller::new(state, client);

    let request = build_request();
    let code = controller.execute(request)?;

    // Keep deterministic screen routing explicit even without TUI rendering yet.
    match screen {
        Screen::Run | Screen::Replay | Screen::Snapshot | Screen::Validate => {
            let _status_line = StatusWidget::render(controller.state());
        }
    }

    Ok(code)
}

pub fn run_cli(
    ticks: u64,
    seed: u64,
    scenario: &Path,
    out: &Path,
    replay_out: Option<&Path>,
) -> Result<i32, UiError> {
    execute_with_screen(Screen::Run, || {
        RunScreen::request(
            ticks,
            seed,
            scenario.to_path_buf(),
            out.to_path_buf(),
            replay_out.map(Path::to_path_buf),
        )
    })
}

pub fn replay_cli(replay: &Path, out: &Path) -> Result<i32, UiError> {
    execute_with_screen(Screen::Replay, || {
        ReplayScreen::request(replay.to_path_buf(), out.to_path_buf())
    })
}

pub fn snapshot_cli(replay: &Path, at_tick: u64, out: &Path) -> Result<i32, UiError> {
    execute_with_screen(Screen::Snapshot, || {
        SnapshotScreen::request(replay.to_path_buf(), at_tick, out.to_path_buf())
    })
}

pub fn validate_cli(scenario: &Path) -> Result<i32, UiError> {
    execute_with_screen(Screen::Validate, || {
        ValidateScreen::request(scenario.to_path_buf())
    })
}
