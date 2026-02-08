// projects/products/unstable/autonomous_dev_ai/tests/integration_test.rs

use autonomous_dev_ai::{AutonomousAgent, config::AgentConfig, load_config, save_ron};
use std::fs;
use std::path::Path;

#[test]
fn test_agent_creation() {
    let test_dir = "/tmp/autonomous_dev_ai_test_creation";
    fs::create_dir_all(test_dir).unwrap();

    let config_path = format!("{}/test_config", test_dir);
    let audit_path = format!("{}/test_audit.log", test_dir);

    // Create default config
    let config = AgentConfig::default();
    save_ron(format!("{}.ron", config_path), &config).unwrap();

    // Create agent
    let agent = AutonomousAgent::new(&config_path, &audit_path);
    assert!(agent.is_ok());

    // Cleanup
    fs::remove_dir_all(test_dir).ok();
}

#[test]
fn test_config_serialization() {
    let test_dir = "/tmp/autonomous_dev_ai_test_config";
    fs::create_dir_all(test_dir).unwrap();

    let config_path = format!("{}/test_config", test_dir);

    // Create and save config
    let config = AgentConfig::default();
    save_ron(format!("{}.ron", config_path), &config).unwrap();

    // Load config
    let loaded = load_config(&config_path).unwrap();

    assert_eq!(loaded.agent_name, config.agent_name);
    assert_eq!(loaded.execution_mode, config.execution_mode);
    assert_eq!(loaded.objectives.len(), config.objectives.len());

    // Cleanup
    fs::remove_dir_all(test_dir).ok();
}

#[test]
fn test_state_save_and_load() {
    let test_dir = "/tmp/autonomous_dev_ai_test_state";
    fs::create_dir_all(test_dir).unwrap();

    let config_path = format!("{}/test_config", test_dir);
    let audit_path = format!("{}/test_audit.log", test_dir);

    // Create config
    let config = AgentConfig::default();
    save_ron(format!("{}.ron", config_path), &config).unwrap();

    // Create agent and add some data
    let mut agent = AutonomousAgent::new(&config_path, &audit_path).unwrap();
    agent
        .lifecycle
        .memory
        .add_explored_file("test.rs".to_string());
    agent.lifecycle.memory.add_decision(
        0,
        "test decision".to_string(),
        None,
        "symbolic choice".to_string(),
    );

    // Save state
    agent.save_state().unwrap();

    // Create new agent and load state
    let mut agent2 = AutonomousAgent::new(&config_path, &audit_path).unwrap();
    agent2.load_state().unwrap();

    assert_eq!(agent2.lifecycle.memory.explored_files.len(), 1);
    assert_eq!(agent2.lifecycle.memory.decisions.len(), 1);

    // Cleanup
    fs::remove_dir_all(test_dir).ok();
}

#[test]
fn test_symbolic_only_mode() {
    let test_dir = "/tmp/autonomous_dev_ai_test_symbolic";
    fs::create_dir_all(test_dir).unwrap();

    let config_path = format!("{}/test_config", test_dir);
    let audit_path = format!("{}/test_audit.log", test_dir);

    // Create config with neural enabled
    let mut config = AgentConfig::default();
    config.neural.enabled = true;
    save_ron(format!("{}.ron", config_path), &config).unwrap();

    // Create agent
    let mut agent = AutonomousAgent::new(&config_path, &audit_path).unwrap();

    // Verify neural is initially enabled
    assert!(agent.lifecycle.neural.enabled);

    // Run in symbolic-only mode
    let result = agent.run_symbolic_only("test goal");

    // Should complete successfully
    assert!(result.is_ok());

    // Neural should be disabled
    assert!(!agent.lifecycle.neural.enabled);

    // Cleanup
    fs::remove_dir_all(test_dir).ok();
}

#[test]
fn test_autonomous_iterations() {
    let test_dir = "/tmp/autonomous_dev_ai_test_iterations";
    fs::create_dir_all(test_dir).unwrap();

    let config_path = format!("{}/test_config", test_dir);
    let audit_path = format!("{}/test_audit.log", test_dir);

    // Create config
    let config = AgentConfig::default();
    save_ron(format!("{}.ron", config_path), &config).unwrap();

    // Create and run agent
    let mut agent = AutonomousAgent::new(&config_path, &audit_path).unwrap();
    let result = agent.run("test goal: fix issues");

    // Should complete
    assert!(result.is_ok());

    // Should have completed at least 2 iterations (acceptance criteria)
    assert!(
        agent.lifecycle.iteration >= 2,
        "Agent must complete at least 2 iterations"
    );

    // Should reach a terminal state
    assert!(agent.lifecycle.state.is_terminal());

    // Should have audit log
    assert!(Path::new(&audit_path).exists());

    // Cleanup
    fs::remove_dir_all(test_dir).ok();
}

#[test]
fn test_policy_enforcement() {
    let test_dir = "/tmp/autonomous_dev_ai_test_policy";
    fs::create_dir_all(test_dir).unwrap();

    let config_path = format!("{}/test_config", test_dir);
    let audit_path = format!("{}/test_audit.log", test_dir);

    // Create config
    let config = AgentConfig::default();
    save_ron(format!("{}.ron", config_path), &config).unwrap();

    let agent = AutonomousAgent::new(&config_path, &audit_path).unwrap();

    // Test policy engine
    assert!(!agent.lifecycle.policy.validate_action("force-push"));
    assert!(!agent.lifecycle.policy.validate_action("rm -rf /"));
    assert!(agent.lifecycle.policy.validate_action("git commit"));

    // Cleanup
    fs::remove_dir_all(test_dir).ok();
}

#[test]
fn test_objectives_evaluation() {
    use autonomous_dev_ai::objectives::{ObjectiveEvaluator, default_objectives};

    let objectives = default_objectives();
    let evaluator = ObjectiveEvaluator::new(objectives);

    // Test with all objectives met
    let scores = vec![
        ("task_completion".to_string(), 1.0),
        ("policy_safety".to_string(), 1.0),
        ("tests_pass".to_string(), 1.0),
        ("minimal_diff".to_string(), 0.8),
        ("time_budget".to_string(), 0.5),
    ];

    let results = evaluator.evaluate(&scores);
    assert!(evaluator.hard_objectives_satisfied(&results));

    // Test with hard objective failed
    let scores_failed = vec![
        ("task_completion".to_string(), 0.5),
        ("policy_safety".to_string(), 1.0),
        ("tests_pass".to_string(), 1.0),
        ("minimal_diff".to_string(), 0.8),
        ("time_budget".to_string(), 0.5),
    ];

    let results_failed = evaluator.evaluate(&scores_failed);
    assert!(!evaluator.hard_objectives_satisfied(&results_failed));
}
