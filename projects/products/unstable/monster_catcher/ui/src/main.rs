mod app;
mod diagnostics;
mod fixtures;
mod public_api;
mod screens;
mod transport;
mod widgets;

use crate::app::app_state::AppState;
use crate::app::controller::Controller;
use crate::diagnostics::error::UiError;
use crate::screens::battle_screen::BattleScreen;
use crate::screens::encounter_screen::EncounterScreen;
use crate::screens::overworld_screen::OverworldScreen;
use crate::screens::party_screen::PartyScreen;
use crate::screens::report_screen::ReportScreen;
use crate::transport::backend_process::BackendProcess;
use crate::transport::ipc_client::IpcClient;

fn main() -> Result<(), UiError> {
    let args: Vec<String> = std::env::args().collect();
    let backend_bin = args
        .iter()
        .position(|a| a == "--backend")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| "monster_catcher_backend".to_string());

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
        crate::app::app_state::Screen::Overworld => OverworldScreen::render(state),
        crate::app::app_state::Screen::Encounter => EncounterScreen::render(state),
        crate::app::app_state::Screen::Battle => BattleScreen::render(state),
        crate::app::app_state::Screen::Party => PartyScreen::render(state),
        crate::app::app_state::Screen::Report => ReportScreen::render(state),
    }
}
