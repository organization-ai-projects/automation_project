// projects/products/unstable/autonomous_dev_ai/src/audit_event.rs
use serde::{Deserialize, Serialize};

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
    FinalState {
        state: String,
        iteration: usize,
        timestamp: u64,
    },
}
