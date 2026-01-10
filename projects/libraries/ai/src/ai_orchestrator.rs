// projects/libraries/ai/src/ai_orchestrator.rs
use neural::NeuralError;
use neural::feedback::FeedbackType;
use neural::{NeuralSolver, feedback::UserFeedback};
use symbolic::{feedback_symbolic::SymbolicFeedback, symbolic_solver::SymbolicSolver};
use tracing::{info, warn};

use crate::{
    ai_error::AiError, dispatcher::Dispatcher, solver_strategy::SolverStrategy, task::Task,
    task_result::TaskResult,
};

pub struct AiOrchestrator {
    symbolic: SymbolicSolver,
    neural: Option<NeuralSolver>,
    dispatcher: Dispatcher,
}

impl AiOrchestrator {
    pub fn new() -> Result<Self, AiError> {
        info!("Initializing AI orchestrator...");

        Ok(Self {
            symbolic: SymbolicSolver::new()?,
            neural: None,
            dispatcher: Dispatcher::new(),
        })
    }

    pub fn load_neural_model(
        &mut self,
        model_path: &std::path::Path,
        tokenizer_path: &std::path::Path,
    ) -> Result<(), AiError> {
        info!("Loading neural model...");
        self.neural = Some(NeuralSolver::load(model_path, tokenizer_path)?);
        info!("Neural model loaded successfully");
        Ok(())
    }

    pub fn solve(&mut self, task: &Task) -> Result<TaskResult, AiError> {
        info!(task=?task.task_type(), "Solving task");

        let strategy = self.dispatcher.decide_strategy(task, self.neural.is_some());
        info!(?strategy, "Strategy decided");

        self.solve_forced(task, strategy)
    }

    fn solve_symbolic(&mut self, task: &Task) -> Result<TaskResult, AiError> {
        let result = self
            .symbolic
            .solve(task.input(), task.task_type_str(), task.context())?;

        Ok(TaskResult {
            output: result.output,
            confidence: result.confidence,
            strategy_used: SolverStrategy::SymbolicOnly,
            metadata: result.metadata,
        })
    }

    fn solve_neural(&mut self, task: &Task) -> Result<TaskResult, AiError> {
        let result = self.neural_mut()?.solve(task.input())?;

        Ok(TaskResult {
            output: result.output,
            confidence: result.confidence,
            strategy_used: SolverStrategy::NeuralOnly,
            metadata: result.metadata,
        })
    }

    fn solve_symbolic_then_neural(&mut self, task: &Task) -> Result<TaskResult, AiError> {
        match self
            .symbolic
            .solve(task.input(), task.task_type_str(), task.context())
        {
            Ok(result) if result.confidence > 0.8 => {
                info!(
                    "Symbolic solved with high confidence: {:.2}",
                    result.confidence
                );
                return Ok(TaskResult {
                    output: result.output,
                    confidence: result.confidence,
                    strategy_used: SolverStrategy::SymbolicThenNeural,
                    metadata: result.metadata,
                });
            }
            Ok(_) => info!("Symbolic confidence low, trying neural..."),
            Err(e) => info!("Symbolic failed: {}, trying neural...", e),
        }

        let result = self.neural_mut()?.solve(task.input())?;

        Ok(TaskResult {
            output: result.output,
            confidence: result.confidence,
            strategy_used: SolverStrategy::SymbolicThenNeural,
            metadata: result.metadata,
        })
    }

    fn solve_neural_with_validation(&mut self, task: &Task) -> Result<TaskResult, AiError> {
        let neural = self.neural_mut()?;

        let neural_result = neural.solve(task.input())?;
        info!("Neural generated output, validating with symbolic...");

        let validation = self.symbolic.validate_code(&neural_result.output)?;

        if !validation.is_valid {
            warn!("Validation failed: {}", validation.errors.join(", "));
            return Err(AiError::TaskError(format!(
                "Validation failed: {}",
                validation.errors.join(", ")
            )));
        }

        info!("Validation passed");

        Ok(TaskResult {
            output: neural_result.output,
            confidence: neural_result.confidence,
            strategy_used: SolverStrategy::NeuralWithSymbolicValidation,
            metadata: Some("Validated by symbolic".into()),
        })
    }

    fn neural_mut(&mut self) -> Result<&mut NeuralSolver, AiError> {
        self.neural
            .as_mut()
            .ok_or_else(|| AiError::TaskError("Neural model not loaded".into()))
    }

