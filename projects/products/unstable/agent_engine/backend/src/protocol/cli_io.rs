//! projects/products/unstable/agent_engine/backend/src/protocol/cli_io.rs

use crate::{diagnostics::AgentEngineError, engine::AgentOutcome};

pub fn write_outcome(outcome: &AgentOutcome) -> Result<(), AgentEngineError> {
    println!("{}", common_json::to_string_pretty(outcome)?);
    Ok(())
}

pub fn write_error(err: &AgentEngineError) {
    eprintln!("Error: {err}");
}
