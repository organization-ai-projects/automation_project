//projects/products/unstable/autonomous_dev_ai/src/lifecycle/lifecycle_manager.rs
// Agent lifecycle implementation - production-grade flow.
use super::{
    CircuitBreaker, ExecutionContext, IterationNumber, LifecycleError, LifecycleMetrics,
    LifecycleResult, MaxIterations, MetricsCollector, ResourceType, RetryStrategy, StepIndex,
    validation_strategy::select_validation_command,
};

use crate::agent_config::AgentConfig;
use crate::audit_logger::AuditLogger;
use crate::error::{AgentError, AgentResult};
use crate::memory_graph::MemoryGraph;
use crate::neural::NeuralLayer;
use crate::objective_evaluator::ObjectiveEvaluator;
use crate::security::{ActorIdentity, AuthzDecision, AuthzEngine};
use crate::state::AgentState;
use crate::symbolic::{Plan, PlanStep, PolicyEngine, SymbolicController};
use crate::tools::{
    GitWrapper, PrDescriptionGenerator, RepoReader, TestRunner, ToolRegistry, ToolResult,
};

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Agent lifecycle manager.
pub struct LifecycleManager {
    // Public fields preserved for compatibility with existing callers/tests.
    pub state: AgentState,
    pub config: AgentConfig,
    pub memory: MemoryGraph,
    pub symbolic: SymbolicController,
    pub neural: NeuralLayer,
    pub policy: PolicyEngine,
    pub authz: AuthzEngine,
    pub actor: ActorIdentity,
    pub tools: ToolRegistry,
    pub audit: AuditLogger,
    pub iteration: usize,
    pub current_plan: Option<Plan>,
    pub current_step: usize,
    pub max_iterations: usize,

    // Typed execution state.
    current_iteration_number: IterationNumber,
    max_iterations_limit: MaxIterations,
    current_step_index: StepIndex,
    execution_context: Option<ExecutionContext>,

    // Resilience and observability.
    circuit_breakers: HashMap<String, CircuitBreaker>,
    retry_strategy: RetryStrategy,
    metrics: MetricsCollector,

    // Timeouts.
    global_timeout: Duration,
    iteration_timeout: Duration,
    tool_timeout: Duration,
}

impl LifecycleManager {
    pub(crate) fn extract_pr_number_from_goal(goal: &str) -> Option<String> {
        let bytes = goal.as_bytes();
        let mut i = 0usize;

        while i < bytes.len() {
            if bytes[i] == b'#' {
                let start = i + 1;
                let mut end = start;
                while end < bytes.len() && bytes[end].is_ascii_digit() {
                    end += 1;
                }
                if end > start {
                    return Some(goal[start..end].to_string());
                }
            }
            i += 1;
        }
        None
    }

    pub fn new(config: AgentConfig, audit_log_path: &str) -> Self {
        let max_iterations_limit =
            MaxIterations::new(config.max_iterations).unwrap_or_else(MaxIterations::default_value);

        let objectives = config.objectives.clone();
        let evaluator = ObjectiveEvaluator::new(objectives);

        let symbolic = SymbolicController::new(
            evaluator,
            config.symbolic.strict_validation,
            config.symbolic.deterministic,
        );

        let neural = NeuralLayer::new(
            config.neural.enabled,
            config.neural.prefer_gpu,
            config.neural.cpu_fallback,
        );

        let policy = PolicyEngine::new();
        let authz = AuthzEngine::new();
        let actor = ActorIdentity::default();

        let mut tools = ToolRegistry::new();
        tools.register(Box::new(RepoReader));
        tools.register(Box::new(TestRunner));
        tools.register(Box::new(GitWrapper));
        tools.register(Box::new(PrDescriptionGenerator));

        let global_timeout = Duration::from_secs(config.timeout_seconds.unwrap_or(3600));

        Self {
            state: AgentState::Idle,
            config,
            memory: MemoryGraph::new(),
            symbolic,
            neural,
            policy,
            authz,
            actor,
            tools,
            audit: AuditLogger::new(audit_log_path),
            iteration: IterationNumber::first().get(),
            current_plan: None,
            current_step: StepIndex::zero().get(),
            max_iterations: max_iterations_limit.get(),
            current_iteration_number: IterationNumber::first(),
            max_iterations_limit,
            current_step_index: StepIndex::zero(),
            execution_context: None,
            circuit_breakers: HashMap::new(),
            retry_strategy: RetryStrategy::default(),
            metrics: MetricsCollector::new(),
            global_timeout,
            iteration_timeout: Duration::from_secs(300),
            tool_timeout: Duration::from_secs(30),
        }
    }

