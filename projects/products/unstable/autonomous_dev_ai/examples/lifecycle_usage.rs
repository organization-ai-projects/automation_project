//! Example usage of the production-grade lifecycle manager.

use autonomous_dev_ai::config::{AgentConfig, NeuralConfig, SymbolicConfig};
use autonomous_dev_ai::lifecycle::LifecycleManager;
use autonomous_dev_ai::objectives::Objective;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = AgentConfig {
        agent_name: "autonomous_dev_ai".to_string(),
        execution_mode: "ci_bound".to_string(),
        max_iterations: 10,
        timeout_seconds: Some(3600),
        objectives: vec![
            Objective::new("task_completion".to_string(), 1.0, true).with_threshold(0.9),
            Objective::new("policy_safety".to_string(), 1.0, true).with_threshold(1.0),
            Objective::new("tests_pass".to_string(), 0.8, true).with_threshold(1.0),
            Objective::new("minimal_diff".to_string(), 0.5, false).with_threshold(0.7),
        ],
        symbolic: SymbolicConfig {
            strict_validation: true,
            deterministic: true,
        },
        neural: NeuralConfig {
            enabled: true,
            prefer_gpu: true,
            cpu_fallback: true,
            models: HashMap::new(),
        },
    };

    let mut manager = LifecycleManager::new(config, "audit.log");
    let goal = "Implement user authentication with JWT tokens. Related to PR #1234";

    match manager.run(goal) {
        Ok(()) => {
            println!("Agent completed successfully");
            let metrics = manager.metrics();
            println!("Total duration: {:?}", metrics.total_duration);
            println!("Iterations: {}", metrics.iterations_total);
            println!("Tool executions: {}", metrics.tool_executions_total);
        }
        Err(e) => {
            eprintln!("Agent failed: {}", e);
            let metrics = manager.metrics();
            eprintln!("Duration: {:?}", metrics.total_duration);
            eprintln!("Iterations: {}", metrics.iterations_total);
            return Err(Box::<dyn std::error::Error>::from(e));
        }
    }

    Ok(())
}
