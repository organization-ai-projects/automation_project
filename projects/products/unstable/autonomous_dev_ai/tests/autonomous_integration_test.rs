// projects/products/unstable/autonomous_dev_ai/tests/integration_test.rs

use autonomous_dev_ai::{AutonomousAgent, config::AgentConfig, load_config, save_ron};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static TEST_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

struct TestDirGuard {
    path: PathBuf,
}

impl TestDirGuard {
    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestDirGuard {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn create_unique_test_dir(label: &str) -> TestDirGuard {
    let sequence = TEST_DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    let timestamp_nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let mut path = std::env::temp_dir();
    path.push(format!(
        "autonomous_dev_ai_{label}_{}_{}_{}",
        std::process::id(),
        timestamp_nanos,
        sequence
    ));
    fs::create_dir_all(&path).expect("create test directory");
    TestDirGuard { path }
}

#[test]
fn test_agent_creation() {
    let test_dir = create_unique_test_dir("creation");
    let config_path = test_dir.path().join("test_config");
    let audit_path = test_dir.path().join("test_audit.log");
    let config_path_str = config_path.to_string_lossy().into_owned();
    let audit_path_str = audit_path.to_string_lossy().into_owned();

    // Create default config
    let config = AgentConfig::default();
    save_ron(format!("{}.ron", config_path_str), &config).unwrap();

    // Create agent
    let agent = AutonomousAgent::new(&config_path_str, &audit_path_str);
    assert!(agent.is_ok());
}

#[test]
fn test_config_serialization() {
    let test_dir = create_unique_test_dir("config");
    let config_path = test_dir.path().join("test_config");
    let config_path_str = config_path.to_string_lossy().into_owned();

    // Create and save config
    let config = AgentConfig::default();
    save_ron(format!("{}.ron", config_path_str), &config).unwrap();

    // Load config
    let loaded = load_config(&config_path_str).unwrap();

    assert_eq!(loaded.agent_name, config.agent_name);
    assert_eq!(loaded.execution_mode, config.execution_mode);
    assert_eq!(loaded.objectives.len(), config.objectives.len());
}

#[test]
fn test_state_save_and_load() {
    let test_dir = create_unique_test_dir("state");
    let config_path = test_dir.path().join("test_config");
    let audit_path = test_dir.path().join("test_audit.log");
    let config_path_str = config_path.to_string_lossy().into_owned();
    let audit_path_str = audit_path.to_string_lossy().into_owned();

    // Create config
    let config = AgentConfig::default();
    save_ron(format!("{}.ron", config_path_str), &config).unwrap();

    // Create agent and add some data
    let mut agent = AutonomousAgent::new(&config_path_str, &audit_path_str).unwrap();
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
    let mut agent2 = AutonomousAgent::new(&config_path_str, &audit_path_str).unwrap();
    agent2.load_state().unwrap();

    assert_eq!(agent2.lifecycle.memory.explored_files.len(), 1);
    assert_eq!(agent2.lifecycle.memory.decisions.len(), 1);
}

#[test]
fn test_symbolic_only_mode() {
    let test_dir = create_unique_test_dir("symbolic");
    let config_path = test_dir.path().join("test_config");
    let audit_path = test_dir.path().join("test_audit.log");
    let config_path_str = config_path.to_string_lossy().into_owned();
    let audit_path_str = audit_path.to_string_lossy().into_owned();

    // Create config with neural enabled
    let mut config = AgentConfig::default();
    config.neural.enabled = true;
    save_ron(format!("{}.ron", config_path_str), &config).unwrap();

    // Create agent
    let mut agent = AutonomousAgent::new(&config_path_str, &audit_path_str).unwrap();

    // Verify neural is initially enabled
    assert!(agent.lifecycle.neural.enabled);

    // Run in symbolic-only mode
    let result = agent.run_symbolic_only("test goal");

    // Should complete successfully
    assert!(result.is_ok());

    // Neural should be disabled
    assert!(!agent.lifecycle.neural.enabled);
}

#[test]
fn test_autonomous_iterations() {
    let test_dir = create_unique_test_dir("iterations");
    let config_path = test_dir.path().join("test_config");
    let audit_path = test_dir.path().join("test_audit.log");
    let config_path_str = config_path.to_string_lossy().into_owned();
    let audit_path_str = audit_path.to_string_lossy().into_owned();

    // Create config
    let config = AgentConfig::default();
    save_ron(format!("{}.ron", config_path_str), &config).unwrap();

    // Create and run agent
    let mut agent = AutonomousAgent::new(&config_path_str, &audit_path_str).unwrap();
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
    assert!(Path::new(&audit_path_str).exists());
}

#[test]
fn test_policy_enforcement() {
    let test_dir = create_unique_test_dir("policy");
    let config_path = test_dir.path().join("test_config");
    let audit_path = test_dir.path().join("test_audit.log");
    let config_path_str = config_path.to_string_lossy().into_owned();
    let audit_path_str = audit_path.to_string_lossy().into_owned();

    // Create config
    let config = AgentConfig::default();
    save_ron(format!("{}.ron", config_path_str), &config).unwrap();

    let agent = AutonomousAgent::new(&config_path_str, &audit_path_str).unwrap();

    // Test policy engine
    assert!(!agent.lifecycle.policy.validate_action("force-push"));
    assert!(!agent.lifecycle.policy.validate_action("force_push"));
    assert!(!agent.lifecycle.policy.validate_action("git push --force"));
    assert!(!agent.lifecycle.policy.validate_action("git push -f"));
    assert!(!agent.lifecycle.policy.validate_action("push -f"));
    assert!(!agent.lifecycle.policy.validate_action("FORCE-PUSH"));
    assert!(!agent.lifecycle.policy.validate_action("rm -rf /"));
    assert!(agent.lifecycle.policy.validate_action("git commit"));
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
