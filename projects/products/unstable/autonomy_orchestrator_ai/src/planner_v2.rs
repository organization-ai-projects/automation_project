// projects/products/unstable/autonomy_orchestrator_ai/src/planner_v2.rs
use crate::domain::{PlannerEdge, PlannerNode, PlannerPathRecord};

pub const REASON_GRAPH_INVALID: &str = "PLANNER_GRAPH_INVALID";
pub const REASON_FALLBACK_STEP_APPLIED: &str = "PLANNER_FALLBACK_STEP_APPLIED";
pub const REASON_FALLBACK_BUDGET_EXHAUSTED: &str = "PLANNER_FALLBACK_BUDGET_EXHAUSTED";

pub struct PlannerGraph {
    pub nodes: Vec<PlannerNode>,
    pub edges: Vec<PlannerEdge>,
}

pub struct PlannerV2Result {
    pub path_record: PlannerPathRecord,
    /// True when the fallback budget was exhausted (fail-closed required).
    pub budget_exhausted: bool,
}

/// Validate the graph for structural correctness.
///
/// Rules (deterministic):
/// - Node IDs must be non-empty and unique.
/// - Edge `from`/`to` must reference existing node IDs.
/// - No duplicate edges (same from/to/condition_code triple).
/// - The graph must have exactly one root (node with no incoming edges).
/// - The graph must be acyclic (no cycles reachable from root).
pub fn validate_graph(nodes: &[PlannerNode], edges: &[PlannerEdge]) -> Result<(), String> {
    // Non-empty IDs and uniqueness.
    let mut seen_ids = std::collections::BTreeSet::new();
    for node in nodes {
        if node.id.is_empty() {
            return Err("PlannerNode id must not be empty".to_string());
        }
        if !seen_ids.insert(node.id.as_str()) {
            return Err(format!("Duplicate PlannerNode id '{}'", node.id));
        }
    }

    // Edge references valid nodes.
    let node_ids: std::collections::BTreeSet<&str> = nodes.iter().map(|n| n.id.as_str()).collect();
    let mut seen_edges = std::collections::BTreeSet::new();
    for edge in edges {
        if !node_ids.contains(edge.from.as_str()) {
            return Err(format!(
                "PlannerEdge references unknown from-node '{}'",
                edge.from
            ));
        }
        if !node_ids.contains(edge.to.as_str()) {
            return Err(format!(
                "PlannerEdge references unknown to-node '{}'",
                edge.to
            ));
        }
        let triple = (&edge.from, &edge.to, &edge.condition_code);
        if !seen_edges.insert(triple) {
            return Err(format!(
                "Duplicate PlannerEdge from '{}' to '{}' with condition_code '{}'",
                edge.from, edge.to, edge.condition_code
            ));
        }
    }

    // Find root: nodes with no incoming edges.
    let nodes_with_incoming: std::collections::BTreeSet<&str> =
        edges.iter().map(|e| e.to.as_str()).collect();
    let roots: Vec<&str> = node_ids
        .iter()
        .copied()
        .filter(|id| !nodes_with_incoming.contains(id))
        .collect();
    if nodes.is_empty() {
        // Empty graph is valid (no planning graph provided).
        return Ok(());
    }
    if roots.len() != 1 {
        return Err(format!(
            "PlannerGraph must have exactly one root node (found {})",
            roots.len()
        ));
    }

    // Cycle detection via DFS from root.
    let root = roots[0];
    let adj = build_adjacency(edges);
    let mut visited = std::collections::BTreeSet::new();
    let mut stack = std::collections::BTreeSet::new();
    detect_cycle(root, &adj, &mut visited, &mut stack)?;

    Ok(())
}

fn build_adjacency<'a>(
    edges: &'a [PlannerEdge],
) -> std::collections::BTreeMap<&'a str, Vec<&'a PlannerEdge>> {
    let mut adj: std::collections::BTreeMap<&str, Vec<&PlannerEdge>> =
        std::collections::BTreeMap::new();
    for edge in edges {
        adj.entry(edge.from.as_str()).or_default().push(edge);
    }
    // Sort edges for determinism: primary (empty condition_code) first, then by (condition_code, to).
    for neighbours in adj.values_mut() {
        neighbours.sort_by(|a, b| {
            a.condition_code
                .cmp(&b.condition_code)
                .then(a.to.cmp(&b.to))
        });
    }
    adj
}

fn detect_cycle<'a>(
    node: &'a str,
    adj: &std::collections::BTreeMap<&'a str, Vec<&'a PlannerEdge>>,
    visited: &mut std::collections::BTreeSet<&'a str>,
    stack: &mut std::collections::BTreeSet<&'a str>,
) -> Result<(), String> {
    if stack.contains(node) {
        return Err(format!("PlannerGraph contains a cycle at node '{node}'"));
    }
    if visited.contains(node) {
        return Ok(());
    }
    visited.insert(node);
    stack.insert(node);
    if let Some(neighbours) = adj.get(node) {
        for edge in &*neighbours {
            detect_cycle(edge.to.as_str(), adj, visited, stack)?;
        }
    }
    stack.remove(node);
    Ok(())
}

