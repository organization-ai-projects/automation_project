use serde::{Deserialize, Serialize};

/// A minimal graph node for display.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
}

/// A minimal graph edge for display.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
}

/// Describes what the graph viewer screen needs for rendering.
pub struct GraphViewerScreen {
    pub title: String,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

impl GraphViewerScreen {
    pub fn new() -> Self {
        Self {
            title: "Graph Viewer".to_string(),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn from_json(json_str: &str) -> Result<Self, String> {
        let parsed: GraphData = common_json::from_str(json_str).map_err(|e| e.to_string())?;
        Ok(Self {
            title: "Graph Viewer".to_string(),
            nodes: parsed.nodes,
            edges: parsed.edges,
        })
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

impl Default for GraphViewerScreen {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
struct GraphData {
    nodes: Vec<GraphNode>,
    edges: Vec<GraphEdge>,
}
