use neural::{NeuralSolver, feedback};
use symbolic::symbolic_solver::SymbolicSolver;

use crate::{ai_error::AiError, dispatcher::Dispatcher, solver_strategy::SolverStrategy, task::Task, task_result::TaskResult};

pub struct AiOrchestrator {
    symbolic: SymbolicSolver,
    neural: Option<NeuralSolver>,
    dispatcher: Dispatcher,
}

impl AiOrchestrator {
    pub fn new() -> Result<Self, AiError> {
        println!("Initializing AI orchestrator...");

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
        println!("Loading neural model...");
        self.neural = Some(NeuralSolver::load(model_path, tokenizer_path)?);
        println!("Neural model loaded successfully");
        Ok(())
    }

    pub fn solve(&mut self, task: Task) -> Result<TaskResult, AiError> {
        println!("Solving task: {:?}", task.task_type());

        let strategy = self
            .dispatcher
            .decide_strategy(&task, self.neural.is_some());
        println!("Strategy: {:?}", strategy);

        match strategy {
            SolverStrategy::SymbolicOnly => self.solve_symbolic(&task),
            SolverStrategy::NeuralOnly => self.solve_neural(&task),
            SolverStrategy::SymbolicThenNeural => self.solve_symbolic_then_neural(&task),
            SolverStrategy::NeuralWithSymbolicValidation => {
                self.solve_neural_with_validation(&task)
            }
            SolverStrategy::Hybrid => self.solve_hybrid(&task),
        }
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
        let neural = self
            .neural
            .as_mut()
            .ok_or_else(|| AiError::TaskError("Neural model not loaded".into()))?;

        let result = neural.solve(task.input())?;

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
                println!(
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
            Ok(_) => println!("Symbolic confidence low, trying neural..."),
            Err(e) => println!("Symbolic failed: {}, trying neural...", e),
        }

        let neural = self
            .neural
            .as_mut()
            .ok_or_else(|| AiError::TaskError("Neural model not loaded".into()))?;

        let result = neural.solve(task.input())?;

        Ok(TaskResult {
            output: result.output,
            confidence: result.confidence,
            strategy_used: SolverStrategy::SymbolicThenNeural,
            metadata: result.metadata,
        })
    }

    fn solve_neural_with_validation(&mut self, task: &Task) -> Result<TaskResult, AiError> {
        let neural = self
            .neural
            .as_mut()
            .ok_or_else(|| AiError::TaskError("Neural model not loaded".into()))?;

        let neural_result = neural.solve(task.input())?;
        println!("Neural generated output, validating with symbolic...");

        let validation = self.symbolic.validate_code(&neural_result.output)?;

        if !validation.is_valid {
            println!("Validation failed: {}", validation.errors.join(", "));

            if !validation.errors.is_empty() {
                println!("Applying symbolic correction");
                return Ok(TaskResult {
                    output: validation.errors.join("; "),
                    confidence: 0.7,
                    strategy_used: SolverStrategy::NeuralWithSymbolicValidation,
                    metadata: Some(format!("Corrected: {}", validation.errors.join(", "))),
                });
            }

            return Err(AiError::TaskError(format!(
                "Invalid output: {}",
                validation.errors.join(", ")
            )));
        }

        println!("Validation passed");

        Ok(TaskResult {
            output: neural_result.output,
            confidence: neural_result.confidence,
            strategy_used: SolverStrategy::NeuralWithSymbolicValidation,
            metadata: Some("Validated by symbolic".into()),
        })
    }

    fn solve_hybrid(&mut self, task: &Task) -> Result<TaskResult, AiError> {
        println!("Running hybrid strategy");

        let symbolic_result =
            self.symbolic
                .solve(task.input(), task.task_type_str(), task.context());

        let neural_result = if let Some(neural) = self.neural.as_mut() {
            neural.solve(task.input())
        } else {
            Err(neural::NeuralError::ModelNotLoaded)
        };

        match (symbolic_result, neural_result) {
            (Ok(sym), Ok(neu)) => {
                if sym.confidence >= neu.confidence {
                    println!(
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
                    println!("Neural won: {:.2} vs {:.2}", neu.confidence, sym.confidence);
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
        let neural = self
            .neural
            .as_mut()
            .ok_or_else(|| AiError::TaskError("Neural model not loaded".into()))?;

        neural.train(training_data)?;
        Ok(())
    }

    pub fn adjust_with_feedback(
        &mut self,
        feedback: feedback::UserFeedback,
    ) -> Result<(), AiError> {
        let neural = self
            .neural
            .as_mut()
            .ok_or_else(|| AiError::TaskError("Neural model not loaded".into()))?;

        neural.record_feedback(feedback)?;
        Ok(())
    }

    // API simplifiée
    pub fn generate_code(&mut self, prompt: &str) -> Result<String, AiError> {
        let task = Task::new_code_generation(prompt.to_string());
        let result = self.solve(task)?;
        Ok(result.output)
    }

    pub fn analyze_code(&mut self, code: &str) -> Result<String, AiError> {
        let task = Task::new_code_analysis(code.to_string());
        let result = self.solve(task)?;
        Ok(result.output)
    }

    pub fn refactor_code(&mut self, code: &str, instruction: &str) -> Result<String, AiError> {
        let task = Task::new_refactoring(code.to_string(), instruction.to_string());
        let result = self.solve(task)?;
        Ok(result.output)
    }
}
