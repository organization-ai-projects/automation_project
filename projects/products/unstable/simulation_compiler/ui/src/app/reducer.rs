// projects/products/unstable/simulation_compiler/ui/src/app/reducer.rs
use super::action::Action;
use super::app_state::AppState;

pub fn apply(state: &mut AppState, action: Action) {
    match action {
        Action::LoadDsl { path } => {
            state.dsl_path = path;
            state.dsl_source = None;
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
    fn load_dsl_action_sets_path() {
        let mut state = AppState::default();
        apply(&mut state, Action::LoadDsl { path: "world.dsl".to_string() });
        assert_eq!(state.dsl_path, "world.dsl");
    }

    #[test]
    fn set_error_action() {
        let mut state = AppState::default();
        apply(&mut state, Action::SetError("oops".to_string()));
        assert_eq!(state.error.as_deref(), Some("oops"));
    }
}