    /// Run the lifecycle with typed errors, retries, and metrics.
    pub fn run(&mut self, goal: &str) -> LifecycleResult<()> {
        let start_time = Instant::now();

        self.current_iteration_number = IterationNumber::first();
        self.iteration = self.current_iteration_number.get();
        self.current_plan = None;
        self.current_step_index = StepIndex::zero();
        self.current_step = self.current_step_index.get();

        tracing::info!("=== Starting Agent Lifecycle ===");
        tracing::info!("Goal: {}", goal);
        tracing::info!("Max iterations: {}", self.max_iterations_limit.get());
        tracing::info!("Global timeout: {:?}", self.global_timeout);

        self.transition_to(AgentState::LoadConfig)
            .map_err(|e| LifecycleError::Fatal {
                iteration: 0,
                error: e,
                context: "Failed to load config".to_string(),
            })?;

        self.transition_to(AgentState::LoadMemory)
            .map_err(|e| LifecycleError::Fatal {
                iteration: 0,
                error: e,
                context: "Failed to load memory".to_string(),
            })?;

        self.transition_to(AgentState::ReceiveGoal)
            .map_err(|e| LifecycleError::Fatal {
                iteration: 0,
                error: e,
                context: "Failed to receive goal".to_string(),
            })?;

        self.memory
            .metadata
            .insert("goal".to_string(), goal.to_string());

        let result = self.execute_main_loop(start_time);

        self.metrics.log_summary();

        if let Err(e) = self
            .audit
            .log_final_state(&format!("{:?}", self.state), self.iteration)
        {
            tracing::error!("Failed to log final state: {}", e);
        }

        tracing::info!("=== Agent Lifecycle Complete ===");
        tracing::info!("Final state: {:?}", self.state);
        tracing::info!("Total iterations: {}", self.iteration);
        tracing::info!("Total duration: {:?}", start_time.elapsed());

        result
    }

    fn execute_main_loop(&mut self, start_time: Instant) -> LifecycleResult<()> {
        let mut recoverable_attempts = 0usize;

        while !self.state.is_terminal() {
            if start_time.elapsed() > self.global_timeout {
                tracing::error!("Global timeout exceeded: {:?}", start_time.elapsed());
                self.metrics.record_iteration_failure(start_time.elapsed());

                return Err(LifecycleError::Timeout {
                    iteration: self.iteration,
                    elapsed: start_time.elapsed(),
                    limit: self.global_timeout,
                });
            }

            if self
                .current_iteration_number
                .exceeds(self.max_iterations_limit)
            {
                tracing::error!(
                    "Maximum iterations exceeded: {} > {}",
                    self.current_iteration_number,
                    self.max_iterations_limit.get()
                );

                self.memory.add_failure(
                    self.iteration,
                    "Maximum iterations exceeded".to_string(),
                    format!(
                        "Agent exceeded maximum allowed iterations ({})",
                        self.max_iterations_limit.get()
                    ),
                    None,
                );

                self.transition_to(AgentState::Failed)
                    .map_err(|e| LifecycleError::Fatal {
                        iteration: self.iteration,
                        error: e,
                        context: "Failed to transition to Failed state".to_string(),
                    })?;

                return Err(LifecycleError::ResourceExhausted {
                    resource: ResourceType::Iterations,
                    limit: self.max_iterations_limit.get(),
                    current: self.iteration,
                });
            }

            let iteration_start = Instant::now();
            self.metrics.record_iteration_start();

            let result = self.execute_current_state();
            let duration = iteration_start.elapsed();

            match result {
                Ok(()) => {
                    recoverable_attempts = 0;
                    self.metrics.record_iteration_success(duration);
                }
                Err(err) => {
                    self.metrics.record_iteration_failure(duration);

                    if err.is_recoverable() {
                        let delay = err.retry_delay().or_else(|| {
                            self.retry_strategy.delay_for_attempt(recoverable_attempts)
                        });

                        if let Some(delay) = delay {
                            recoverable_attempts = recoverable_attempts.saturating_add(1);
                            tracing::warn!(
                                "Recoverable error (attempt {}/{}), retrying in {:?}: {}",
                                recoverable_attempts,
                                self.retry_strategy.max_attempts(),
                                delay,
                                err
                            );
                            std::thread::sleep(delay);
                            continue;
                        }

                        tracing::error!("Recoverable error exhausted retries: {}", err);
                    }

                    return Err(err);
                }
            }
        }

        Ok(())
    }

