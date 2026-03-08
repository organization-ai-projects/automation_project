// projects/products/unstable/agent_engine/backend/src/protocol/tests/cli_io.rs
use std::collections::BTreeMap;

use crate::{engine::agent_outcome::AgentOutcome, protocol};

#[test]
fn write_outcome_serializes_json() {
    let outcome = AgentOutcome {
        task_id: "task-1".to_string(),
        success: true,
        step_results: Vec::new(),
        output: BTreeMap::new(),
        logs: Vec::new(),
    };

    let result = protocol::cli_io::write_outcome(&outcome);
    assert!(result.is_ok());
}
