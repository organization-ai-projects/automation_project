use crate::app::action::Action;
use crate::app::controller::Controller;
use crate::diagnostics::error::UiError;

pub fn run_headless(controller: &mut Controller) -> Result<(), UiError> {
    controller.dispatch(Action::LoadScenario("default".to_string()))?;
    controller.dispatch(Action::NewRun { seed: 42 })?;
    controller.dispatch(Action::EncounterStep)?;
    controller.dispatch(Action::StartBattle)?;
    controller.dispatch(Action::BattleStep)?;
    controller.dispatch(Action::BattleStep)?;
    controller.dispatch(Action::BattleStep)?;
    controller.dispatch(Action::EndBattle)?;
    controller.dispatch(Action::GetSnapshot)?;
    controller.dispatch(Action::SaveReplay)?;
    if let Some(replay) = controller.state.replay_data.clone() {
        controller.dispatch(Action::LoadReplay(replay))?;
        controller.dispatch(Action::ReplayToEnd)?;
    }
    controller.dispatch(Action::GetReport)?;
    controller.dispatch(Action::Quit)?;
    Ok(())
}