    fn execute_current_state(&mut self) -> LifecycleResult<()> {
        self.execution_context = Some(ExecutionContext::new(
            self.current_iteration_number,
            self.iteration_timeout,
        ));

        tracing::debug!(
            "Executing state {:?} (iteration {})",
            self.state,
            self.current_iteration_number
        );

        let result = match self.state {
            AgentState::ExploreRepository => self.explore_repository(),
            AgentState::GeneratePlan => self.generate_plan(),
            AgentState::ExecuteStep => self.execute_step(),
            AgentState::Verify => self.verify_step(),
            AgentState::EvaluateObjectives => self.evaluate_objectives(),
            AgentState::PrCreation => self.create_pr(),
            AgentState::ReviewFeedback => self.handle_review(),
            _ => {
                if let Some(next) = self.state.next_on_success() {
                    self.transition_to(next)
                        .map_err(|error| LifecycleError::Fatal {
                            iteration: self.iteration,
                            error,
                            context: "State transition failed".to_string(),
                        })?;
                }
                Ok(())
            }
        };

        self.execution_context = None;

        result.map_err(|error| match error {
            AgentError::PolicyViolation(_) => LifecycleError::Fatal {
                iteration: self.iteration,
                error,
                context: "Policy violation during state execution".to_string(),
            },
            _ => LifecycleError::Recoverable {
                iteration: self.iteration,
                error,
                retry_after: None,
            },
        })
    }

    fn check_timeout(&self) -> AgentResult<()> {
        if let Some(ctx) = &self.execution_context
            && ctx.is_timed_out()
        {
            return Err(AgentError::State(format!(
                "Iteration {} timed out: {:?} > {:?}",
                ctx.iteration.get(),
                ctx.start_time.elapsed(),
                ctx.timeout
            )));
        }
        Ok(())
    }

    pub fn transition_to(&mut self, new_state: AgentState) -> AgentResult<()> {
        let old_state = format!("{:?}", self.state);
        let new_state_str = format!("{:?}", new_state);

        tracing::debug!(
            "State transition: {} -> {} (iteration {})",
            old_state,
            new_state_str,
            self.current_iteration_number
        );

        self.audit
            .log_state_transition(&old_state, &new_state_str)
            .map_err(|e| AgentError::State(e.to_string()))?;

        self.metrics.record_state_transition();
        self.state = new_state;
        Ok(())
    }

    fn explore_repository(&mut self) -> AgentResult<()> {
        self.check_timeout()?;

        tracing::info!(
            "Iteration {}: Exploring repository",
            self.current_iteration_number
        );

        self.memory.add_explored_file("README.md".to_string());
        self.memory.add_explored_file("src/main.rs".to_string());
        self.memory.add_explored_file("Cargo.toml".to_string());

        self.transition_to(AgentState::GeneratePlan)
    }

