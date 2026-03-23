//! projects/products/unstable/autonomous_dev_ai/src/audit_logger.rs
use crate::audit_logger::AuditLogger;
use std::path::PathBuf;

fn setup_logger() -> AuditLogger {
    AuditLogger::new(PathBuf::from("test_log.json"))
}

#[test]
fn log_state_transition() {
    let logger = setup_logger();
    let result = logger.log_state_transition("state1", "state2");
    assert!(result.is_ok());
}

#[test]
fn log_tool_execution() {
    let logger = setup_logger();
    let args = vec!["arg1".to_string(), "arg2".to_string()];
    let result = logger.log_tool_execution("tool_name", &args, true);
    assert!(result.is_ok());
}

#[test]
fn log_neural_suggestion() {
    let logger = setup_logger();
    let result = logger.log_neural_suggestion("suggestion", 0.95);
    assert!(result.is_ok());
}

#[test]
fn log_symbolic_decision() {
    let logger = setup_logger();
    let result = logger.log_symbolic_decision("decision", "reasoning");
    assert!(result.is_ok());
}

#[test]
fn log_file_modification() {
    let logger = setup_logger();
    let result = logger.log_file_modified("/path/to/file");
    assert!(result.is_ok());
}

#[test]
fn log_objective_evaluation() {
    let logger = setup_logger();
    let scores = vec![("metric1".to_string(), 0.8), ("metric2".to_string(), 0.9)];
    let result = logger.log_objective_evaluation(1, scores);
    assert!(result.is_ok());
}

#[test]
fn log_final_state() {
    let logger = setup_logger();
    let result = logger.log_final_state("final_state", 10);
    assert!(result.is_ok());
}
