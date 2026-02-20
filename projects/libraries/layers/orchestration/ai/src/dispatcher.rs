// projects/libraries/layers/orchestration/ai/src/dispatch.rs
use crate::SolverStrategy;
use crate::task::Task;
use crate::task_type::TaskType;

#[allow(dead_code)]
pub struct Dispatcher {
    // Statistics to improve decision-making
    symbolic_success_rate: f64,
    neural_success_rate: f64,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            symbolic_success_rate: 0.9,
            neural_success_rate: 0.8,
        }
    }

    /// Decides which strategy to use based on the task type
    pub fn decide_strategy(&self, task: &Task, neural_available: bool) -> SolverStrategy {
        match task.task_type() {
            // Static analysis = always symbolic
            TaskType::CodeAnalysis | TaskType::Linting | TaskType::Documentation => {
                SolverStrategy::SymbolicOnly
            }

            // Simple generation = symbolic first
            TaskType::SimpleGeneration => {
                if neural_available {
                    SolverStrategy::SymbolicThenNeural
                } else {
                    SolverStrategy::SymbolicOnly
                }
            }

            // Complex generation = neural with validation
            TaskType::ComplexGeneration => {
                if neural_available {
                    SolverStrategy::NeuralWithSymbolicValidation
                } else {
                    SolverStrategy::SymbolicOnly
                }
            }

            // Refactoring = hybrid (best of both)
            TaskType::Refactoring => {
                if neural_available {
                    SolverStrategy::Hybrid
                } else {
                    SolverStrategy::SymbolicOnly
                }
            }

            // Intent parsing = neural if available
            TaskType::IntentParsing => {
                if neural_available {
                    SolverStrategy::NeuralOnly
                } else {
                    SolverStrategy::SymbolicOnly
                }
            }
        }
    }

    /// Updates success statistics
    #[allow(dead_code)]
    pub fn update_stats(&mut self, strategy: SolverStrategy, success: bool) {
        // Update using moving average
        let alpha = 0.1; // Adjustment factor for moving average

        match strategy {
            SolverStrategy::SymbolicOnly | SolverStrategy::SymbolicThenNeural => {
                if success {
                    self.symbolic_success_rate = self.symbolic_success_rate * (1.0 - alpha) + alpha;
                } else {
                    self.symbolic_success_rate *= 1.0 - alpha;
                }
            }
            SolverStrategy::NeuralOnly | SolverStrategy::NeuralWithSymbolicValidation => {
                if success {
                    self.neural_success_rate = self.neural_success_rate * (1.0 - alpha) + alpha;
                } else {
                    self.neural_success_rate *= 1.0 - alpha;
                }
            }
            _ => {}
        }
    }
}

impl Default for Dispatcher {
    fn default() -> Self {
        Self::new()
    }
}
