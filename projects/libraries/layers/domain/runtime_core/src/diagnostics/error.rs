use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum RuntimeError {
    #[error("cyclic graph detected")]
    CyclicGraph,
    #[error("node not found: {0}")]
    NodeNotFound(String),
    #[error("serialization error: {0}")]
    Serialization(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cyclic_graph_displays() {
        let e = RuntimeError::CyclicGraph;
        assert_eq!(e.to_string(), "cyclic graph detected");
    }

    #[test]
    fn node_not_found_displays() {
        let e = RuntimeError::NodeNotFound("n1".to_string());
        assert_eq!(e.to_string(), "node not found: n1");
    }
}
