//projects/libraries/ai/src/solver.rs
use neural::NeuralError;
use tracing::{info, warn};

use crate::{
    ai_error::AiError, ai_orchestrator::AiOrchestrator, solver_strategy::SolverStrategy,
    task::Task, task_result::TaskResult,
};

impl AiOrchestrator {
    pub(crate) fn solve(&mut self, task: &Task) -> Result<TaskResult, AiError> {
        info!(task=?task.task_type(), "Solving task");

        let strategy = self
            .dispatcher
            .decide_strategy(task, self.feedback.neural.is_some());
        info!(?strategy, "Strategy decided");

        self.solve_forced(task, strategy)
    }

    pub(crate) fn solve_forced(
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

    pub(crate) fn solve_symbolic(&mut self, task: &Task) -> Result<TaskResult, AiError> {
        let result =
            self.feedback
                .symbolic
                .solve(task.input(), task.task_type_str(), task.context())?;

        Ok(TaskResult::new(
            result.output,
            result.confidence,
            SolverStrategy::SymbolicOnly,
            result.metadata,
        ))
    }

    pub(crate) fn solve_neural(&mut self, task: &Task) -> Result<TaskResult, AiError> {
        let result = self.feedback.neural_mut()?.solve(task.input())?;

        Ok(TaskResult::new(
            result.output,
            result.confidence,
            SolverStrategy::NeuralOnly,
            result.metadata,
        ))
    }

    pub(crate) fn solve_symbolic_then_neural(
        &mut self,
        task: &Task,
    ) -> Result<TaskResult, AiError> {
        match self
            .feedback
            .symbolic
            .solve(task.input(), task.task_type_str(), task.context())
        {
            Ok(result) if result.confidence > 0.8 => {
                info!(
                    "Symbolic solved with high confidence: {:.2}",
                    result.confidence
                );
                return Ok(TaskResult::new(
                    result.output,
                    result.confidence,
                    SolverStrategy::SymbolicThenNeural,
                    result.metadata,
                ));
            }
            Ok(_) => info!("Symbolic confidence low, trying neural..."),
            Err(e) => info!("Symbolic failed: {}, trying neural...", e),
        }

        let result = self.feedback.neural_mut()?.solve(task.input())?;

        Ok(TaskResult::new(
            result.output,
            result.confidence,
            SolverStrategy::SymbolicThenNeural,
            result.metadata,
        ))
    }

    pub(crate) fn solve_neural_with_validation(
        &mut self,
        task: &Task,
    ) -> Result<TaskResult, AiError> {
        let neural = self.feedback.neural_mut()?;

        let neural_result = neural.solve(task.input())?;
        info!("Neural generated output, validating with symbolic...");

        let validation = self.feedback.symbolic.validate_code(&neural_result.output);

        match validation {
            Ok(valid) if valid.is_valid => {
                info!("Validation passed");
                Ok(TaskResult::new(
                    neural_result.output,
                    neural_result.confidence,
                    SolverStrategy::NeuralWithSymbolicValidation,
                    Some("Validated by symbolic".into()),
                ))
            }
            Ok(_) => {
                warn!("Validation failed, falling back to symbolic");
                let symbolic_result = self.solve_symbolic(task)?;
                Ok(TaskResult::new(
                    symbolic_result.output,
                    symbolic_result.confidence,
                    SolverStrategy::NeuralWithSymbolicValidation,
                    Some("Fallback to symbolic due to invalid neural output".into()),
                ))
            }
            Err(e) => {
                warn!(?e, "Validation error, falling back to symbolic");
                let symbolic_result = self.solve_symbolic(task)?;
                Ok(TaskResult::new(
                    symbolic_result.output,
                    symbolic_result.confidence,
                    SolverStrategy::NeuralWithSymbolicValidation,
                    Some("Fallback to symbolic due to validation error".into()),
                ))
            }
        }
    }

    pub(crate) fn solve_hybrid(&mut self, task: &Task) -> Result<TaskResult, AiError> {
        info!("Running hybrid strategy");

        let symbolic_result =
            self.feedback
                .symbolic
                .solve(task.input(), task.task_type_str(), task.context());

        let neural_result = match self.feedback.neural.as_mut() {
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
                    Ok(TaskResult::new(
                        sym.output,
                        sym.confidence,
                        SolverStrategy::Hybrid,
                        Some(format!("Symbolic (conf: {:.2})", sym.confidence)),
                    ))
                } else {
                    info!("Neural won: {:.2} vs {:.2}", neu.confidence, sym.confidence);
                    Ok(TaskResult::new(
                        neu.output,
                        neu.confidence,
                        SolverStrategy::Hybrid,
                        Some(format!("Neural (conf: {:.2})", neu.confidence)),
                    ))
                }
            }
            (Ok(sym), Err(e)) => {
                warn!(?e, "Neural failed, using symbolic");
                Ok(TaskResult::new(
                    sym.output,
                    sym.confidence,
                    SolverStrategy::Hybrid,
                    Some("Only symbolic available".into()),
                ))
            }
            (Err(e), Ok(neu)) => {
                warn!(?e, "Symbolic failed, using neural");
                Ok(TaskResult::new(
                    neu.output,
                    neu.confidence,
                    SolverStrategy::Hybrid,
                    Some("Only neural available".into()),
                ))
            }
            (Err(e1), Err(e2)) => Err(AiError::TaskError(format!(
                "Both failed. Symbolic: {}, Neural: {}",
                e1, e2
            ))),
        }
    }
}
