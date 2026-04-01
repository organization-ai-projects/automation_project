use crate::graph::{Edge, GraphDefinition, Node};
use crate::layout::{LayeredDag, LayoutEngine};
use crate::render::SvgRenderer;

fn diamond_graph() -> GraphDefinition {
    GraphDefinition {
        nodes: vec![
            Node {
                id: "a".to_string(),
                label: None,
            },
            Node {
                id: "b".to_string(),
                label: None,
            },
            Node {
                id: "c".to_string(),
                label: None,
            },
            Node {
                id: "d".to_string(),
                label: None,
            },
        ],
        edges: vec![
            Edge {
                from: "a".to_string(),
                to: "b".to_string(),
            },
            Edge {
                from: "a".to_string(),
                to: "c".to_string(),
            },
            Edge {
                from: "b".to_string(),
                to: "d".to_string(),
            },
            Edge {
                from: "c".to_string(),
                to: "d".to_string(),
            },
        ],
    }
}

fn linear_chain() -> GraphDefinition {
    GraphDefinition {
        nodes: vec![
            Node {
                id: "start".to_string(),
                label: Some("Start".to_string()),
            },
            Node {
                id: "middle".to_string(),
                label: Some("Middle".to_string()),
            },
            Node {
                id: "end".to_string(),
                label: Some("End".to_string()),
            },
        ],
        edges: vec![
            Edge {
                from: "start".to_string(),
                to: "middle".to_string(),
            },
            Edge {
                from: "middle".to_string(),
                to: "end".to_string(),
            },
        ],
    }
}

fn single_node() -> GraphDefinition {
    GraphDefinition {
        nodes: vec![Node {
            id: "solo".to_string(),
            label: Some("Solo Node".to_string()),
        }],
        edges: vec![],
    }
}

#[test]
fn golden_diamond_dag_layered_svg() {
    let graph = diamond_graph();
    let positions = LayeredDag.compute(&graph);
    let svg = SvgRenderer::render(&graph, &positions);
    let golden = include_str!("golden/diamond_dag_layered.svg");
    assert_eq!(
        svg.trim(),
        golden.trim(),
        "diamond DAG layered SVG mismatch"
    );
}

#[test]
fn golden_linear_chain_layered_svg() {
    let graph = linear_chain();
    let positions = LayeredDag.compute(&graph);
    let svg = SvgRenderer::render(&graph, &positions);
    let golden = include_str!("golden/linear_chain_layered.svg");
    assert_eq!(
        svg.trim(),
        golden.trim(),
        "linear chain layered SVG mismatch"
    );
}

#[test]
fn golden_single_node_layered_svg() {
    let graph = single_node();
    let positions = LayeredDag.compute(&graph);
    let svg = SvgRenderer::render(&graph, &positions);
    let golden = include_str!("golden/single_node_layered.svg");
    assert_eq!(
        svg.trim(),
        golden.trim(),
        "single node layered SVG mismatch"
    );
}
