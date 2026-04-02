use crate::plugins::builtin_plugins::BuiltinPlugins;

#[test]
fn builtin_log_viewer() {
    let plugin = BuiltinPlugins::log_viewer();
    assert_eq!(plugin.id().as_str(), "log_viewer");
    assert_eq!(plugin.name(), "Log Viewer");
}

#[test]
fn builtin_report_viewer() {
    let plugin = BuiltinPlugins::report_viewer();
    assert_eq!(plugin.id().as_str(), "report_viewer");
    assert_eq!(plugin.name(), "Report Viewer");
}

#[test]
fn builtin_graph_viewer() {
    let plugin = BuiltinPlugins::graph_viewer();
    assert_eq!(plugin.id().as_str(), "graph_viewer");
    assert_eq!(plugin.name(), "Graph Viewer");
}

#[test]
fn builtin_all_returns_three_plugins() {
    let plugins = BuiltinPlugins::all();
    assert_eq!(plugins.len(), 3);
}

#[test]
fn builtin_all_is_sorted() {
    let plugins = BuiltinPlugins::all();
    let ids: Vec<&str> = plugins.iter().map(|p| p.id().as_str()).collect();
    assert_eq!(ids, vec!["graph_viewer", "log_viewer", "report_viewer"]);
}

#[test]
fn builtin_all_is_deterministic() {
    let a = BuiltinPlugins::all();
    let b = BuiltinPlugins::all();
    assert_eq!(a, b);
}
