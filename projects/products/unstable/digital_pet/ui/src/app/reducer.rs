// projects/products/unstable/digital_pet/ui/src/app/reducer.rs
use crate::app::action::Action;
use crate::app::app_state::AppState;

pub struct Reducer;

impl Reducer {
    pub fn apply(state: &mut AppState, action: &Action) {
        match action {
            Action::Step(n) => {
                state.current_tick += n;
                state.running = true;
            }
            Action::Quit => {
                state.running = false;
            }
            Action::Feed => {
                state.last_event = Some("feed applied".to_string());
            }
            Action::Rest => {
                state.last_event = Some("rest applied".to_string());
            }
            Action::Play => {
                state.last_event = Some("play applied".to_string());
            }
            Action::Discipline => {
                state.last_event = Some("discipline applied".to_string());
            }
            Action::Medicine => {
                state.last_event = Some("medicine applied".to_string());
            }
            Action::Train(k) => {
                state.last_event = Some(format!("training: {}", k));
            }
            Action::StartBattle => {
                state.last_event = Some("battle started".to_string());
            }
            Action::BattleStep => {
                state.last_event = Some("battle step".to_string());
            }
            Action::GetSnapshot => {
                state.last_event = Some("snapshot requested".to_string());
            }
            Action::GetReport => {
                state.last_event = Some("report requested".to_string());
            }
            Action::SaveReplay(p) => {
                state.last_event = Some(format!("saved replay: {}", p));
            }
            Action::LoadReplay(p) => {
                state.last_event = Some(format!("loaded replay: {}", p));
            }
            Action::ReplayToEnd => {
                state.last_event = Some("replay to end".to_string());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Reducer;
    use crate::app::action::Action;
    use crate::app::app_state::AppState;
    use crate::fixtures::fixture_loader::FixtureLoader;
    use crate::transport::run_report_dto::RunReportDto;
    use std::path::PathBuf;

    #[test]
    fn reducer_is_deterministic_for_same_action_stream() {
        let actions = vec![
            Action::Step(10),
            Action::Feed,
            Action::Train("strength".to_string()),
            Action::StartBattle,
            Action::BattleStep,
            Action::GetReport,
            Action::Quit,
        ];

        let mut a = AppState::new(42, 100);
        let mut b = AppState::new(42, 100);
        for action in &actions {
            Reducer::apply(&mut a, action);
            Reducer::apply(&mut b, action);
        }
        assert_eq!(a.current_tick, b.current_tick);
        assert_eq!(a.running, b.running);
        assert_eq!(a.last_event, b.last_event);
    }

    #[test]
    fn loading_golden_report_yields_deterministic_screen_model() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("fixtures")
            .join("golden_report.json");
        let parsed = FixtureLoader::load_json(&path);
        assert!(parsed.is_ok());
        let json = if let Ok(v) = parsed { v } else { return };

        let serialized = common_json::to_string(&json);
        assert!(serialized.is_ok());
        let report = if let Ok(s) = serialized {
            common_json::from_str::<RunReportDto>(&s)
        } else {
            return;
        };
        assert!(report.is_ok());
        let report = if let Ok(r) = report { r } else { return };

        let a = AppState::from_report(&report);
        let b = AppState::from_report(&report);
        assert_eq!(a.species, b.species);
        assert_eq!(a.evolution_stage, b.evolution_stage);
        assert_eq!(a.last_event, b.last_event);
    }
}
