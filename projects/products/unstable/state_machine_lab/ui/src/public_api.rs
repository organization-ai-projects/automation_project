use crate::app::action::Action;
use crate::app::controller::Controller;
use crate::diagnostics::error::UiError;

pub fn run_headless(controller: &mut Controller) -> Result<(), UiError> {
    let machine_source = "\
machine toggle
state off
state on
event flip
initial off
transition off flip -> on
transition on flip -> off
";
    controller.dispatch(Action::LoadMachine(machine_source.to_string()))?;
    controller.dispatch(Action::Validate)?;
    controller.dispatch(Action::Run(vec![
        "flip".to_string(),
        "flip".to_string(),
        "flip".to_string(),
    ]))?;
    controller.dispatch(Action::GetTranscript)?;
    controller.dispatch(Action::TestExhaustive)?;
    controller.dispatch(Action::TestFuzz {
        seed: 42,
        steps: 20,
    })?;
    controller.dispatch(Action::Quit)?;
    Ok(())
}
