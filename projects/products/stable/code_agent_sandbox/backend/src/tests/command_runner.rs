//! projects/products/code_agent_sandbox/src/tests/command_runner.rs
use crate::command_runner::CommandRunner;

#[test]
fn requires_bunker_flags_sensitive_subcommands() {
    assert!(CommandRunner::requires_bunker("install"));
    assert!(CommandRunner::requires_bunker("publish"));
    assert!(!CommandRunner::requires_bunker("check"));
}
