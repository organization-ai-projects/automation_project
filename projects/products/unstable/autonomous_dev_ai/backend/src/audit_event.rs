// projects/products/unstable/autonomous_dev_ai/src/audit_event.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum AuditEvent {
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

impl AuditEvent {
    pub(crate) fn new(
        event_type: &str,
        args: Vec<String>,
        success: bool,
        timestamp: u64,
    ) -> Option<Self> {
        match event_type {
            "StateTransition" => {
                if args.len() == 2 {
                    Some(AuditEvent::StateTransition {
                        from: args[0].clone(),
                        to: args[1].clone(),
                        timestamp,
                    })
                } else {
                    None
                }
            }
            "ToolExecution" => {
                if !args.is_empty() {
                    Some(AuditEvent::ToolExecution {
                        tool: args[0].clone(),
                        args: args[1..].to_vec(),
                        success,
                        timestamp,
                    })
                } else {
                    None
                }
            }
            "NeuralSuggestion" => {
                if args.len() == 2 {
                    args[1]
                        .parse::<f64>()
                        .ok()
                        .map(|confidence| AuditEvent::NeuralSuggestion {
                            suggestion: args[0].clone(),
                            confidence,
                            timestamp,
                        })
                } else {
                    None
                }
            }
            "SymbolicDecision" => {
                if args.len() == 2 {
                    Some(AuditEvent::SymbolicDecision {
                        decision: args[0].clone(),
                        reasoning: args[1].clone(),
                        timestamp,
                    })
                } else {
                    None
                }
            }
            "FileModified" => {
                if args.len() == 1 {
                    Some(AuditEvent::FileModified {
                        path: args[0].clone(),
                        timestamp,
                    })
                } else {
                    None
                }
            }
            "ObjectiveEvaluation" => {
                if args.len() >= 2 {
                    let iteration = args[0].parse::<usize>().ok()?;
                    let scores = args[1..]
                        .chunks(2)
                        .filter_map(|chunk| match chunk.len() {
                            2 => chunk[1]
                                .parse::<f64>()
                                .ok()
                                .map(|score| (chunk[0].clone(), score)),
                            _ => None,
                        })
                        .collect();
                    Some(AuditEvent::ObjectiveEvaluation {
                        iteration,
                        scores,
                        timestamp,
                    })
                } else {
                    None
                }
            }
            "FinalState" => {
                if args.len() == 2 {
                    let iteration = args[1].parse::<usize>().ok()?;
                    Some(AuditEvent::FinalState {
                        state: args[0].clone(),
                        iteration,
                        timestamp,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
