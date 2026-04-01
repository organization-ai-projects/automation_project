use common_json::JsonAccess;

use crate::diagnostics::GraphvizorError;
use crate::graph::GraphDefinition;

/// Reads and writes graph JSON files.
pub struct JsonCodec;

impl JsonCodec {
    pub fn load(path: &std::path::Path) -> Result<GraphDefinition, GraphvizorError> {
        let content = std::fs::read_to_string(path)?;
        Self::parse(&content)
    }

    pub fn parse(json_str: &str) -> Result<GraphDefinition, GraphvizorError> {
        let value: common_json::Json = common_json::from_str(json_str)
            .map_err(|e| GraphvizorError::JsonParse(e.to_string()))?;

        let nodes_val = value
            .get_field("nodes")
            .map_err(|_| GraphvizorError::JsonParse("missing 'nodes' field".to_string()))?;
        let edges_val = value
            .get_field("edges")
            .map_err(|_| GraphvizorError::JsonParse("missing 'edges' field".to_string()))?;

        let nodes_arr = nodes_val
            .as_array()
            .ok_or_else(|| GraphvizorError::JsonParse("'nodes' must be an array".to_string()))?;
        let edges_arr = edges_val
            .as_array()
            .ok_or_else(|| GraphvizorError::JsonParse("'edges' must be an array".to_string()))?;

        let mut nodes = Vec::new();
        for item in nodes_arr {
            let id = item
                .get_field("id")
                .map_err(|_| GraphvizorError::JsonParse("node missing 'id'".to_string()))?
                .as_str()
                .ok_or_else(|| {
                    GraphvizorError::JsonParse("node 'id' must be a string".to_string())
                })?
                .to_string();
            let label = item
                .get_field("label")
                .ok()
                .and_then(|v| v.as_str().map(|s| s.to_string()));
            nodes.push(crate::graph::Node { id, label });
        }

        let node_ids: std::collections::BTreeSet<&str> =
            nodes.iter().map(|n| n.id.as_str()).collect();

        let mut edges = Vec::new();
        for item in edges_arr {
            let from = item
                .get_field("from")
                .map_err(|_| GraphvizorError::JsonParse("edge missing 'from'".to_string()))?
                .as_str()
                .ok_or_else(|| {
                    GraphvizorError::JsonParse("edge 'from' must be a string".to_string())
                })?
                .to_string();
            let to = item
                .get_field("to")
                .map_err(|_| GraphvizorError::JsonParse("edge missing 'to'".to_string()))?
                .as_str()
                .ok_or_else(|| {
                    GraphvizorError::JsonParse("edge 'to' must be a string".to_string())
                })?
                .to_string();

            if !node_ids.contains(from.as_str()) {
                return Err(GraphvizorError::JsonParse(format!(
                    "edge references unknown node '{from}'"
                )));
            }
            if !node_ids.contains(to.as_str()) {
                return Err(GraphvizorError::JsonParse(format!(
                    "edge references unknown node '{to}'"
                )));
            }

            edges.push(crate::graph::Edge { from, to });
        }

        Ok(GraphDefinition { nodes, edges })
    }
}
