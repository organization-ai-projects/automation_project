use crate::models::editor_state::EditorState;

#[test]
fn editor_state_validates_non_empty() {
    let state = EditorState::new("fn main() {}".into());
    assert!(state.validate().is_ok());
}

#[test]
fn editor_state_rejects_empty() {
    let state = EditorState::new("".into());
    assert!(state.validate().is_err());
}

#[test]
fn editor_state_set_transpiled() {
    let mut state = EditorState::new("fn main() {}".into());
    state.set_transpiled("fn main() {}".into());
    assert!(state.transpiled_output.is_some());
    assert!(state.error_message.is_none());
}

#[test]
fn editor_state_set_error() {
    let mut state = EditorState::new("fn main() {}".into());
    state.set_error("parse error".into());
    assert!(state.error_message.is_some());
    assert!(state.transpiled_output.is_none());
}
