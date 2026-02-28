use crate::diagnostics::error::RuntimeError;
use crate::graph::graph::Graph;
use crate::id::runtime_id::RuntimeId;
use crate::scheduler::job::Job;

pub struct Scheduler {
    graph: Graph,
}

impl Scheduler {
    pub fn new(graph: Graph) -> Self {
        Self { graph }
    }

    /// Produces a deterministic sequence of jobs for the graph.
    /// Returns `RuntimeError::CyclicGraph` when the graph contains a cycle.
    pub fn schedule(&self) -> Result<Vec<Job>, RuntimeError> {
        let order = self.graph.topological_order()?;
        let jobs = order
            .into_iter()
            .enumerate()
            .map(|(seq, node_id)| Job::new(RuntimeId::new(seq as u64), node_id))
            .collect();
        Ok(jobs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::edge::Edge;
    use crate::graph::node::Node;

    fn id(v: u64) -> RuntimeId {
        RuntimeId::new(v)
    }

    #[test]
    fn schedule_linear_dag() {
        let graph = Graph::new(
            vec![
                Node::new(id(1), "a"),
                Node::new(id(2), "b"),
                Node::new(id(3), "c"),
            ],
            vec![Edge::new(id(1), id(2)), Edge::new(id(2), id(3))],
        );
        let scheduler = Scheduler::new(graph);
        let jobs = scheduler.schedule().unwrap();
        assert_eq!(jobs.len(), 3);
        assert_eq!(jobs[0].node_id, id(1));
        assert_eq!(jobs[1].node_id, id(2));
        assert_eq!(jobs[2].node_id, id(3));
    }

    #[test]
    fn schedule_rejects_cyclic_graph() {
        let graph = Graph::new(
            vec![Node::new(id(1), "a"), Node::new(id(2), "b")],
            vec![Edge::new(id(1), id(2)), Edge::new(id(2), id(1))],
        );
        let scheduler = Scheduler::new(graph);
        assert!(matches!(
            scheduler.schedule(),
            Err(RuntimeError::CyclicGraph)
        ));
    }
}
