//projects/libraries/layers/orchestration/ai/src/solver.rs
use neural::NeuralError;
use tracing::{info, warn};

use crate::{
    ai_error::AiError, ai_orchestrator::AiOrchestrator, solve_decision::SolveDecision,
    solve_trace::SolveTrace, solve_winner::SolveWinner, solver_strategy::SolverStrategy,
    task::Task, task_result::TaskResult,
};

impl AiOrchestrator {
    pub(crate) fn solve(&mut self, task: &Task) -> Result<TaskResult, AiError> {
        info!(task=?task.task_type(), "Solving task");

        let strategy = self
            .dispatcher
            .decide_strategy(task, self.feedback.has_neural());
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
                .symbolic()
                .solve(task.input(), task.task_type_str(), task.context())?;

        let mut trace = SolveTrace::new(SolverStrategy::SymbolicOnly);
        trace.symbolic_ran = true;
        trace.neural_ran = false;
        trace.winner = Some(SolveWinner::Symbolic);
        trace.symbolic_confidence = Some(result.confidence);
        trace.decision = SolveDecision::OnlyOneSolverAvailable;

        Ok(TaskResult::new(
            result.output,
            result.confidence,
            SolverStrategy::SymbolicOnly,
            result.metadata,
            trace,
        ))
    }

    pub(crate) fn solve_neural(&mut self, task: &Task) -> Result<TaskResult, AiError> {
        let result = self.feedback.neural_mut()?.solve(task.input())?;

        let mut trace = SolveTrace::new(SolverStrategy::NeuralOnly);
        trace.symbolic_ran = false;
        trace.neural_ran = true;
        trace.winner = Some(SolveWinner::Neural);
        trace.neural_confidence = Some(result.confidence);
        trace.decision = SolveDecision::OnlyOneSolverAvailable;

        Ok(TaskResult::new(
            result.output,
            result.confidence,
            SolverStrategy::NeuralOnly,
            result.metadata,
            trace,
        ))
    }

    pub(crate) fn solve_symbolic_then_neural(
        &mut self,
        task: &Task,
    ) -> Result<TaskResult, AiError> {
        let mut trace = SolveTrace::new(SolverStrategy::SymbolicThenNeural);
        trace.symbolic_ran = true;
        match self
            .feedback
            .symbolic()
            .solve(task.input(), task.task_type_str(), task.context())
        {
            Ok(result) if result.confidence > 0.8 => {
                info!(
                    "Symbolic solved with high confidence: {:.2}",
                    result.confidence
                );
                trace.winner = Some(SolveWinner::Symbolic);
                trace.symbolic_confidence = Some(result.confidence);
                trace.decision = SolveDecision::SymbolicHighConfidence;
                return Ok(TaskResult::new(
                    result.output,
                    result.confidence,
                    SolverStrategy::SymbolicThenNeural,
                    result.metadata,
                    trace,
                ));
            }
            Ok(result) => {
                trace.symbolic_confidence = Some(result.confidence);
                info!("Symbolic confidence low, trying neural...");
            }
            Err(e) => info!("Symbolic failed: {}, trying neural...", e),
        }

        let result = self.feedback.neural_mut()?.solve(task.input())?;
        trace.neural_ran = true;
        trace.winner = Some(SolveWinner::Neural);
        trace.neural_confidence = Some(result.confidence);
        trace.fallback_used = true;
        trace.decision = SolveDecision::SymbolicFailedOrLowConfidence;
        Ok(TaskResult::new(
            result.output,
            result.confidence,
            SolverStrategy::SymbolicThenNeural,
            result.metadata,
            trace,
        ))
    }

