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
            Action::Feed => { state.last_event = Some("feed applied".to_string()); }
            Action::Rest => { state.last_event = Some("rest applied".to_string()); }
            Action::Play => { state.last_event = Some("play applied".to_string()); }
            Action::Discipline => { state.last_event = Some("discipline applied".to_string()); }
            Action::Medicine => { state.last_event = Some("medicine applied".to_string()); }
            Action::Train(k) => { state.last_event = Some(format!("training: {}", k)); }
            Action::StartBattle => { state.last_event = Some("battle started".to_string()); }
            Action::BattleStep => { state.last_event = Some("battle step".to_string()); }
            Action::GetSnapshot => { state.last_event = Some("snapshot requested".to_string()); }
            Action::GetReport => { state.last_event = Some("report requested".to_string()); }
            Action::SaveReplay(p) => { state.last_event = Some(format!("saved replay: {}", p)); }
            Action::LoadReplay(p) => { state.last_event = Some(format!("loaded replay: {}", p)); }
            Action::ReplayToEnd => { state.last_event = Some("replay to end".to_string()); }
        }
    }
}
