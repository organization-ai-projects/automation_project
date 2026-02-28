use crate::app::app_state::AppState;

pub fn apply_response(state: &mut AppState, response_json: &str) {
    state.last_response = Some(response_json.to_string());
}