    fn generate_plan(&mut self) -> AgentResult<()> {
        self.check_timeout()?;

        tracing::info!(
            "Iteration {}: Generating execution plan",
            self.current_iteration_number
        );

        let goal = self
            .memory
            .metadata
            .get("goal")
            .ok_or_else(|| AgentError::State("No goal set".to_string()))?
            .clone();

        let neural_suggestion = if self.neural.enabled {
            match self.neural.propose_action(&goal) {
                Ok(suggestion) => {
                    if let Some(ref suggested) = suggestion {
                        tracing::debug!(
                            "Neural suggestion: {} (confidence: {:.2})",
                            suggested.action,
                            suggested.confidence
                        );

                        self.audit
                            .log_neural_suggestion(&suggested.action, suggested.confidence)
                            .map_err(|e| AgentError::State(e.to_string()))?;
                    }
                    suggestion
                }
                Err(err) => {
                    tracing::warn!("Neural layer failed to propose action: {}", err);
                    None
                }
            }
        } else {
            None
        };

        let mut plan = Plan::new(goal.clone());
        self.build_plan_steps(&mut plan, &goal, neural_suggestion.as_ref())?;
        self.validate_plan(&plan)?;

        let plan_description = format!(
            "Plan for iteration {} ({} steps)",
            self.current_iteration_number,
            plan.steps.len()
        );

        self.memory.add_plan(
            self.iteration,
            plan_description,
            plan.steps.iter().map(|s| s.description.clone()).collect(),
        );

        tracing::info!(
            "Generated plan with {} steps for iteration {}",
            plan.steps.len(),
            self.current_iteration_number
        );

        self.current_plan = Some(plan);
        self.current_step_index = StepIndex::zero();
        self.current_step = self.current_step_index.get();

        self.transition_to(AgentState::ExecuteStep)
    }

    fn build_plan_steps(
        &self,
        plan: &mut Plan,
        goal: &str,
        neural_suggestion: Option<&crate::symbolic::NeuralProposal>,
    ) -> AgentResult<()> {
        plan.add_step(PlanStep {
            description: format!(
                "Read repository structure (iteration {})",
                self.current_iteration_number
            ),
            tool: "read_file".to_string(),
            args: vec!["Cargo.toml".to_string()],
            verification: "file_exists".to_string(),
        });

        if let Some(suggestion) = neural_suggestion
            && suggestion.confidence > 0.7
        {
            tracing::debug!(
                "Including neural suggestion in planning: {}",
                suggestion.action
            );
        }

        if let Some(command_plan) = select_validation_command(goal, &self.config.agent_name) {
            plan.add_step(PlanStep {
                description: command_plan.description,
                tool: "run_tests".to_string(),
                args: command_plan.command_tokens,
                verification: "validation_passes".to_string(),
            });
        }

        Ok(())
    }

    fn validate_plan(&self, plan: &Plan) -> AgentResult<()> {
        for (idx, step) in plan.steps.iter().enumerate() {
            if !self.policy.is_tool_allowed(&step.tool) {
                return Err(AgentError::PolicyViolation(format!(
                    "Step {}: Tool '{}' not allowed by policy",
                    idx + 1,
                    step.tool
                )));
            }

            let action = build_action_from_step(step);
            if !self.policy.validate_action(&action) {
                return Err(AgentError::PolicyViolation(format!(
                    "Step {}: action '{}' violates policy patterns",
                    idx + 1,
                    action
                )));
            }

            self.enforce_authz_for_action(&step.tool)?;
        }
        Ok(())
    }

