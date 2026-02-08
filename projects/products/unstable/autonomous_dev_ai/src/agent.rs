// projects/products/unstable/autonomous_dev_ai/src/agent.rs

//! Main agent implementation

use crate::config::load_config;
use crate::error::{AgentError, AgentResult};
use crate::lifecycle::LifecycleManager;
use std::path::Path;

/// Autonomous developer AI agent
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
        // Save memory graph as .ron and .bin
        let ron_path = format!("{}.ron", self.state_path);
        let bin_path = format!("{}.bin", self.state_path);

        // Save as RON
        let ron_content =
            ron::ser::to_string_pretty(&self.lifecycle.memory, ron::ser::PrettyConfig::default())
                .map_err(|e| AgentError::Ron(e.to_string()))?;
        std::fs::write(&ron_path, ron_content)?;

        // Save as bincode
        let bin_content =
            bincode::encode_to_vec(&self.lifecycle.memory, bincode::config::standard())
                .map_err(|e| AgentError::Bincode(format!("{:?}", e)))?;
        std::fs::write(&bin_path, bin_content)?;

        tracing::info!("State saved to {} and {}", ron_path, bin_path);
        Ok(())
    }

    /// Load agent state
    pub fn load_state(&mut self) -> AgentResult<()> {
        let bin_path = format!("{}.bin", self.state_path);
        let ron_path = format!("{}.ron", self.state_path);

        // Try binary first
        if Path::new(&bin_path).exists() {
            let bytes = std::fs::read(&bin_path)?;
            let (memory, _) = bincode::decode_from_slice(&bytes, bincode::config::standard())
                .map_err(|e| AgentError::Bincode(format!("{:?}", e)))?;
            self.lifecycle.memory = memory;
            tracing::info!("State loaded from {}", bin_path);
            return Ok(());
        }

        // Fall back to RON
        if Path::new(&ron_path).exists() {
            let content = std::fs::read_to_string(&ron_path)?;
            self.lifecycle.memory =
                ron::from_str(&content).map_err(|e| AgentError::Ron(e.to_string()))?;
            tracing::info!("State loaded from {}", ron_path);

            // Rebuild binary
            let bin_content =
                bincode::encode_to_vec(&self.lifecycle.memory, bincode::config::standard())
                    .map_err(|e| AgentError::Bincode(format!("{:?}", e)))?;
            std::fs::write(&bin_path, bin_content)?;
            tracing::info!("Binary state rebuilt at {}", bin_path);

            return Ok(());
        }

        Err(AgentError::State("No saved state found".to_string()))
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
