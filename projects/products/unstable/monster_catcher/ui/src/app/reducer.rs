use crate::app::app_state::{AppState, Screen};

pub struct Reducer;

impl Reducer {
    pub fn set_scenario_loaded(state: &mut AppState) {
        state.scenario_loaded = true;
        state.last_error = None;
    }

    pub fn set_run_active(state: &mut AppState) {
        state.run_active = true;
        state.current_screen = Screen::Overworld;
        state.last_error = None;
    }

    pub fn set_encounter(state: &mut AppState, json: String) {
        state.encounter_json = Some(json);
        state.current_screen = Screen::Encounter;
        state.last_error = None;
    }

    pub fn set_battle(state: &mut AppState, json: String) {
        state.battle_json = Some(json);
        state.current_screen = Screen::Battle;
        state.last_error = None;
    }

    pub fn set_battle_ended(state: &mut AppState) {
        state.current_screen = Screen::Overworld;
        state.battle_json = None;
        state.last_error = None;
    }

    pub fn set_snapshot(state: &mut AppState, hash: String, json: String) {
        state.snapshot_hash = Some(hash);
        state.snapshot_json = Some(json);
        state.last_error = None;
    }

    pub fn set_report(state: &mut AppState, run_hash: String, json: String) {
        state.run_hash = Some(run_hash);
        state.report_json = Some(json);
        state.current_screen = Screen::Report;
        state.last_error = None;
    }

    pub fn set_replay(state: &mut AppState, replay: String) {
        state.replay_data = Some(replay);
        state.last_error = None;
    }

    pub fn set_error(state: &mut AppState, message: String) {
        state.last_error = Some(message);
    }
}