    fn execute_step(&mut self) -> AgentResult<()> {
        self.check_timeout()?;

        let steps_len = self
            .current_plan
            .as_ref()
            .ok_or_else(|| AgentError::State("No plan available".to_string()))?
            .steps
            .len();

        if self.current_step_index.get() >= steps_len {
            tracing::info!(
                "Iteration {}: All {} steps completed successfully",
                self.current_iteration_number,
                steps_len
            );
            self.transition_to(AgentState::PrCreation)?;
            return Ok(());
        }

        let step = self
            .current_plan
            .as_ref()
            .ok_or_else(|| AgentError::State("No plan available".to_string()))?
            .steps[self.current_step_index.get()]
        .clone();

        tracing::info!(
            "Iteration {}: Executing step {}/{}: {}",
            self.current_iteration_number,
            self.current_step_index.get() + 1,
            steps_len,
            step.description
        );

        if !self.policy.is_tool_allowed(&step.tool) {
            let error_msg = format!(
                "Tool '{}' not allowed by policy (step {})",
                step.tool,
                self.current_step_index.get() + 1
            );
            tracing::error!("{}", error_msg);

            self.memory.add_failure(
                self.iteration,
                "Policy violation during execution".to_string(),
                error_msg.clone(),
                Some("Plan validation should have caught this".to_string()),
            );

            return Err(AgentError::PolicyViolation(error_msg));
        }

        let action = build_action_from_step(&step);
        if !self.policy.validate_action(&action) {
            let error_msg = format!(
                "Action '{}' violates policy patterns (step {})",
                action,
                self.current_step_index.get() + 1
            );
            tracing::error!("{}", error_msg);
            self.memory.add_failure(
                self.iteration,
                "Policy action validation failed".to_string(),
                error_msg.clone(),
                None,
            );
            return Err(AgentError::PolicyViolation(error_msg));
        }

        self.enforce_authz_for_action(&step.tool)?;

        {
            let breaker = self
                .circuit_breakers
                .entry(step.tool.clone())
                .or_insert_with(|| CircuitBreaker::new(3, 2, Duration::from_secs(60)));

            if !breaker.should_allow_request() {
                tracing::warn!(
                    "Circuit breaker open for tool '{}', skipping execution",
                    step.tool
                );

                return Err(AgentError::Tool(format!(
                    "Circuit breaker open for tool '{}'",
                    step.tool
                )));
            }
        }

        let tool_start = Instant::now();
        let result = self.execute_tool_with_timeout(&step.tool, &step.args)?;
        let tool_duration = tool_start.elapsed();

        self.metrics
            .record_tool_execution(&step.tool, result.success, tool_duration);

        {
            let breaker = self
                .circuit_breakers
                .entry(step.tool.clone())
                .or_insert_with(|| CircuitBreaker::new(3, 2, Duration::from_secs(60)));

            if result.success {
                breaker.record_success();
            } else {
                breaker.record_failure();
            }
        }

        self.audit
            .log_tool_execution(&step.tool, &step.args, result.success)
            .map_err(|e| AgentError::State(e.to_string()))?;

        if !result.success {
            let error_detail = result.error.unwrap_or_else(|| "Unknown error".to_string());

            tracing::warn!(
                "Tool '{}' failed: {} (duration: {:?})",
                step.tool,
                error_detail,
                tool_duration
            );

            self.memory.add_failure(
                self.iteration,
                format!("Tool execution failed: {}", step.tool),
                error_detail.clone(),
                Some("Will retry in next iteration".to_string()),
            );

            return Err(AgentError::Tool(format!(
                "Tool '{}' execution failed: {}",
                step.tool, error_detail
            )));
        }

        self.memory.add_decision(
            self.iteration,
            format!("Execute {}", step.tool),
            None,
            "Tool executed successfully".to_string(),
        );

        self.current_step_index = self.current_step_index.increment();
        self.current_step = self.current_step_index.get();

        self.transition_to(AgentState::Verify)
    }

    fn execute_tool_with_timeout(
        &self,
        tool_name: &str,
        args: &[String],
    ) -> AgentResult<ToolResult> {
        let tool = self
            .tools
            .get_tool(tool_name)
            .ok_or_else(|| AgentError::Tool(format!("Tool '{}' not found", tool_name)))?;

        let start = Instant::now();
        let mut result = tool.execute(args)?;

        if start.elapsed() > self.tool_timeout {
            result.success = false;
            result.error = Some(format!(
                "Tool '{}' timed out after {:?}",
                tool_name, self.tool_timeout
            ));
        }

        Ok(result)
    }

    fn enforce_authz_for_action(&self, action: &str) -> AgentResult<()> {
        match self.authz.check(&self.actor, action) {
            AuthzDecision::Allow => Ok(()),
            AuthzDecision::Deny { reason } => Err(AgentError::PolicyViolation(format!(
                "Authorization denied for action '{}': {}",
                action, reason
            ))),
            AuthzDecision::RequiresEscalation { required_role } => {
                Err(AgentError::PolicyViolation(format!(
                    "Action '{}' requires escalation: {}",
                    action, required_role
                )))
            }
        }
    }

