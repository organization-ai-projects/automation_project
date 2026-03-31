use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::reducer::Reducer;
use crate::persistence::snapshot_codec::SnapshotCodec;
use crate::plugins::builtin_plugins::BuiltinPlugins;
use crate::ui_model::panel_registry::PanelRegistry;

#[test]
fn full_snapshot_roundtrip() {
    // Build initial state with actions
    let state = AppState::new();
    let state = Reducer::reduce(
        &state,
        &Action::LoadLogFile {
            path: "fixture/logs.json".to_string(),
        },
    );
    let state = Reducer::reduce(
        &state,
        &Action::LoadReportFile {
            path: "fixture/report.json".to_string(),
        },
    );
    let state = Reducer::reduce(
        &state,
        &Action::LoadGraphFile {
            path: "fixture/graph.json".to_string(),
        },
    );

    // Export -> Import -> Verify identical
    let exported = SnapshotCodec::export(&state).unwrap();
    let imported = SnapshotCodec::import(&exported).unwrap();
    assert_eq!(state, imported);
}

#[test]
fn repeated_snapshot_stability() {
    let state = AppState::new();
    let state = Reducer::reduce(
        &state,
        &Action::LoadLogFile {
            path: "a.json".to_string(),
        },
    );

    let snap1 = SnapshotCodec::export(&state).unwrap();
    let state2 = SnapshotCodec::import(&snap1).unwrap();
    let snap2 = SnapshotCodec::export(&state2).unwrap();
    let state3 = SnapshotCodec::import(&snap2).unwrap();
    let snap3 = SnapshotCodec::export(&state3).unwrap();

    // Compare deserialized snapshots since outer JSON field order may vary
    let s1: crate::persistence::ui_snapshot::UiSnapshot = common_json::from_str(&snap1).unwrap();
    let s2: crate::persistence::ui_snapshot::UiSnapshot = common_json::from_str(&snap2).unwrap();
    let s3: crate::persistence::ui_snapshot::UiSnapshot = common_json::from_str(&snap3).unwrap();
    assert_eq!(s1, s2);
    assert_eq!(s2, s3);
    assert_eq!(state, state2);
    assert_eq!(state2, state3);
}

#[test]
fn builtin_plugin_loading_flow() {
    let mut registry = PanelRegistry::new();
    for plugin in BuiltinPlugins::all() {
        registry.register(plugin);
    }
    assert_eq!(registry.len(), 3);

    // Verify all builtin plugins are accessible
    let ids: Vec<&str> = registry.ids().iter().map(|id| id.as_str()).collect();
    assert_eq!(ids, vec!["graph_viewer", "log_viewer", "report_viewer"]);
}

#[test]
fn canonical_output_stability() {
    let state1 = AppState::new();
    let state1 = Reducer::reduce(
        &state1,
        &Action::LoadGraphFile {
            path: "g.json".to_string(),
        },
    );
    let state1 = Reducer::reduce(
        &state1,
        &Action::LoadLogFile {
            path: "l.json".to_string(),
        },
    );

    // Build same state in different order
    let state2 = AppState::new();
    let state2 = Reducer::reduce(
        &state2,
        &Action::LoadLogFile {
            path: "l.json".to_string(),
        },
    );
    let state2 = Reducer::reduce(
        &state2,
        &Action::LoadGraphFile {
            path: "g.json".to_string(),
        },
    );

    // Both should produce same panels in same order
    let ids1: Vec<&str> = state1.panels.iter().map(|p| p.plugin_id.as_str()).collect();
    let ids2: Vec<&str> = state2.panels.iter().map(|p| p.plugin_id.as_str()).collect();
    assert_eq!(ids1, ids2);

    // Apply the same final action to both so active_panel matches
    let state1 = Reducer::reduce(
        &state1,
        &Action::SelectPanel {
            plugin_id: "graph_viewer".to_string(),
        },
    );
    let state2 = Reducer::reduce(
        &state2,
        &Action::SelectPanel {
            plugin_id: "graph_viewer".to_string(),
        },
    );

    // Panels and active_panel match; status_message may differ (set by the
    // last Load action), so verify the key structural invariants individually.
    assert_eq!(state1.panels, state2.panels);
    assert_eq!(state1.active_panel, state2.active_panel);
    assert_eq!(state1.error_message, state2.error_message);
}
