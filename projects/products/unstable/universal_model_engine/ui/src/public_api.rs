use crate::app::action::Action;
use crate::app::controller::Controller;
use crate::diagnostics::ui_error::UiError;

pub fn run_headless(controller: &mut Controller) -> Result<(), UiError> {
    controller.dispatch(Action::LoadModel(
        "var x int 0\nvar y int 5\nconstraint c1 y min -100".to_string(),
    ))?;
    controller.dispatch(Action::ValidateModel)?;
    controller.dispatch(Action::NewRun { seed: 7 })?;

    controller.dispatch(Action::Step)?;
    controller.dispatch(Action::RunToEnd)?;

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
