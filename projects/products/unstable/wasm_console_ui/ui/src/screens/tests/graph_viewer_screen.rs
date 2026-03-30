use crate::screens::graph_viewer_screen::GraphViewerScreen;

#[test]
fn graph_viewer_empty() {
    let screen = GraphViewerScreen::new();
    assert_eq!(screen.node_count(), 0);
    assert_eq!(screen.edge_count(), 0);
}

#[test]
fn graph_viewer_from_json() {
    let json = r#"{"nodes":[{"id":"a","label":"A"},{"id":"b","label":"B"}],"edges":[{"from":"a","to":"b"}]}"#;
    let screen = GraphViewerScreen::from_json(json).unwrap();
    assert_eq!(screen.node_count(), 2);
    assert_eq!(screen.edge_count(), 1);
}

#[test]
fn graph_viewer_from_invalid_json() {
    let result = GraphViewerScreen::from_json("not json");
    assert!(result.is_err());
}

#[test]
fn graph_viewer_title() {
    let screen = GraphViewerScreen::new();
    assert_eq!(screen.title, "Graph Viewer");
}

#[test]
fn graph_viewer_default() {
    let screen = GraphViewerScreen::default();
    assert_eq!(screen.node_count(), 0);
}
