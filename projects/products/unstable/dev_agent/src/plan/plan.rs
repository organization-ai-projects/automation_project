use crate::plan::task::Task;
use runtime_core::{Edge, Graph, Node, RuntimeId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub tasks: Vec<Task>,
    pub edges: Vec<(RuntimeId, RuntimeId)>,
}

impl Plan {
    pub fn new(tasks: Vec<Task>, edges: Vec<(RuntimeId, RuntimeId)>) -> Self {
        Self { tasks, edges }
    }

    /// Converts this plan into a `runtime_core::Graph` for scheduling.
    pub fn to_graph(&self) -> Graph {
        let nodes = self
            .tasks
            .iter()
            .map(|t| Node::new(t.id, &t.label))
            .collect();
        let edges = self
            .edges
            .iter()
            .map(|(from, to)| Edge::new(*from, *to))
            .collect();
        Graph::new(nodes, edges)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::task::TaskKind;

    fn make_plan() -> Plan {
        Plan::new(
            vec![
                Task::new(RuntimeId::new(1), "scan", TaskKind::Scan),
                Task::new(RuntimeId::new(2), "plan", TaskKind::Plan),
            ],
            vec![(RuntimeId::new(1), RuntimeId::new(2))],
        )
    }

    #[test]
    fn to_graph_produces_valid_dag() {
        let plan = make_plan();
        let graph = plan.to_graph();
        assert_eq!(graph.nodes().len(), 2);
        assert_eq!(graph.edges().len(), 1);
        assert!(!graph.has_cycle());
    }

    #[test]
    fn serializes_roundtrip() {
        let plan = make_plan();
        let json = serde_json::to_string(&plan).unwrap();
        let restored: Plan = serde_json::from_str(&json).unwrap();
        assert_eq!(plan.tasks, restored.tasks);
        assert_eq!(plan.edges, restored.edges);
    }
}