/// Select a deterministic path through the graph, bounded by `fallback_max_steps`.
///
/// - Primary edges (empty `condition_code`) are followed first.
/// - Fallback edges (non-empty `condition_code`) are followed when no primary edge exists;
///   each costs one step from the budget.
/// - If the budget is exhausted before reaching a terminal node, the result is marked
///   `budget_exhausted = true`.
pub fn select_path(graph: &PlannerGraph, fallback_max_steps: u32) -> PlannerV2Result {
    if graph.nodes.is_empty() {
        return PlannerV2Result {
            path_record: PlannerPathRecord {
                selected_path: Vec::new(),
                fallback_steps_used: 0,
                reason_codes: Vec::new(),
            },
            budget_exhausted: false,
        };
    }

    let adj = build_adjacency(&graph.edges);
    let nodes_with_incoming: std::collections::BTreeSet<&str> =
        graph.edges.iter().map(|e| e.to.as_str()).collect();
    // Root = node with no incoming edges; exactly one guaranteed after validate_graph.
    let root = graph
        .nodes
        .iter()
        .map(|n| n.id.as_str())
        .find(|id| !nodes_with_incoming.contains(id))
        .unwrap_or_else(|| graph.nodes[0].id.as_str());

    let mut selected_path: Vec<String> = vec![root.to_string()];
    let mut fallback_steps_used: u32 = 0;
    let mut reason_codes: Vec<String> = Vec::new();
    let mut budget_exhausted = false;

    let mut current = root.to_string();
    let mut visited = std::collections::BTreeSet::new();
    visited.insert(current.clone());

    loop {
        let Some(neighbours) = adj.get(current.as_str()) else {
            // Terminal node.
            break;
        };

        // Separate primary (empty condition_code) from fallback.
        let primary: Vec<&&PlannerEdge> = neighbours
            .iter()
            .filter(|e| e.condition_code.is_empty())
            .collect();
        let fallback: Vec<&&PlannerEdge> = neighbours
            .iter()
            .filter(|e| !e.condition_code.is_empty())
            .collect();

        if let Some(edge) = primary.first() {
            // Take primary edge.
            let next = edge.to.clone();
            if visited.contains(next.as_str()) {
                break;
            }
            visited.insert(next.clone());
            selected_path.push(next.clone());
            current = next;
        } else if let Some(edge) = fallback.first() {
            // Fallback edge.
            if fallback_steps_used >= fallback_max_steps {
                budget_exhausted = true;
                if !reason_codes.contains(&REASON_FALLBACK_BUDGET_EXHAUSTED.to_string()) {
                    reason_codes.push(REASON_FALLBACK_BUDGET_EXHAUSTED.to_string());
                }
                break;
            }
            fallback_steps_used += 1;
            if !reason_codes.contains(&REASON_FALLBACK_STEP_APPLIED.to_string()) {
                reason_codes.push(REASON_FALLBACK_STEP_APPLIED.to_string());
            }
            let next = edge.to.clone();
            if visited.contains(next.as_str()) {
                break;
            }
            visited.insert(next.clone());
            selected_path.push(next.clone());
            current = next;
        } else {
            // No outgoing edges from current (terminal).
            break;
        }
    }

    PlannerV2Result {
        path_record: PlannerPathRecord {
            selected_path,
            fallback_steps_used,
            reason_codes,
        },
        budget_exhausted,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{PlannerEdge, PlannerNode};

    fn node(id: &str) -> PlannerNode {
        PlannerNode {
            id: id.to_string(),
            action: id.to_string(),
        }
    }

    fn edge(from: &str, to: &str, condition_code: &str) -> PlannerEdge {
        PlannerEdge {
            from: from.to_string(),
            to: to.to_string(),
            condition_code: condition_code.to_string(),
        }
    }

    // --- Graph validation tests ---

    #[test]
    fn validate_empty_graph_is_ok() {
        assert!(validate_graph(&[], &[]).is_ok());
    }

    #[test]
    fn validate_single_node_no_edges_is_ok() {
        assert!(validate_graph(&[node("a")], &[]).is_ok());
    }

    #[test]
    fn validate_linear_graph_is_ok() {
        let nodes = vec![node("a"), node("b"), node("c")];
        let edges = vec![edge("a", "b", ""), edge("b", "c", "")];
        assert!(validate_graph(&nodes, &edges).is_ok());
    }

    #[test]
    fn validate_rejects_duplicate_node_id() {
        let nodes = vec![node("a"), node("a")];
        let result = validate_graph(&nodes, &[]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Duplicate"));
    }

    #[test]
    fn validate_rejects_empty_node_id() {
        let nodes = vec![node("")];
        let result = validate_graph(&nodes, &[]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty"));
    }

    #[test]
    fn validate_rejects_unknown_edge_from() {
        let nodes = vec![node("a")];
        let edges = vec![edge("z", "a", "")];
        let result = validate_graph(&nodes, &edges);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("unknown from-node"));
    }

    #[test]
    fn validate_rejects_unknown_edge_to() {
        let nodes = vec![node("a")];
        let edges = vec![edge("a", "z", "")];
        let result = validate_graph(&nodes, &edges);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("unknown to-node"));
    }

    #[test]
    fn validate_rejects_duplicate_edge() {
        let nodes = vec![node("a"), node("b")];
        let edges = vec![edge("a", "b", ""), edge("a", "b", "")];
        let result = validate_graph(&nodes, &edges);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Duplicate PlannerEdge"));
    }

    #[test]
    fn validate_rejects_multiple_roots() {
        let nodes = vec![node("a"), node("b"), node("c")];
        let edges = vec![edge("a", "c", ""), edge("b", "c", "")];
        let result = validate_graph(&nodes, &edges);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("root"));
    }

    #[test]
    fn validate_rejects_cycle() {
        let nodes = vec![node("a"), node("b"), node("c")];
        let edges = vec![edge("a", "b", ""), edge("b", "c", ""), edge("c", "b", "")];
        let result = validate_graph(&nodes, &edges);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cycle"));
    }

    // --- Path selection tests ---

    #[test]
    fn select_path_empty_graph_returns_empty() {
        let graph = PlannerGraph {
            nodes: vec![],
            edges: vec![],
        };
        let result = select_path(&graph, 3);
        assert_eq!(result.path_record.selected_path, Vec::<String>::new());
        assert_eq!(result.path_record.fallback_steps_used, 0);
        assert!(!result.budget_exhausted);
    }

    #[test]
    fn select_path_single_node_returns_root() {
        let graph = PlannerGraph {
            nodes: vec![node("start")],
            edges: vec![],
        };
        let result = select_path(&graph, 3);
        assert_eq!(result.path_record.selected_path, vec!["start"]);
        assert_eq!(result.path_record.fallback_steps_used, 0);
        assert!(!result.budget_exhausted);
    }

    #[test]
    fn select_path_linear_primary_path_is_deterministic() {
        let graph = PlannerGraph {
            nodes: vec![node("a"), node("b"), node("c")],
            edges: vec![edge("a", "b", ""), edge("b", "c", "")],
        };
        let result1 = select_path(&graph, 3);
        let result2 = select_path(&graph, 3);
        assert_eq!(result1.path_record.selected_path, vec!["a", "b", "c"]);
        assert_eq!(result1.path_record, result2.path_record);
        assert_eq!(result1.path_record.fallback_steps_used, 0);
        assert!(!result1.budget_exhausted);
    }

    #[test]
    fn select_path_fallback_edge_is_taken_when_no_primary() {
        let graph = PlannerGraph {
            nodes: vec![node("a"), node("b"), node("c")],
            edges: vec![edge("a", "b", "ON_FAIL"), edge("b", "c", "")],
        };
        let result = select_path(&graph, 3);
        assert_eq!(result.path_record.selected_path, vec!["a", "b", "c"]);
        assert_eq!(result.path_record.fallback_steps_used, 1);
        assert!(
            result
                .path_record
                .reason_codes
                .contains(&REASON_FALLBACK_STEP_APPLIED.to_string())
        );
        assert!(!result.budget_exhausted);
    }

    #[test]
    fn select_path_fallback_budget_exhaustion_fails_closed() {
        let graph = PlannerGraph {
            nodes: vec![node("a"), node("b"), node("c"), node("d")],
            edges: vec![
                edge("a", "b", "ON_FAIL"),
                edge("b", "c", "ON_FAIL"),
                edge("c", "d", "ON_FAIL"),
            ],
        };
        // Budget of 2 means we can take 2 fallback steps (a→b, b→c) but not c→d.
        let result = select_path(&graph, 2);
        assert_eq!(result.path_record.fallback_steps_used, 2);
        assert!(result.budget_exhausted);
        assert!(
            result
                .path_record
                .reason_codes
                .contains(&REASON_FALLBACK_BUDGET_EXHAUSTED.to_string())
        );
    }

    #[test]
    fn select_path_primary_preferred_over_fallback() {
        let graph = PlannerGraph {
            nodes: vec![node("a"), node("b"), node("c")],
            edges: vec![edge("a", "b", ""), edge("a", "c", "ON_FAIL")],
        };
        let result = select_path(&graph, 3);
        // Primary edge a→b should be taken, not fallback a→c.
        assert_eq!(result.path_record.selected_path, vec!["a", "b"]);
        assert_eq!(result.path_record.fallback_steps_used, 0);
        assert!(!result.budget_exhausted);
    }

    #[test]
    fn select_path_is_deterministic_across_multiple_calls() {
        let graph = PlannerGraph {
            nodes: vec![node("a"), node("b"), node("c"), node("d")],
            edges: vec![
                edge("a", "b", ""),
                edge("a", "c", "ON_FAIL"),
                edge("b", "d", ""),
            ],
        };
        let r1 = select_path(&graph, 3);
        let r2 = select_path(&graph, 3);
        let r3 = select_path(&graph, 3);
        assert_eq!(r1.path_record, r2.path_record);
        assert_eq!(r2.path_record, r3.path_record);
    }
}
