use crate::graph::{Edge, GraphDefinition, Node};
use crate::layout::{LayeredDag, LayoutEngine, SimpleForce};

fn diamond_graph() -> GraphDefinition {
    GraphDefinition {
        nodes: vec![
            Node { id: "a".to_string(), label: None },
            Node { id: "b".to_string(), label: None },
            Node { id: "c".to_string(), label: None },
            Node { id: "d".to_string(), label: None },
        ],
        edges: vec![
            Edge { from: "a".to_string(), to: "b".to_string() },
            Edge { from: "a".to_string(), to: "c".to_string() },
            Edge { from: "b".to_string(), to: "d".to_string() },
            Edge { from: "c".to_string(), to: "d".to_string() },
        ],
    }
}

fn linear_chain() -> GraphDefinition {
    GraphDefinition {
        nodes: vec![
            Node { id: "start".to_string(), label: Some("Start".to_string()) },
            Node { id: "middle".to_string(), label: Some("Middle".to_string()) },
            Node { id: "end".to_string(), label: Some("End".to_string()) },
        ],
        edges: vec![
            Edge { from: "start".to_string(), to: "middle".to_string() },
            Edge { from: "middle".to_string(), to: "end".to_string() },
        ],
    }
}

fn single_node() -> GraphDefinition {
    GraphDefinition {
        nodes: vec![Node { id: "solo".to_string(), label: Some("Solo Node".to_string()) }],
        edges: vec![],
    }
}

#[test]
fn layered_dag_deterministic_across_runs() {
    let graph = diamond_graph();
    let p1 = LayeredDag.compute(&graph);
    let p2 = LayeredDag.compute(&graph);
    assert_eq!(p1.len(), p2.len());
    for (a, b) in p1.iter().zip(p2.iter()) {
        assert_eq!(a.id, b.id);
        assert_eq!(a.x, b.x);
        assert_eq!(a.y, b.y);
    }
}

#[test]
fn layered_dag_deterministic_with_reordered_input() {
    let graph1 = diamond_graph();
    let mut graph2 = diamond_graph();
    graph2.nodes.reverse();
    graph2.edges.reverse();
    let p1 = LayeredDag.compute(&graph1);
    let p2 = LayeredDag.compute(&graph2);
    assert_eq!(p1.len(), p2.len());
    for (a, b) in p1.iter().zip(p2.iter()) {
        assert_eq!(a.id, b.id);
        assert_eq!(a.x, b.x);
        assert_eq!(a.y, b.y);
    }
}

#[test]
fn simple_force_deterministic_across_runs() {
    let graph = diamond_graph();
    let engine = SimpleForce::default();
    let p1 = engine.compute(&graph);
    let p2 = engine.compute(&graph);
    assert_eq!(p1.len(), p2.len());
    for (a, b) in p1.iter().zip(p2.iter()) {
        assert_eq!(a.id, b.id);
        assert_eq!(a.x, b.x);
        assert_eq!(a.y, b.y);
    }
}

#[test]
fn simple_force_deterministic_with_reordered_input() {
    let mut graph2 = diamond_graph();
    graph2.nodes.reverse();
    graph2.edges.reverse();
    let engine = SimpleForce::default();
    let p1 = engine.compute(&diamond_graph());
    let p2 = engine.compute(&graph2);
    assert_eq!(p1.len(), p2.len());
    for (a, b) in p1.iter().zip(p2.iter()) {
        assert_eq!(a.id, b.id);
        assert_eq!(a.x, b.x);
        assert_eq!(a.y, b.y);
    }
}

#[test]
fn layered_dag_single_node() {
    let graph = single_node();
    let positions = LayeredDag.compute(&graph);
    assert_eq!(positions.len(), 1);
    assert_eq!(positions[0].id, "solo");
}

#[test]
fn layered_dag_linear_chain_layers() {
    let graph = linear_chain();
    let positions = LayeredDag.compute(&graph);
    assert_eq!(positions.len(), 3);
    let end_pos = positions.iter().find(|p| p.id == "end").unwrap();
    let mid_pos = positions.iter().find(|p| p.id == "middle").unwrap();
    let start_pos = positions.iter().find(|p| p.id == "start").unwrap();
    // Each should be on a different layer (different y)
    assert!(start_pos.y < mid_pos.y);
    assert!(mid_pos.y < end_pos.y);
}

#[test]
fn simple_force_single_node() {
    let graph = single_node();
    let positions = SimpleForce::default().compute(&graph);
    assert_eq!(positions.len(), 1);
    assert_eq!(positions[0].id, "solo");
}

#[test]
fn empty_graph_produces_no_positions() {
    let graph = GraphDefinition { nodes: vec![], edges: vec![] };
    let p1 = LayeredDag.compute(&graph);
    let p2 = SimpleForce::default().compute(&graph);
    assert!(p1.is_empty());
    assert!(p2.is_empty());
}
