// projects/products/unstable/hospital_tycoon/ui/src/app/reducer.rs
use crate::app::action::Action;
use crate::app::app_state::AppState;

pub struct Reducer;

impl Reducer {
    pub fn apply(state: &mut AppState, action: &Action) {
        match action {
            Action::Step(n) => {
                state.current_tick += n;
                state.running = true;
                state.last_event = Some(format!("stepped {} ticks", n));
            }
            Action::RunToEnd => {
                state.current_tick = state.ticks;
                state.running = false;
                state.last_event = Some("run completed".to_string());
            }
            Action::GetReport => {
                state.last_event = Some("report requested".to_string());
            }
            Action::GetSnapshot => {
                state.last_event = Some("snapshot requested".to_string());
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
            Action::Quit => {
                state.running = false;
                state.last_event = Some("quit".to_string());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reducer_determinism() {
        let mut s1 = AppState::new(42, 100);
        let mut s2 = AppState::new(42, 100);
        let actions = vec![
            Action::Step(10),
            Action::Step(20),
            Action::RunToEnd,
            Action::GetReport,
        ];
        for a in &actions {
            Reducer::apply(&mut s1, a);
            Reducer::apply(&mut s2, a);
        }
        assert_eq!(s1.current_tick, s2.current_tick);
        assert_eq!(s1.running, s2.running);
        assert_eq!(s1.last_event, s2.last_event);
    }
}