    fn verify_step(&mut self) -> AgentResult<()> {
        self.check_timeout()?;

        tracing::info!(
            "Iteration {}: Verifying step {} execution",
            self.current_iteration_number,
            self.current_step_index.get()
        );

        self.transition_to(AgentState::EvaluateObjectives)
    }

    fn evaluate_objectives(&mut self) -> AgentResult<()> {
        self.check_timeout()?;

        tracing::info!(
            "Iteration {}: Evaluating objectives",
            self.current_iteration_number
        );

        let scores = self.compute_objective_scores()?;
        let objective_scores = self.symbolic.evaluator.evaluate(&scores);
        let hard_satisfied = self
            .symbolic
            .evaluator
            .hard_objectives_satisfied(&objective_scores);

        self.audit
            .log_objective_evaluation(self.iteration, scores.clone())
            .map_err(|e| AgentError::State(e.to_string()))?;

        self.memory
            .add_objective_evaluation(self.iteration, scores.clone(), hard_satisfied);

        if !hard_satisfied {
            tracing::warn!(
                "Iteration {}: Hard objectives not satisfied",
                self.current_iteration_number
            );

            let Some(next_iteration) = self.current_iteration_number.try_next() else {
                self.transition_to(AgentState::Failed)?;
                return Err(AgentError::State("Iteration counter overflow".to_string()));
            };

            if next_iteration.exceeds(self.max_iterations_limit) {
                self.memory.add_failure(
                    self.iteration,
                    "Hard objectives not satisfied - max iterations reached".to_string(),
                    format!(
                        "Failed after {} iterations with scores: {:?}",
                        self.current_iteration_number.get(),
                        scores
                    ),
                    None,
                );

                self.transition_to(AgentState::Failed)?;
                return Ok(());
            }

            self.memory.add_failure(
                self.iteration,
                "Hard objectives not satisfied".to_string(),
                format!("Objective scores: {:?}", scores),
                Some(format!(
                    "Retry with adjusted approach (attempt {}/{})",
                    next_iteration,
                    self.max_iterations_limit.get()
                )),
            );

            self.current_iteration_number = next_iteration;
            self.iteration = next_iteration.get();
            self.current_plan = None;
            self.current_step_index = StepIndex::zero();
            self.current_step = self.current_step_index.get();

            self.transition_to(AgentState::GeneratePlan)?;
        } else {
            tracing::info!(
                "Iteration {}: All hard objectives satisfied",
                self.current_iteration_number
            );

            self.transition_to(AgentState::ExecuteStep)?;
        }

        Ok(())
    }

    fn compute_objective_scores(&self) -> AgentResult<Vec<(String, f64)>> {
        // Simulate progress across iterations: iteration 1 fails task_completion,
        // iteration >= 2 succeeds hard objectives.
        let task_completion = if self.current_iteration_number.get() >= 2 {
            1.0
        } else {
            0.6
        };

        Ok(vec![
            ("task_completion".to_string(), task_completion),
            ("policy_safety".to_string(), 1.0),
            ("tests_pass".to_string(), 1.0),
            ("minimal_diff".to_string(), 0.8),
            ("time_budget".to_string(), 0.7),
        ])
    }

    fn create_pr(&mut self) -> AgentResult<()> {
        self.check_timeout()?;

        tracing::info!(
            "Iteration {}: Creating pull request",
            self.current_iteration_number
        );

        let goal = self
            .memory
            .metadata
            .get("goal")
            .unwrap_or(&"No goal".to_string())
            .clone();

        let default_pr_body = self.build_default_pr_body(&goal);

        let pr_body = self
            .try_generate_enhanced_pr_description(&goal, &default_pr_body)
            .unwrap_or(default_pr_body);

        self.audit
            .log_symbolic_decision("create_pr", &pr_body)
            .map_err(|e| AgentError::State(e.to_string()))?;

        tracing::info!("PR would be created with body ({} chars)", pr_body.len());

        self.transition_to(AgentState::ReviewFeedback)
    }

