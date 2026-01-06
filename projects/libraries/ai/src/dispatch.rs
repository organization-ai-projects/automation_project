// projects/libraries/ai/src/dispatch.rs
use crate::SolverStrategy;
use crate::task::{Task, TaskType};

pub struct Dispatcher {
    // Statistiques pour améliorer les décisions
    symbolic_success_rate: f64,
    neural_success_rate: f64,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            symbolic_success_rate: 0.9, // Initial estimates
            neural_success_rate: 0.8,
        }
    }

    /// Décide quelle stratégie utiliser basé sur le type de tâche
    pub fn decide_strategy(&self, task: &Task, neural_available: bool) -> SolverStrategy {
        match task.task_type() {
            // Analyse statique = toujours symbolic
            TaskType::CodeAnalysis | TaskType::Linting | TaskType::Documentation => {
                SolverStrategy::SymbolicOnly
            }

            // Génération simple = symbolic d'abord
            TaskType::SimpleGeneration => {
                if neural_available {
                    SolverStrategy::SymbolicThenNeural
                } else {
                    SolverStrategy::SymbolicOnly
                }
            }

            // Génération complexe = neural avec validation
            TaskType::ComplexGeneration => {
                if neural_available {
                    SolverStrategy::NeuralWithSymbolicValidation
                } else {
                    SolverStrategy::SymbolicOnly
                }
            }

            // Refactoring = hybride (meilleur des deux)
            TaskType::Refactoring => {
                if neural_available {
                    SolverStrategy::Hybrid
                } else {
                    SolverStrategy::SymbolicOnly
                }
            }

            // Intent parsing = neural si disponible
            TaskType::IntentParsing => {
                if neural_available {
                    SolverStrategy::NeuralOnly
                } else {
                    SolverStrategy::SymbolicOnly
                }
            }
        }
    }

    /// Met à jour les statistiques de succès
    pub fn update_stats(&mut self, strategy: SolverStrategy, success: bool) {
        // Mise à jour avec moyenne mobile
        let alpha = 0.1; // Learning rate

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
