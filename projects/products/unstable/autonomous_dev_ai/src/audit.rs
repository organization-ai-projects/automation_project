// projects/products/unstable/autonomous_dev_ai/src/audit.rs

//! Audit and traceability system

use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

/// Audit event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEvent {
    StateTransition {
        from: String,
        to: String,
        timestamp: u64,
    },
    ToolExecution {
        tool: String,
        args: Vec<String>,
        success: bool,
        timestamp: u64,
    },
    NeuralSuggestion {
        suggestion: String,
        confidence: f64,
        timestamp: u64,
    },
    SymbolicDecision {
        decision: String,
        reasoning: String,
        timestamp: u64,
    },
    FileModified {
        path: String,
        timestamp: u64,
    },
    ObjectiveEvaluation {
        iteration: usize,
        scores: Vec<(String, f64)>,
        timestamp: u64,
    },
}

/// Audit logger
pub struct AuditLogger {
    log_path: std::path::PathBuf,
}

impl AuditLogger {
    pub fn new<P: AsRef<Path>>(log_path: P) -> Self {
        Self {
            log_path: log_path.as_ref().to_path_buf(),
        }
    }

    /// Log an audit event
    pub fn log(&self, event: AuditEvent) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;

        let json = serde_json::to_string(&event)?;
        writeln!(file, "{}", json)?;
        file.flush()?;
        Ok(())
    }

    /// Get current timestamp
    fn timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// Log state transition
    pub fn log_state_transition(&self, from: &str, to: &str) -> std::io::Result<()> {
        self.log(AuditEvent::StateTransition {
            from: from.to_string(),
            to: to.to_string(),
            timestamp: Self::timestamp(),
        })
    }

    /// Log tool execution
    pub fn log_tool_execution(
        &self,
        tool: &str,
        args: &[String],
        success: bool,
    ) -> std::io::Result<()> {
        self.log(AuditEvent::ToolExecution {
            tool: tool.to_string(),
            args: args.to_vec(),
            success,
            timestamp: Self::timestamp(),
        })
    }

    /// Log neural suggestion
    pub fn log_neural_suggestion(&self, suggestion: &str, confidence: f64) -> std::io::Result<()> {
        self.log(AuditEvent::NeuralSuggestion {
            suggestion: suggestion.to_string(),
            confidence,
            timestamp: Self::timestamp(),
        })
    }

    /// Log symbolic decision
    pub fn log_symbolic_decision(&self, decision: &str, reasoning: &str) -> std::io::Result<()> {
        self.log(AuditEvent::SymbolicDecision {
            decision: decision.to_string(),
            reasoning: reasoning.to_string(),
            timestamp: Self::timestamp(),
        })
    }

    /// Log file modification
    pub fn log_file_modified(&self, path: &str) -> std::io::Result<()> {
        self.log(AuditEvent::FileModified {
            path: path.to_string(),
            timestamp: Self::timestamp(),
        })
    }

    /// Log objective evaluation
    pub fn log_objective_evaluation(
        &self,
        iteration: usize,
        scores: Vec<(String, f64)>,
    ) -> std::io::Result<()> {
        self.log(AuditEvent::ObjectiveEvaluation {
            iteration,
            scores,
            timestamp: Self::timestamp(),
        })
    }
}
