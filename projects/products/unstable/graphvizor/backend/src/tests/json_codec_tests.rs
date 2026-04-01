use crate::io::JsonCodec;
use crate::layout::{LayeredDag, LayoutEngine};
use crate::render::SvgRenderer;

#[test]
fn parse_valid_graph() {
    let json = r#"{
        "nodes": [{"id": "a", "label": "Alpha"}, {"id": "b"}],
        "edges": [{"from": "a", "to": "b"}]
    }"#;
    let graph = JsonCodec::parse(json).unwrap();
    assert_eq!(graph.nodes.len(), 2);
    assert_eq!(graph.edges.len(), 1);
    assert_eq!(graph.nodes[0].id, "a");
    assert_eq!(graph.nodes[0].label.as_deref(), Some("Alpha"));
    assert_eq!(graph.nodes[1].label, None);
}

#[test]
fn parse_empty_graph() {
    let json = r#"{"nodes": [], "edges": []}"#;
    let graph = JsonCodec::parse(json).unwrap();
    assert!(graph.nodes.is_empty());
    assert!(graph.edges.is_empty());
}

#[test]
fn parse_missing_nodes_field() {
    let json = r#"{"edges": []}"#;
    let err = JsonCodec::parse(json).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("nodes"), "expected 'nodes' in error: {msg}");
}

#[test]
fn parse_missing_edges_field() {
    let json = r#"{"nodes": []}"#;
    let err = JsonCodec::parse(json).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("edges"), "expected 'edges' in error: {msg}");
}

#[test]
fn parse_nodes_not_array() {
    let json = r#"{"nodes": "oops", "edges": []}"#;
    let err = JsonCodec::parse(json).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("array"), "expected 'array' in error: {msg}");
}

#[test]
fn parse_node_missing_id() {
    let json = r#"{"nodes": [{"label": "no-id"}], "edges": []}"#;
    let err = JsonCodec::parse(json).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("id"), "expected 'id' in error: {msg}");
}

#[test]
fn parse_edge_missing_from() {
    let json = r#"{"nodes": [{"id": "a"}], "edges": [{"to": "a"}]}"#;
    let err = JsonCodec::parse(json).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("from"), "expected 'from' in error: {msg}");
}

#[test]
fn parse_edge_references_unknown_node() {
    let json = r#"{
        "nodes": [{"id": "a"}],
        "edges": [{"from": "a", "to": "missing"}]
    }"#;
    let err = JsonCodec::parse(json).unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("unknown node"),
        "expected 'unknown node' in error: {msg}"
    );
}

#[test]
fn parse_invalid_json() {
    let err = JsonCodec::parse("not json").unwrap_err();
    assert!(err.to_string().contains("json parse error"));
}

#[test]
fn end_to_end_json_to_svg() {
    let json = include_str!("golden/diamond_dag.json");
    let graph = JsonCodec::parse(json).unwrap();
    let positions = LayeredDag.compute(&graph);
    let svg = SvgRenderer::render(&graph, &positions);
    assert!(svg.contains("<svg"));
    assert!(svg.contains("</svg>"));
    assert!(svg.contains("<marker id=\"arrow\""));
}
