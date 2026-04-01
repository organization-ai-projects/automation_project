use crate::graph::GraphDefinition;
use crate::layout::node_position::NodePosition;

/// Trait for deterministic layout engines.
pub trait LayoutEngine {
    fn compute(&self, graph: &GraphDefinition) -> Vec<NodePosition>;
}
