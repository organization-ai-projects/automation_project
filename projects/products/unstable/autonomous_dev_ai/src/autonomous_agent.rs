use crate::config_loader::load_config;
//projects/products/unstable/autonomous_dev_ai/src/autonomous_agent.rs
//Main agent implementation
use crate::error::AgentResult;
use crate::lifecycle::LifecycleManager;
use crate::persistence::{
    load_decision_inverted_index, load_failure_inverted_index, load_memory_state_index,
    load_memory_state_with_fallback, memory_transaction_completed, save_memory_state_transactional,
};

//Autonomous developer AI agent
pub struct AutonomousAgent {
    pub lifecycle: LifecycleManager,
    state_path: String,
}

impl AutonomousAgent {
    /// Create a new agent from config path
    pub fn new(config_path: &str, audit_log_path: &str) -> AgentResult<Self> {
        let config = load_config(config_path)?;
        let lifecycle = LifecycleManager::new(config, audit_log_path);

        let state_path = format!("{}_state", config_path);

        Ok(Self {
            lifecycle,
            state_path,
        })
    }

    /// Run the agent with a goal
    pub fn run(&mut self, goal: &str) -> AgentResult<()> {
        tracing::info!("Starting autonomous agent with goal: {}", goal);

        self.lifecycle.run(goal)?;

        tracing::info!("Agent completed successfully");
        Ok(())
    }

    /// Save agent state
    pub fn save_state(&self) -> AgentResult<()> {
        let index = save_memory_state_transactional(&self.state_path, &self.lifecycle.memory)?;
        tracing::info!(
            "State saved transactionally at base '{}' (max_iteration={}, decisions={}, failures={})",
            self.state_path,
            index.max_iteration_seen,
            index.decisions_count,
            index.failures_count
        );
        Ok(())
    }

    /// Load agent state
    pub fn load_state(&mut self) -> AgentResult<()> {
        if !memory_transaction_completed(&self.state_path)? {
            tracing::warn!(
                "Previous state transaction was not completed cleanly for base '{}'",
                self.state_path
            );
        }
        self.lifecycle.memory = load_memory_state_with_fallback(&self.state_path)?;
        if let Some(index) = load_memory_state_index(&self.state_path)? {
            self.lifecycle.memory.metadata.insert(
                "previous_state_max_iteration".to_string(),
                index.max_iteration_seen.to_string(),
            );
            self.lifecycle.memory.metadata.insert(
                "previous_state_failures_count".to_string(),
                index.failures_count.to_string(),
            );
            self.lifecycle.memory.metadata.insert(
                "previous_state_decisions_count".to_string(),
                index.decisions_count.to_string(),
            );
        }
        if let Some(failure_index) = load_failure_inverted_index(&self.state_path)? {
            if let Some((kind, count)) = failure_index.by_kind.iter().max_by_key(|(_, v)| *v) {
                self.lifecycle.memory.metadata.insert(
                    "previous_state_top_failure_kind".to_string(),
                    format!("{kind}:{count}"),
                );
            }
            if let Some((tool, count)) = failure_index.by_tool.iter().max_by_key(|(_, v)| *v) {
                self.lifecycle.memory.metadata.insert(
                    "previous_state_top_failure_tool".to_string(),
                    format!("{tool}:{count}"),
                );
            }
            if let Some(iteration) = failure_index.latest_failure_iteration {
                self.lifecycle.memory.metadata.insert(
                    "previous_state_latest_failure_iteration".to_string(),
                    iteration.to_string(),
                );
            }
        }
        if let Some(decision_index) = load_decision_inverted_index(&self.state_path)? {
            if let Some((action, count)) = decision_index.by_action.iter().max_by_key(|(_, v)| *v) {
                self.lifecycle.memory.metadata.insert(
                    "previous_state_top_decision_action".to_string(),
                    format!("{action}:{count}"),
                );
            }
            if let Some(iteration) = decision_index.latest_decision_iteration {
                self.lifecycle.memory.metadata.insert(
                    "previous_state_latest_decision_iteration".to_string(),
                    iteration.to_string(),
                );
            }
        }
        tracing::info!("State loaded from transactional base '{}'", self.state_path);
        Ok(())
    }

    /// Test symbolic-only mode (neural disabled)
    pub fn run_symbolic_only(&mut self, goal: &str) -> AgentResult<()> {
        tracing::info!("Running in symbolic-only mode (neural disabled)");

        // Disable neural
        self.lifecycle.neural.enabled = false;

        self.lifecycle.run(goal)?;

        tracing::info!("Symbolic-only mode completed successfully");
        Ok(())
    }
}