    fn build_default_pr_body(&self, goal: &str) -> String {
        let plan_summary = self
            .current_plan
            .as_ref()
            .map(|p| format!("{} steps executed", p.steps.len()))
            .unwrap_or_else(|| "No plan".to_string());

        let metrics = self.metrics.snapshot();

        format!(
            "## Goal\n\
             {}\n\n\
             ## Execution Summary\n\
             - Iterations: {}\n\
             - Plan: {}\n\
             - Tool executions: {} ({} failed)\n\
             - Duration: {:?}\n\n\
             ## Risk Assessment\n\
             Low - All hard objectives satisfied\n\n\
             ## Verification\n\
             - [ ] Tests pass\n\
             - [ ] No policy violations\n\
             - [ ] Code review complete",
            goal,
            self.current_iteration_number,
            plan_summary,
            metrics.tool_executions_total,
            metrics.tool_executions_failed,
            metrics.total_duration,
        )
    }

    fn try_generate_enhanced_pr_description(
        &mut self,
        goal: &str,
        default_body: &str,
    ) -> Option<String> {
        let main_pr_number = std::env::var("AUTONOMOUS_MAIN_PR_NUMBER")
            .ok()
            .or_else(|| Self::extract_pr_number_from_goal(goal));

        let output_file = std::env::var("AUTONOMOUS_PR_DESCRIPTION_OUTPUT")
            .unwrap_or_else(|_| "pr_description.md".to_string());

        let tool_name = "generate_pr_description";

        if !self.policy.is_tool_allowed(tool_name) {
            tracing::debug!("Tool '{}' not allowed by policy", tool_name);
            return Some(default_body.to_string());
        }

        let main_pr_number = main_pr_number?;
        let tool_args = vec![main_pr_number, output_file.clone()];

        match self.generate_pr_description(&tool_args, &output_file) {
            Ok(generated) => Some(generated),
            Err(err) => {
                self.memory.add_failure(
                    self.iteration,
                    "PR description generation failed".to_string(),
                    err.to_string(),
                    Some("Using default PR body".to_string()),
                );
                None
            }
        }
    }

    fn generate_pr_description(
        &mut self,
        tool_args: &[String],
        output_file: &str,
    ) -> AgentResult<String> {
        let tool_name = "generate_pr_description";
        let tool_start = Instant::now();

        let tool = self
            .tools
            .get_tool(tool_name)
            .ok_or_else(|| AgentError::Tool(format!("Tool '{}' not found", tool_name)))?;

        let result = tool.execute(tool_args)?;
        let tool_duration = tool_start.elapsed();

        self.metrics
            .record_tool_execution(tool_name, result.success, tool_duration);

        self.audit
            .log_tool_execution(tool_name, tool_args, result.success)
            .map_err(|e| AgentError::State(e.to_string()))?;

        if !result.success {
            return Err(AgentError::Tool(
                result.error.unwrap_or_else(|| "Unknown error".to_string()),
            ));
        }

        let generated = std::fs::read_to_string(output_file).map_err(|e| {
            AgentError::Tool(format!(
                "Failed to read generated PR description from '{}': {}",
                output_file, e
            ))
        })?;

        self.audit
            .log_file_modified(output_file)
            .map_err(|e| AgentError::State(e.to_string()))?;

        self.memory.metadata.insert(
            "generated_pr_description".to_string(),
            output_file.to_string(),
        );

        Ok(generated)
    }

    fn handle_review(&mut self) -> AgentResult<()> {
        self.check_timeout()?;

        tracing::info!(
            "Iteration {}: Handling review feedback",
            self.current_iteration_number
        );

        self.transition_to(AgentState::Done)
    }

    pub fn metrics(&self) -> LifecycleMetrics {
        self.metrics.snapshot()
    }

    pub fn current_state(&self) -> AgentState {
        self.state
    }

    pub fn current_iteration(&self) -> usize {
        self.current_iteration_number.get()
    }
}

fn build_action_from_step(step: &PlanStep) -> String {
    if step.args.is_empty() {
        step.tool.clone()
    } else {
        format!("{} {}", step.tool, step.args.join(" "))
    }
}