    pub(crate) fn solve_neural_with_validation(
        &mut self,
        task: &Task,
    ) -> Result<TaskResult, AiError> {
        let neural_result = {
            let neural = self.feedback.neural_mut()?;
            neural.solve(task.input())?
        };

        info!("Neural generated output, validating with symbolic...");

        let validation = self
            .feedback
            .symbolic()
            .validate_code(&neural_result.output);
        let mut trace = SolveTrace::new(SolverStrategy::NeuralWithSymbolicValidation);
        trace.neural_ran = true;
        trace.neural_confidence = Some(neural_result.confidence);
        trace.symbolic_validated_neural = true;

        match validation {
            Ok(valid) if valid.is_valid => {
                info!("Validation passed");
                trace.winner = Some(SolveWinner::Neural);
                trace.validation_passed = Some(true);
                trace.decision = SolveDecision::NeuralValidated;
                Ok(TaskResult::new(
                    neural_result.output,
                    neural_result.confidence,
                    SolverStrategy::NeuralWithSymbolicValidation,
                    Some("Validated by symbolic".into()),
                    trace,
                ))
            }
            Ok(_) => {
                warn!("Validation failed, falling back to symbolic");
                let symbolic_result = self.solve_symbolic(task)?;
                trace.symbolic_ran = true;
                trace.symbolic_confidence = Some(symbolic_result.confidence);
                trace.winner = Some(SolveWinner::Symbolic);
                trace.validation_passed = Some(false);
                trace.fallback_used = true;
                trace.decision = SolveDecision::NeuralInvalidFallbackToSymbolic;
                Ok(TaskResult::new(
                    symbolic_result.output,
                    symbolic_result.confidence,
                    SolverStrategy::NeuralWithSymbolicValidation,
                    Some("Fallback to symbolic due to invalid neural output".into()),
                    trace,
                ))
            }
            Err(e) => {
                warn!(?e, "Validation error, falling back to symbolic");
                let symbolic_result = self.solve_symbolic(task)?;
                trace.symbolic_ran = true;
                trace.symbolic_confidence = Some(symbolic_result.confidence);
                trace.winner = Some(SolveWinner::Symbolic);
                trace.fallback_used = true;
                trace.decision = SolveDecision::NeuralInvalidFallbackToSymbolic;
                Ok(TaskResult::new(
                    symbolic_result.output,
                    symbolic_result.confidence,
                    SolverStrategy::NeuralWithSymbolicValidation,
                    Some("Fallback to symbolic due to validation error".into()),
                    trace,
                ))
            }
        }
    }

    pub(crate) fn solve_hybrid(&mut self, task: &Task) -> Result<TaskResult, AiError> {
        info!("Running hybrid strategy");

        let symbolic_result = {
            self.feedback
                .symbolic()
                .solve(task.input(), task.task_type_str(), task.context())
        };

        let neural_result = match self.feedback.neural_mut() {
            Ok(neural) => neural.solve(task.input()),
            Err(_) => Err(NeuralError::ModelNotLoaded),
        };

        match (symbolic_result, neural_result) {
            (Ok(sym), Ok(neu)) => {
                let mut trace = SolveTrace::new(SolverStrategy::Hybrid);
                trace.symbolic_ran = true;
                trace.neural_ran = true;
                trace.symbolic_confidence = Some(sym.confidence);
                trace.neural_confidence = Some(neu.confidence);
                trace.decision = SolveDecision::HybridBestOfBoth;
                if sym.confidence >= neu.confidence {
                    info!(
                        "Symbolic won: {:.2} vs {:.2}",
                        sym.confidence, neu.confidence
                    );
                    trace.winner = Some(SolveWinner::Symbolic);
                    Ok(TaskResult::new(
                        sym.output,
                        sym.confidence,
                        SolverStrategy::Hybrid,
                        Some(format!("Symbolic (conf: {:.2})", sym.confidence)),
                        trace,
                    ))
                } else {
                    info!("Neural won: {:.2} vs {:.2}", neu.confidence, sym.confidence);
                    trace.winner = Some(SolveWinner::Neural);
                    Ok(TaskResult::new(
                        neu.output,
                        neu.confidence,
                        SolverStrategy::Hybrid,
                        Some(format!("Neural (conf: {:.2})", neu.confidence)),
                        trace,
                    ))
                }
            }
            (Ok(sym), Err(e)) => {
                warn!(?e, "Neural failed, using symbolic");
                let mut trace = SolveTrace::new(SolverStrategy::Hybrid);
                trace.symbolic_ran = true;
                trace.symbolic_confidence = Some(sym.confidence);
                trace.fallback_used = true;
                trace.winner = Some(SolveWinner::Symbolic);
                trace.decision = SolveDecision::OnlyOneSolverAvailable;
                Ok(TaskResult::new(
                    sym.output,
                    sym.confidence,
                    SolverStrategy::Hybrid,
                    Some("Only symbolic available".into()),
                    trace,
                ))
            }
            (Err(e), Ok(neu)) => {
                warn!(?e, "Symbolic failed, using neural");
                let mut trace = SolveTrace::new(SolverStrategy::Hybrid);
                trace.neural_ran = true;
                trace.neural_confidence = Some(neu.confidence);
                trace.fallback_used = true;
                trace.winner = Some(SolveWinner::Neural);
                trace.decision = SolveDecision::OnlyOneSolverAvailable;
                Ok(TaskResult::new(
                    neu.output,
                    neu.confidence,
                    SolverStrategy::Hybrid,
                    Some("Only neural available".into()),
                    trace,
                ))
            }
            (Err(e1), Err(e2)) => Err(AiError::TaskError(format!(
                "Both failed. Symbolic: {}, Neural: {}",
                e1, e2
            ))),
        }
    }
}
