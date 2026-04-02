use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::reducer::Reducer;

#[test]
fn reduce_load_log_sets_active_panel() {
    let state = AppState::new();
    let action = Action::LoadLogFile {
        path: "test.json".to_string(),
    };
    let next = Reducer::reduce(&state, &action);
    assert!(next.active_panel.is_some());
    assert_eq!(next.active_panel.unwrap().as_str(), "log_viewer");
}

#[test]
fn reduce_is_deterministic() {
    let state = AppState::new();
    let action = Action::LoadLogFile {
        path: "test.json".to_string(),
    };
    let a = Reducer::reduce(&state, &action);
    let b = Reducer::reduce(&state, &action);
    assert_eq!(a, b);
}

#[test]
fn reduce_clear_removes_all_panels() {
    let state = AppState::new();
    let state = Reducer::reduce(
        &state,
        &Action::LoadLogFile {
            path: "a.json".to_string(),
        },
    );
    assert!(!state.panels.is_empty());
    let state = Reducer::reduce(&state, &Action::ClearPanelData);
    assert!(state.panels.is_empty());
    assert!(state.active_panel.is_none());
}

#[test]
fn reduce_select_nonexistent_panel_sets_error() {
    let state = AppState::new();
    let next = Reducer::reduce(
        &state,
        &Action::SelectPanel {
            plugin_id: "nonexistent".to_string(),
        },
    );
    assert!(next.error_message.is_some());
}

#[test]
fn reduce_select_existing_panel() {
    let state = AppState::new();
    let state = Reducer::reduce(
        &state,
        &Action::LoadLogFile {
            path: "a.json".to_string(),
        },
    );
    let state = Reducer::reduce(
        &state,
        &Action::LoadReportFile {
            path: "b.json".to_string(),
        },
    );
    let state = Reducer::reduce(
        &state,
        &Action::SelectPanel {
            plugin_id: "log_viewer".to_string(),
        },
    );
    assert_eq!(state.active_panel.unwrap().as_str(), "log_viewer");
    assert!(state.error_message.is_none());
}

#[test]
fn reduce_panels_sorted_deterministically() {
    let state = AppState::new();
    let state = Reducer::reduce(
        &state,
        &Action::LoadGraphFile {
            path: "g.json".to_string(),
        },
    );
    let state = Reducer::reduce(
        &state,
        &Action::LoadLogFile {
            path: "l.json".to_string(),
        },
    );
    let state = Reducer::reduce(
        &state,
        &Action::LoadReportFile {
            path: "r.json".to_string(),
        },
    );
    // Panels should be sorted by plugin_id
    let ids: Vec<&str> = state.panels.iter().map(|p| p.plugin_id.as_str()).collect();
    assert_eq!(ids, vec!["graph_viewer", "log_viewer", "report_viewer"]);
}
