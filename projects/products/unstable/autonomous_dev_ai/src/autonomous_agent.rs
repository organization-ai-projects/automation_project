use crate::config_loader::load_config;
//projects/products/unstable/autonomous_dev_ai/src/autonomous_agent.rs
//Main agent implementation
use crate::error::AgentResult;
use crate::lifecycle::LifecycleManager;
use crate::persistence::{
    LearningSnapshot, append_learning_snapshot, load_action_outcome_index,
    load_decision_inverted_index, load_failure_inverted_index, load_memory_state_index,
    load_memory_state_with_fallback, load_recent_learning_snapshots, memory_transaction_completed,
    save_memory_state_transactional,
};
use crate::value_types::{ActionOutcomeSummary, ConfidenceScore, LearningWindow};

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
        let learning_window = learning_window_size();
        let learning_snapshot = append_learning_snapshot(
            &self.state_path,
            &self.lifecycle.memory,
            learning_window.get(),
        )?;
        tracing::info!(
            "State saved transactionally at base '{}' (max_iteration={}, decisions={}, failures={}, learning_top_failure_kind={:?})",
            self.state_path,
            index.max_iteration_seen,
            index.decisions_count,
            index.failures_count,
            learning_snapshot.top_failure_kind
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
        if let Some(action_outcome_index) = load_action_outcome_index(&self.state_path)? {
            if let Some((action, stats)) = select_worst_action_outcome(&action_outcome_index) {
                let summary = ActionOutcomeSummary {
                    action,
                    pass_rate: stats.pass_rate,
                    total: stats.total,
                };
                self.lifecycle.memory.metadata.insert(
                    "previous_state_worst_action_outcome".to_string(),
                    serde_json::to_string(&summary).unwrap_or_else(|_| {
                        format!(
                            "{}:{:.3}:{}",
                            summary.action,
                            summary.pass_rate.get(),
                            summary.total
                        )
                    }),
                );
            }
            if let Some((action, stats)) = select_best_action_outcome(&action_outcome_index) {
                let summary = ActionOutcomeSummary {
                    action,
                    pass_rate: stats.pass_rate,
                    total: stats.total,
                };
                self.lifecycle.memory.metadata.insert(
                    "previous_state_best_action_outcome".to_string(),
                    serde_json::to_string(&summary).unwrap_or_else(|_| {
                        format!(
                            "{}:{:.3}:{}",
                            summary.action,
                            summary.pass_rate.get(),
                            summary.total
                        )
                    }),
                );
            }
        }
        self.inject_recent_learning_metadata()?;
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

    fn inject_recent_learning_metadata(&mut self) -> AgentResult<()> {
        let learning_window = learning_window_size();
        let snapshots = load_recent_learning_snapshots(&self.state_path, learning_window.get())?;
        if snapshots.is_empty() {
            return Ok(());
        }

        self.lifecycle.memory.metadata.insert(
            "previous_recent_runs_count".to_string(),
            snapshots.len().to_string(),
        );
        let avg_failures = (snapshots.iter().map(|s| s.failures_count).sum::<usize>() as f64)
            / (snapshots.len() as f64);
        self.lifecycle.memory.metadata.insert(
            "previous_recent_avg_failures".to_string(),
            format!("{avg_failures:.2}"),
        );

        if let Some((value, confidence)) =
            dominant_recent_weighted(&snapshots, |s| s.top_failure_kind.clone())
        {
            self.lifecycle
                .memory
                .metadata
                .insert("previous_recent_top_failure_kind".to_string(), value);
            self.lifecycle.memory.metadata.insert(
                "previous_recent_top_failure_kind_confidence".to_string(),
                format!("{:.3}", confidence.get()),
            );
        }
        if let Some((value, confidence)) =
            dominant_recent_weighted(&snapshots, |s| s.top_decision_action.clone())
        {
            self.lifecycle
                .memory
                .metadata
                .insert("previous_recent_top_decision_action".to_string(), value);
            self.lifecycle.memory.metadata.insert(
                "previous_recent_top_decision_action_confidence".to_string(),
                format!("{:.3}", confidence.get()),
            );
        }
        Ok(())
    }
}

fn learning_window_size() -> LearningWindow {
    let raw = std::env::var("AUTONOMOUS_LEARNING_WINDOW")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(20);
    LearningWindow::new(raw)
        .unwrap_or_else(|| LearningWindow::new(20).expect("default window must be valid"))
}

fn dominant_recent_weighted<F>(
    snapshots: &[LearningSnapshot],
    pick: F,
) -> Option<(String, ConfidenceScore)>
where
    F: Fn(&LearningSnapshot) -> Option<String>,
{
    let decay = learning_recency_decay();
    let mut weights: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
    let mut total_weight = 0.0f64;
    for (idx, snapshot) in snapshots.iter().enumerate() {
        // More recent snapshots (higher idx) receive higher weight.
        let reverse_age = (snapshots.len() - 1 - idx) as i32;
        let weight = decay.powi(reverse_age);
        total_weight += weight;
        if let Some(key) = pick(snapshot) {
            *weights.entry(key).or_insert(0.0) += weight;
        }
    }

    weights
        .into_iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(key, score)| {
            let confidence_raw = if total_weight > 0.0 {
                score / total_weight
            } else {
                0.0
            };
            let confidence = ConfidenceScore::new(confidence_raw).unwrap_or_else(|| {
                ConfidenceScore::new(0.0).expect("0.0 must be a valid confidence")
            });
            (key, confidence)
        })
}

fn learning_recency_decay() -> f64 {
    std::env::var("AUTONOMOUS_LEARNING_RECENCY_DECAY")
        .ok()
        .and_then(|v| v.parse::<f64>().ok())
        .filter(|v| *v > 0.0 && *v <= 1.0)
        .unwrap_or(0.85)
}

fn select_worst_action_outcome(
    index: &crate::persistence::ActionOutcomeIndex,
) -> Option<(String, crate::persistence::ActionOutcomeStats)> {
    index
        .by_action
        .iter()
        .filter(|(_, stats)| stats.total >= 2)
        .min_by(|a, b| {
            a.1.pass_rate
                .get()
                .partial_cmp(&b.1.pass_rate.get())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(action, stats)| (action.clone(), stats.clone()))
}

fn select_best_action_outcome(
    index: &crate::persistence::ActionOutcomeIndex,
) -> Option<(String, crate::persistence::ActionOutcomeStats)> {
    index
        .by_action
        .iter()
        .filter(|(_, stats)| stats.total >= 2)
        .max_by(|a, b| {
            a.1.pass_rate
                .get()
                .partial_cmp(&b.1.pass_rate.get())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(action, stats)| (action.clone(), stats.clone()))
}
