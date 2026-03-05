// projects/products/unstable/colony_manager/ui/src/controller.rs
use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::reducer::Reducer;
use crate::ui_error::UiError;
use std::process::Command;

pub struct Controller;

impl Controller {
    pub fn run_command(command: &str, args: &[String]) -> Result<(), UiError> {
        let mut state = AppState::default();
        match command {
            "run" => {
                Reducer::apply(&mut state, &Action::RunRequested);
                Self::forward_to_backend(command, args)?;
                Reducer::apply(&mut state, &Action::RunCompleted);
                Ok(())
            }
            "replay" => {
                Reducer::apply(&mut state, &Action::ReplayRequested);
                Self::forward_to_backend(command, args)?;
                Reducer::apply(&mut state, &Action::ReplayCompleted);
                Ok(())
            }
            _ => Err(UiError::Usage),
        }
    }

    fn forward_to_backend(command: &str, command_args: &[String]) -> Result<(), UiError> {
        let status = Command::new("colony_manager")
            .arg(command)
            .args(command_args)
            .status()
            .map_err(|e| UiError::Io(e.to_string()))?;
        if status.success() {
            return Ok(());
        }
        Err(UiError::Backend(status.code().unwrap_or(5)))
    }
}
