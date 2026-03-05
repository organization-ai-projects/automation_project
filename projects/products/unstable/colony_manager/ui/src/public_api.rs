// projects/products/unstable/colony_manager/ui/src/public_api.rs
use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::reducer::Reducer;
use crate::ui_error::UiError;
use std::process::Command;

pub struct UiApi;

impl UiApi {
    pub fn run_from_args(args: Vec<String>) -> Result<(), UiError> {
        if args.len() < 2 {
            return Err(UiError::Usage);
        }

        let mut state = AppState::default();
        match args[1].as_str() {
            "run" => {
                Reducer::apply(&mut state, &Action::RunRequested);
                Self::forward_to_backend(&args[1..])?;
                Reducer::apply(&mut state, &Action::RunCompleted);
                Ok(())
            }
            "replay" => {
                Reducer::apply(&mut state, &Action::ReplayRequested);
                Self::forward_to_backend(&args[1..])?;
                Reducer::apply(&mut state, &Action::ReplayCompleted);
                Ok(())
            }
            _ => Err(UiError::Usage),
        }
    }

    fn forward_to_backend(command_args: &[String]) -> Result<(), UiError> {
        let status = Command::new("colony_manager")
            .args(command_args)
            .status()
            .map_err(|e| UiError::Io(e.to_string()))?;
        if status.success() {
            return Ok(());
        }
        Err(UiError::Backend(status.code().unwrap_or(5)))
    }
}
