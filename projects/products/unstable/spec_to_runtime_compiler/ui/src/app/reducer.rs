// projects/products/unstable/spec_to_runtime_compiler/ui/src/app/reducer.rs
use super::action::Action;
use super::app_state::AppState;

pub fn apply(state: &mut AppState, action: Action) {
    match action {
        Action::LoadSpec { path } => {
            state.spec_path = path;
            state.spec_source = None;
            state.error = None;
        }
        Action::SetError(msg) => {
            state.error = Some(msg);
        }
        Action::SetReport(report) => {
            state.last_report = Some(report);
        }
        Action::Compile | Action::DryRun | Action::GetReport => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_spec_action_sets_path() {
        let mut state = AppState::default();
        apply(
            &mut state,
            Action::LoadSpec {
                path: "machine.spec".to_string(),
            },
        );
        assert_eq!(state.spec_path, "machine.spec");
    }

    #[test]
    fn set_error_action() {
        let mut state = AppState::default();
        apply(&mut state, Action::SetError("oops".to_string()));
        assert_eq!(state.error.as_deref(), Some("oops"));
    }
}