    fn solve_hybrid(&mut self, task: &Task) -> Result<TaskResult, AiError> {
        info!("Running hybrid strategy");

        let symbolic_result =
            self.symbolic
                .solve(task.input(), task.task_type_str(), task.context());

        let neural_result = match self.neural.as_mut() {
            Some(n) => n.solve(task.input()),
            None => Err(NeuralError::ModelNotLoaded),
        };

        match (symbolic_result, neural_result) {
            (Ok(sym), Ok(neu)) => {
                if sym.confidence >= neu.confidence {
                    info!(
                        "Symbolic won: {:.2} vs {:.2}",
                        sym.confidence, neu.confidence
                    );
                    Ok(TaskResult {
                        output: sym.output,
                        confidence: sym.confidence,
                        strategy_used: SolverStrategy::Hybrid,
                        metadata: Some(format!("Symbolic (conf: {:.2})", sym.confidence)),
                    })
                } else {
                    info!("Neural won: {:.2} vs {:.2}", neu.confidence, sym.confidence);
                    Ok(TaskResult {
                        output: neu.output,
                        confidence: neu.confidence,
                        strategy_used: SolverStrategy::Hybrid,
                        metadata: Some(format!("Neural (conf: {:.2})", neu.confidence)),
                    })
                }
            }
            (Ok(sym), Err(_)) => Ok(TaskResult {
                output: sym.output,
                confidence: sym.confidence,
                strategy_used: SolverStrategy::Hybrid,
                metadata: Some("Only symbolic available".into()),
            }),
            (Err(_), Ok(neu)) => Ok(TaskResult {
                output: neu.output,
                confidence: neu.confidence,
                strategy_used: SolverStrategy::Hybrid,
                metadata: Some("Only neural available".into()),
            }),
            (Err(e1), Err(e2)) => Err(AiError::TaskError(format!(
                "Both failed. Symbolic: {}, Neural: {}",
                e1, e2
            ))),
        }
    }

    pub fn train_neural(&mut self, training_data: Vec<String>) -> Result<(), AiError> {
        self.neural_mut()?.train(training_data)?;
        Ok(())
    }

    pub fn adjust_with_feedback(&mut self, feedback: &UserFeedback) -> Result<(), AiError> {
        self.neural_mut()?.record_feedback(feedback)?;
        Ok(())
    }

    pub fn train_with_feedback(
        &mut self,
        task: &Task,
        feedback: &UserFeedback,
    ) -> Result<(), AiError> {
        info!(
            "Entraînement avec retour utilisateur pour la tâche : {:?}",
            task.task_type()
        );

        // Entraîner le modèle neuronal si disponible
        info!("Mise à jour du modèle neuronal avec le retour utilisateur...");
        self.neural_mut()?.record_feedback(feedback)?;

        // Ajuster les règles symboliques si nécessaire
        info!("Mise à jour des règles symboliques...");
        let symbolic_feedback = SymbolicFeedback {
            is_positive: matches!(feedback.feedback_type, FeedbackType::Correct),
            metadata: Some(format!(
                "Input: {}, Output: {}",
                feedback.input, feedback.generated_output
            )),
        };
        self.symbolic
            .adjust_rules(task.input(), symbolic_feedback)?;

        Ok(())
    }

    // API simplifiée
    pub fn generate_code(&mut self, prompt: &str) -> Result<String, AiError> {
        let task = Task::new_code_generation(prompt.to_string());
        let result = self.solve(&task)?;
        Ok(result.output)
    }

    pub fn analyze_code(&mut self, code: &str) -> Result<String, AiError> {
        let task = Task::new_code_analysis(code.to_string());
        let result = self.solve(&task)?;
        Ok(result.output)
    }

    pub fn refactor_code(&mut self, code: &str, instruction: &str) -> Result<String, AiError> {
        let task = Task::new_refactoring(code.to_string(), instruction.to_string());
        let result = self.solve(&task)?;
        Ok(result.output)
    }

    pub fn solve_forced(
        &mut self,
        task: &Task,
        strategy: SolverStrategy,
    ) -> Result<TaskResult, AiError> {
        match strategy {
            SolverStrategy::SymbolicOnly => self.solve_symbolic(task),
            SolverStrategy::NeuralOnly => self.solve_neural(task),
            SolverStrategy::SymbolicThenNeural => self.solve_symbolic_then_neural(task),
            SolverStrategy::NeuralWithSymbolicValidation => self.solve_neural_with_validation(task),
            SolverStrategy::Hybrid => self.solve_hybrid(task),
        }
    }
}
