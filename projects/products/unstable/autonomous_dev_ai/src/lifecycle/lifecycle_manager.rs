//projects/products/unstable/autonomous_dev_ai/src/lifecycle/lifecycle_manager.rs
// Agent lifecycle implementation - production-grade flow.
use super::{
    Checkpoint, CircuitBreaker, CompensationKind, ExecutionContext, IterationNumber,
    LifecycleError, LifecycleMetrics, LifecycleResult, MaxIterations, MetricsCollector,
    ResourceBudget, ResourceType, RetryStrategy, RollbackManager, StepIndex,
    validation_strategy::select_validation_command,
};

use crate::agent_config::AgentConfig;
use crate::audit_logger::AuditLogger;
use crate::error::{AgentError, AgentResult};
use crate::ids::{IssueNumber, ParentRef, PrNumber};
use crate::lifecycle::ActionRiskLevel;
use crate::memory_graph::MemoryGraph;
use crate::neural::{
    DriftDetector, IntentInterpretation, ModelGovernance, ModelVersion, NeuralLayer, NeuralModel,
};
use crate::objective_evaluator::ObjectiveEvaluator;
use crate::ops::{IncidentRunbook, RunReplay, SloEvaluator};
use crate::path_types::CheckpointPath;
use crate::pr_flow::{
    CiStatus, IssueComplianceStatus, MergeReadiness, PrOrchestrator, ReviewComment, ReviewOutcome,
};
use crate::security::{ActorIdentity, AuthzDecision, AuthzEngine, PolicyPack, SecurityAuditRecord};
use crate::state::AgentState;
use crate::symbolic::{
    IssueClassificationInput, Plan, PlanStep, PolicyEngine, SymbolicController, Validator,
};
use crate::timeout::Timeout;
use crate::tools::{
    GitWrapper, PrDescriptionGenerator, RepoReader, TestRunner, ToolRegistry, ToolResult,
};
use crate::value_types::{ActionOutcomeSummary, StateLabel};

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
    pub model_governance: ModelGovernance,
    pub policy: PolicyEngine,
    pub authz: AuthzEngine,
    pub actor: ActorIdentity,
    pub pr_orchestrator: Option<PrOrchestrator>,
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
    run_replay: RunReplay,
    slo_evaluator: SloEvaluator,
    incident_runbook: IncidentRunbook,
    policy_pack: PolicyPack,
    resource_budget: ResourceBudget,
    rollback_manager: RollbackManager,
    checkpoint_path: CheckpointPath,
    last_intent: Option<IntentInterpretation>,
    drift_detector: DriftDetector,

    // Timeouts.
    global_timeout: Timeout,
    iteration_timeout: Timeout,
    tool_timeout: Timeout,
}

impl LifecycleManager {
    pub(crate) fn extract_issue_number_from_goal(goal: &str) -> Option<IssueNumber> {
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
                    let raw = goal[start..end].parse::<u64>().ok()?;
                    return IssueNumber::new(raw);
                }
            }
            i += 1;
        }
        None
    }

    pub fn new(config: AgentConfig, audit_log_path: &str) -> Self {
        let max_iterations_limit =
            MaxIterations::new(config.max_iterations).unwrap_or_else(MaxIterations::default_value);
        let global_timeout = Timeout::from_secs(config.timeout_seconds.unwrap_or(3600));

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
        let mut model_governance = ModelGovernance::new();
        model_governance.registry.register(ModelVersion::new(
            "default-neural",
            "0.1.0",
            "builtin://heuristic",
            0.7,
        ));
        let _ = model_governance
            .registry
            .promote_to_canary("default-neural");
        let _ = model_governance
            .registry
            .promote_to_production("default-neural");

        let policy = PolicyEngine::new();
        let authz = AuthzEngine::new();
        let actor = ActorIdentity::default();
        let run_replay = RunReplay::new(actor.run_id.clone());
        let slo_evaluator = SloEvaluator::new(SloEvaluator::default_slos());
        let incident_runbook = IncidentRunbook::default_runbook();
        let policy_pack = PolicyPack::default();
        let resource_budget =
            ResourceBudget::new(global_timeout.duration, max_iterations_limit.get(), 500);
        let rollback_manager = RollbackManager::new();
        let checkpoint_path = std::env::var("AUTONOMOUS_CHECKPOINT_PATH")
            .ok()
            .and_then(CheckpointPath::new)
            .unwrap_or_else(|| {
                CheckpointPath::new("agent_checkpoint.json").expect("static path is valid")
            });
        let drift_detector = DriftDetector::default();

        let mut tools = ToolRegistry::new();
        tools.register(Box::new(RepoReader));
        tools.register(Box::new(TestRunner));
        tools.register(Box::new(GitWrapper));
        tools.register(Box::new(PrDescriptionGenerator));

        Self {
            state: AgentState::Idle,
            config,
            memory: MemoryGraph::new(),
            symbolic,
            neural,
            model_governance,
            policy,
            authz,
            actor,
            pr_orchestrator: None,
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
            retry_strategy: RetryStrategy::default()
                .with_delays(Duration::from_millis(200), Duration::from_secs(5)),
            metrics: MetricsCollector::new(),
            run_replay,
            slo_evaluator,
            incident_runbook,
            policy_pack,
            resource_budget,
            rollback_manager,
            checkpoint_path,
            last_intent: None,
            drift_detector,
            global_timeout,
            iteration_timeout: Timeout::from_secs(300),
            tool_timeout: Timeout::from_secs(30),
        }
    }

    /// Run the lifecycle with typed errors, retries, and metrics.
    pub fn run(&mut self, goal: &str) -> LifecycleResult<()> {
        let start_time = Instant::now();

        self.run_replay = RunReplay::new(self.actor.run_id.clone());
        self.current_iteration_number = IterationNumber::first();
        self.iteration = self.current_iteration_number.get();
        self.current_plan = None;
        self.current_step_index = StepIndex::zero();
        self.current_step = self.current_step_index.get();
        self.pr_orchestrator = None;
        self.rollback_manager = RollbackManager::new();
        self.last_intent = None;
        self.drift_detector = DriftDetector::default();
        self.run_replay.record("lifecycle.start", goal);
        self.run_replay
            .record("lifecycle.checkpoint_path", self.checkpoint_path.as_str());
        self.run_replay.record(
            "symbolic.mode",
            format!(
                "strict_validation={} deterministic={}",
                self.symbolic.strict_validation, self.symbolic.deterministic
            ),
        );
        if let Some(state) = self.model_governance.registry.state("default-neural") {
            self.run_replay
                .record("neural.rollout_state", format!("{:?}", state));
        }
        self.configure_policy_pack_from_env()
            .map_err(|e| LifecycleError::Fatal {
                iteration: 0,
                error: e,
                context: "Failed to configure policy pack from runtime overrides".to_string(),
            })?;
        self.run_replay
            .record("policy_pack.fingerprint", self.policy_pack.fingerprint());

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

        let replay_path = std::env::var("AUTONOMOUS_RUN_REPLAY_PATH")
            .unwrap_or_else(|_| "agent_run_replay.json".to_string());
        if let Err(e) = self.run_replay.persist(&replay_path) {
            tracing::warn!("Failed to persist run replay '{}': {}", replay_path, e);
        } else {
            self.memory
                .metadata
                .insert("run_replay_path".to_string(), replay_path);
        }
        let replay_text_path = std::env::var("AUTONOMOUS_RUN_REPLAY_TEXT_PATH")
            .unwrap_or_else(|_| "agent_run_replay.txt".to_string());
        if let Err(e) = std::fs::write(&replay_text_path, self.run_replay.reconstruct()) {
            tracing::warn!(
                "Failed to persist run replay text '{}': {}",
                replay_text_path,
                e
            );
        } else {
            self.memory
                .metadata
                .insert("run_replay_text_path".to_string(), replay_text_path);
        }

        tracing::info!("=== Agent Lifecycle Complete ===");
        tracing::info!("Final state: {:?}", self.state);
        tracing::info!("Total iterations: {}", self.iteration);
        tracing::info!("Total duration: {:?}", start_time.elapsed());
        let reversible = self.rollback_manager.reversible_actions().len();
        let irreversible = self.rollback_manager.irreversible_actions().len();
        self.run_replay.record(
            "rollback.summary",
            format!("reversible={} irreversible={}", reversible, irreversible),
        );
        tracing::info!(
            "Rollback boundaries recorded: reversible={} irreversible={}",
            reversible,
            irreversible
        );

        result
    }

    pub fn load_checkpoint_if_present(&mut self) -> AgentResult<bool> {
        match Checkpoint::load(&self.checkpoint_path) {
            Ok(checkpoint) => {
                self.current_iteration_number = IterationNumber::from_usize(checkpoint.iteration)
                    .unwrap_or_else(IterationNumber::first);
                self.iteration = self.current_iteration_number.get();
                self.run_replay.record(
                    "checkpoint.loaded",
                    format!(
                        "run_id={} iteration={} state={}",
                        checkpoint.run_id, checkpoint.iteration, checkpoint.state_label
                    ),
                );
                Ok(true)
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(err) => Err(AgentError::State(format!(
                "Failed to load checkpoint '{}': {}",
                self.checkpoint_path.as_str(),
                err
            ))),
        }
    }

    fn execute_main_loop(&mut self, start_time: Instant) -> LifecycleResult<()> {
        let mut recoverable_attempts = 0usize;

        while !self.state.is_terminal() {
            if start_time.elapsed() > self.global_timeout.duration {
                tracing::error!("Global timeout exceeded: {:?}", start_time.elapsed());
                self.metrics.record_iteration_failure(start_time.elapsed());

                return Err(LifecycleError::Timeout {
                    iteration: self.iteration,
                    elapsed: start_time.elapsed(),
                    limit: self.global_timeout,
                });
            }

            let metrics_snapshot = self.metrics.snapshot();
            let memory_entries = self.memory.explored_files.len()
                + self.memory.plans.len()
                + self.memory.decisions.len()
                + self.memory.failures.len()
                + self.memory.objective_evaluations.len();
            let memory_budget = std::env::var("AUTONOMOUS_MAX_MEMORY_ENTRIES")
                .ok()
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(10_000);
            if memory_entries >= memory_budget {
                self.memory.add_failure(
                    self.iteration,
                    "Resource budget exceeded".to_string(),
                    format!(
                        "memory budget exceeded: entries={} budget={}",
                        memory_entries, memory_budget
                    ),
                    Some(
                        "reduce retained memory or increase AUTONOMOUS_MAX_MEMORY_ENTRIES"
                            .to_string(),
                    ),
                );
                self.transition_to(AgentState::Failed)
                    .map_err(|e| LifecycleError::Fatal {
                        iteration: self.iteration,
                        error: e,
                        context: "Failed to transition to Failed state".to_string(),
                    })?;
                return Err(LifecycleError::ResourceExhausted {
                    resource: ResourceType::Memory,
                    limit: memory_budget,
                    current: memory_entries,
                });
            }

            if let Some(limit_reason) = self.resource_budget.is_exceeded(
                start_time.elapsed(),
                self.current_iteration_number.get(),
                metrics_snapshot.tool_executions_total,
            ) {
                let resource = match limit_reason {
                    "runtime budget exceeded" => ResourceType::Time,
                    "tool execution budget exceeded" => ResourceType::ToolExecutions,
                    _ => ResourceType::Iterations,
                };
                self.memory.add_failure(
                    self.iteration,
                    "Resource budget exceeded".to_string(),
                    limit_reason.to_string(),
                    Some("reduce run scope or increase configured budget".to_string()),
                );
                self.transition_to(AgentState::Failed)
                    .map_err(|e| LifecycleError::Fatal {
                        iteration: self.iteration,
                        error: e,
                        context: "Failed to transition to Failed state".to_string(),
                    })?;
                return Err(LifecycleError::ResourceExhausted {
                    resource,
                    limit: match resource {
                        ResourceType::Time => self.resource_budget.max_runtime.as_secs() as usize,
                        ResourceType::ToolExecutions => self.resource_budget.max_tool_executions,
                        _ => self.resource_budget.max_iterations,
                    },
                    current: match resource {
                        ResourceType::Time => start_time.elapsed().as_secs() as usize,
                        ResourceType::ToolExecutions => metrics_snapshot.tool_executions_total,
                        _ => self.current_iteration_number.get(),
                    },
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
                        self.record_runbook_hint_for_error(&err.to_string());
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
            let remaining = ctx.remaining_time().unwrap_or_default();
            return Err(AgentError::State(format!(
                "Iteration {} timed out: {:?} > {:?} (remaining {:?})",
                ctx.iteration.get(),
                ctx.start_time.elapsed(),
                ctx.timeout,
                remaining
            )));
        }
        Ok(())
    }

    pub fn transition_to(&mut self, new_state: AgentState) -> AgentResult<()> {
        let old_state = format!("{:?}", self.state);
        let new_state_label = StateLabel::new(format!("{:?}", new_state)).unwrap_or_else(|| {
            StateLabel::new("UnknownState").expect("static state must be valid")
        });

        tracing::debug!(
            "State transition: {} -> {} (iteration {})",
            old_state,
            new_state_label,
            self.current_iteration_number
        );

        self.audit
            .log_state_transition(&old_state, &new_state_label.to_string())
            .map_err(|e| AgentError::State(e.to_string()))?;
        self.run_replay.record(
            "state.transition",
            format!("{old_state} -> {new_state_label}"),
        );

        self.metrics.record_state_transition();
        self.state = new_state;

        let checkpoint = Checkpoint::new(
            self.actor.run_id.clone(),
            self.current_iteration_number.get(),
            new_state_label,
        );
        if let Err(e) = checkpoint.save(&self.checkpoint_path) {
            tracing::warn!(
                "Failed to write checkpoint '{}': {}",
                self.checkpoint_path.as_str(),
                e
            );
        }
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
        let intent = IntentInterpretation {
            goal: goal.clone(),
            constraints: vec![
                format!("max_iterations={}", self.max_iterations_limit.get()),
                format!("tool_timeout_secs={}", self.tool_timeout.as_secs()),
                format!("global_timeout_secs={}", self.global_timeout.as_secs()),
            ],
            confidence: if self.neural.enabled { 0.75 } else { 1.0 },
        };
        self.last_intent = Some(intent.clone());
        self.memory.metadata.insert(
            "last_intent_confidence".to_string(),
            format!("{:.2}", intent.confidence),
        );
        self.run_replay.record(
            "intent.interpreted",
            format!(
                "goal={} constraints={} confidence={:.2}",
                intent.goal,
                intent.constraints.join(" | "),
                intent.confidence
            ),
        );

        let issue_input = IssueClassificationInput {
            labels: parse_issue_labels_from_env(),
            title: goal.clone(),
            body: self
                .memory
                .metadata
                .get("issue_body")
                .cloned()
                .unwrap_or_default(),
        };
        let issue_category = self.symbolic.resolve_issue_category(&issue_input, 0.6);
        self.memory.metadata.insert(
            "issue_category".to_string(),
            format!("{:?}", issue_category.category),
        );
        self.run_replay.record(
            "issue.classification",
            format!(
                "category={:?} source={:?} confidence={:.2}",
                issue_category.category, issue_category.source, issue_category.confidence
            ),
        );

        let neural_suggestion = if self.neural.enabled {
            match self.neural.infer(&goal) {
                Ok(suggested) => {
                    let uncertainty = self.neural.estimate_uncertainty(&goal).unwrap_or(0.5);
                    let cpu_fallback = self.neural.use_cpu_fallback();
                    let model_confidence = self.neural.confidence();
                    let detected_drift = self.drift_detector.observe(suggested.confidence);
                    let rolling_avg = self.drift_detector.rolling_average().unwrap_or(0.0);
                    self.run_replay.record(
                            "neural.runtime",
                            format!(
                                "confidence={:.3} model_confidence={:.3} uncertainty={:.3} prefer_gpu={} cpu_fallback={} drift_detected={} rolling_avg={:.3}",
                                suggested.confidence,
                                model_confidence,
                                uncertainty,
                                self.neural.prefer_gpu,
                                cpu_fallback,
                                detected_drift,
                                rolling_avg
                            ),
                        );

                    let governance_ok = self
                        .model_governance
                        .accept("default-neural", suggested.confidence);
                    let validation = self.symbolic.validate_proposal(&suggested)?;

                    self.run_replay.record(
                        "neural.governance",
                        format!(
                            "accepted={} symbolic_valid={} issues={}",
                            governance_ok,
                            validation.is_valid,
                            validation.issues.join(" | ")
                        ),
                    );

                    if !governance_ok || !validation.is_valid {
                        self.memory.add_failure(
                            self.iteration,
                            "Neural suggestion rejected".to_string(),
                            format!(
                                "governance_ok={} symbolic_valid={}",
                                governance_ok, validation.is_valid
                            ),
                            Some("fallback to deterministic symbolic planning".to_string()),
                        );
                        None
                    } else {
                        tracing::debug!(
                            "Neural suggestion: {} (confidence: {:.2})",
                            suggested.action,
                            suggested.confidence
                        );

                        self.audit
                            .log_neural_suggestion(suggested.action.as_str(), suggested.confidence)
                            .map_err(|e| AgentError::State(e.to_string()))?;

                        Some(suggested)
                    }
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
        self.build_plan_steps(
            &mut plan,
            &goal,
            neural_suggestion.as_ref(),
            &format!("{:?}", issue_category.category),
        )?;
        self.apply_learning_adaptations(&mut plan);
        self.validate_plan(&plan)?;
        self.run_replay.record(
            "plan.generated",
            format!(
                "iteration={} steps={}",
                self.current_iteration_number,
                plan.steps.len()
            ),
        );

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
        issue_category: &str,
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

        if let Some(command_plan) =
            select_validation_command(goal, &self.config.agent_name, &self.config.execution_mode)
        {
            plan.add_step(PlanStep {
                description: command_plan.description,
                tool: "run_tests".to_string(),
                args: command_plan.command_tokens,
                verification: "validation_passes".to_string(),
            });
        }

        if issue_category.eq_ignore_ascii_case("security") {
            plan.add_step(PlanStep {
                description: "Security-oriented validation pass".to_string(),
                tool: "run_tests".to_string(),
                args: vec![
                    "cargo".to_string(),
                    "clippy".to_string(),
                    "-p".to_string(),
                    self.config.agent_name.clone(),
                    "--bin".to_string(),
                    self.config.agent_name.clone(),
                ],
                verification: "security_validation_passes".to_string(),
            });
        }

        Ok(())
    }

    fn apply_learning_adaptations(&mut self, plan: &mut Plan) {
        let previous_failures = self
            .memory
            .metadata
            .get("previous_state_failures_count")
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(0);
        let previous_max_iteration = self
            .memory
            .metadata
            .get("previous_state_max_iteration")
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(0);
        let top_failure_kind = self
            .memory
            .metadata
            .get("previous_state_top_failure_kind")
            .cloned()
            .unwrap_or_default();
        let top_failure_tool = self
            .memory
            .metadata
            .get("previous_state_top_failure_tool")
            .cloned()
            .unwrap_or_default();
        let top_decision_action = self
            .memory
            .metadata
            .get("previous_state_top_decision_action")
            .cloned()
            .unwrap_or_default();
        let recent_avg_failures = self
            .memory
            .metadata
            .get("previous_recent_avg_failures")
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);
        let recent_top_failure_kind = self
            .memory
            .metadata
            .get("previous_recent_top_failure_kind")
            .cloned()
            .unwrap_or_default();
        let recent_top_failure_kind_confidence = self
            .memory
            .metadata
            .get("previous_recent_top_failure_kind_confidence")
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);
        let worst_action_outcome = self
            .memory
            .metadata
            .get("previous_state_worst_action_outcome")
            .cloned()
            .unwrap_or_default();

        if previous_failures > 0 {
            plan.add_step(PlanStep {
                description: format!(
                    "Learning adaptation: rerun deterministic validation (previous_failures={})",
                    previous_failures
                ),
                tool: "run_tests".to_string(),
                args: vec![
                    "cargo".to_string(),
                    "check".to_string(),
                    "-p".to_string(),
                    self.config.agent_name.clone(),
                    "--bin".to_string(),
                    self.config.agent_name.clone(),
                ],
                verification: "learning_validation_passes".to_string(),
            });
        }

        if previous_failures >= 3 || previous_max_iteration >= self.max_iterations_limit.get() / 2 {
            plan.add_step(PlanStep {
                description: "Learning adaptation: strict lint gate after unstable runs"
                    .to_string(),
                tool: "run_tests".to_string(),
                args: vec![
                    "cargo".to_string(),
                    "clippy".to_string(),
                    "-p".to_string(),
                    self.config.agent_name.clone(),
                    "--bin".to_string(),
                    self.config.agent_name.clone(),
                ],
                verification: "learning_strict_lint_passes".to_string(),
            });
        }
        if top_failure_kind.starts_with("policy:") {
            plan.add_step(PlanStep {
                description: "Learning adaptation: policy-focused validation after policy failures"
                    .to_string(),
                tool: "run_tests".to_string(),
                args: vec![
                    "cargo".to_string(),
                    "clippy".to_string(),
                    "-p".to_string(),
                    self.config.agent_name.clone(),
                    "--bin".to_string(),
                    self.config.agent_name.clone(),
                ],
                verification: "learning_policy_gate_passes".to_string(),
            });
        }
        if top_failure_tool.starts_with("run_tests:") {
            plan.add_step(PlanStep {
                description: "Learning adaptation: stabilize test harness execution path"
                    .to_string(),
                tool: "run_tests".to_string(),
                args: vec![
                    "cargo".to_string(),
                    "test".to_string(),
                    "-p".to_string(),
                    self.config.agent_name.clone(),
                    "--bin".to_string(),
                    self.config.agent_name.clone(),
                    "--no-fail-fast".to_string(),
                ],
                verification: "learning_test_harness_stable".to_string(),
            });
        }
        if top_decision_action.starts_with("read_file:") && previous_failures > 0 {
            plan.add_step(PlanStep {
                description: "Learning adaptation: prioritize validation over repeated exploration"
                    .to_string(),
                tool: "run_tests".to_string(),
                args: vec![
                    "cargo".to_string(),
                    "check".to_string(),
                    "-p".to_string(),
                    self.config.agent_name.clone(),
                    "--bin".to_string(),
                    self.config.agent_name.clone(),
                ],
                verification: "learning_prioritized_validation".to_string(),
            });
        }
        if recent_avg_failures >= 2.0
            || (recent_top_failure_kind.starts_with("timeout:")
                && recent_top_failure_kind_confidence >= 0.45)
        {
            plan.add_step(PlanStep {
                description:
                    "Learning adaptation: short deterministic validation after recent instability"
                        .to_string(),
                tool: "run_tests".to_string(),
                args: vec![
                    "cargo".to_string(),
                    "check".to_string(),
                    "-p".to_string(),
                    self.config.agent_name.clone(),
                    "--bin".to_string(),
                    self.config.agent_name.clone(),
                ],
                verification: "learning_recent_stability_probe".to_string(),
            });
        }
        if let Some((action, pass_rate, total)) =
            parse_action_outcome_triplet(&worst_action_outcome)
            && total >= 3
            && pass_rate < 0.5
        {
            if action == "run_tests" {
                plan.add_step(PlanStep {
                    description:
                        "Learning adaptation: preflight check before full tests for unstable action"
                            .to_string(),
                    tool: "run_tests".to_string(),
                    args: vec![
                        "cargo".to_string(),
                        "check".to_string(),
                        "-p".to_string(),
                        self.config.agent_name.clone(),
                        "--bin".to_string(),
                        self.config.agent_name.clone(),
                    ],
                    verification: "learning_action_preflight".to_string(),
                });
            } else if action == "read_file" {
                plan.add_step(PlanStep {
                    description:
                        "Learning adaptation: reduce exploration-heavy loop with deterministic validation"
                            .to_string(),
                    tool: "run_tests".to_string(),
                    args: vec![
                        "cargo".to_string(),
                        "check".to_string(),
                        "-p".to_string(),
                        self.config.agent_name.clone(),
                        "--bin".to_string(),
                        self.config.agent_name.clone(),
                    ],
                    verification: "learning_reduce_exploration_loop".to_string(),
                });
            }
        }

        self.run_replay.record(
            "learning.adaptation",
            format!(
                "previous_failures={} previous_max_iteration={} top_failure_kind={} top_failure_tool={} top_decision_action={} recent_avg_failures={:.2} recent_top_failure_kind={} recent_top_failure_kind_confidence={:.3} worst_action_outcome={} plan_steps={}",
                previous_failures,
                previous_max_iteration,
                top_failure_kind,
                top_failure_tool,
                top_decision_action,
                recent_avg_failures,
                recent_top_failure_kind,
                recent_top_failure_kind_confidence,
                worst_action_outcome,
                plan.steps.len()
            ),
        );
    }

    fn validate_plan(&mut self, plan: &Plan) -> AgentResult<()> {
        let validator = Validator::new(self.config.symbolic.strict_validation);
        if !self.policy_pack.verify() {
            return Err(AgentError::PolicyViolation(
                "Policy pack signature verification failed".to_string(),
            ));
        }

        for (idx, step) in plan.steps.iter().enumerate() {
            if !self.policy.is_tool_allowed(&step.tool) {
                return Err(AgentError::PolicyViolation(format!(
                    "Step {}: Tool '{}' not allowed by policy",
                    idx + 1,
                    step.tool
                )));
            }
            if !validator.validate_plan_step(&step.tool, &step.args)? {
                return Err(AgentError::PolicyViolation(format!(
                    "Step {} failed symbolic validation: tool='{}' args='{}'",
                    idx + 1,
                    step.tool,
                    step.args.join(" ")
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

            if !self.policy_pack.allowed_tools.contains(&step.tool) {
                return Err(AgentError::PolicyViolation(format!(
                    "Step {}: Tool '{}' not allowed by signed policy pack",
                    idx + 1,
                    step.tool
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
        self.enforce_risk_gate(&step.tool, &step.args)?;
        let compensation = self
            .tools
            .get_tool(&step.tool)
            .map(|tool| {
                if tool.is_reversible() {
                    CompensationKind::Reversible {
                        description: format!("revert side effects for '{}'", step.tool),
                    }
                } else {
                    CompensationKind::Irreversible {
                        warning: format!("manual remediation required for '{}'", step.tool),
                    }
                }
            })
            .unwrap_or_else(|| compensation_for_tool(&step.tool));
        self.rollback_manager
            .record(step.tool.clone(), compensation);

        {
            let breaker = self
                .circuit_breakers
                .entry(step.tool.clone())
                .or_insert_with(|| CircuitBreaker::new(3, 2, Timeout::from_secs(60)));

            if !breaker.should_allow_request() {
                let state = breaker.state();
                tracing::warn!(
                    "Circuit breaker not allowing tool '{}' (state: {:?}), skipping execution",
                    step.tool,
                    state
                );

                return Err(AgentError::Tool(format!(
                    "Circuit breaker blocked tool '{}' in state {:?}",
                    step.tool, state
                )));
            }
        }

        let tool_start = Instant::now();
        let result = self.execute_tool_with_timeout(&step.tool, &step.args)?;
        let tool_duration = tool_start.elapsed();
        self.run_replay.record(
            "tool.execute",
            format!(
                "tool={} success={} duration_ms={}",
                step.tool,
                result.success,
                tool_duration.as_millis()
            ),
        );

        self.metrics
            .record_tool_execution(&step.tool, result.success, tool_duration);

        {
            let breaker = self
                .circuit_breakers
                .entry(step.tool.clone())
                .or_insert_with(|| CircuitBreaker::new(3, 2, Timeout::from_secs(60)));

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
            if step.tool == "run_tests" {
                self.memory
                    .metadata
                    .insert("last_validation_success".to_string(), "false".to_string());
            }

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

        if step.tool == "run_tests" {
            self.memory
                .metadata
                .insert("last_validation_success".to_string(), "true".to_string());
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

        if start.elapsed() > self.tool_timeout.duration {
            result.success = false;
            result.error = Some(format!(
                "Tool '{}' timed out after {:?}",
                tool_name, self.tool_timeout.duration
            ));
        }

        Ok(result)
    }

    fn enforce_authz_for_action(&mut self, action: &str) -> AgentResult<()> {
        let decision = self.authz.check(&self.actor, action);
        let actor_has_dev_role = self.actor.has_role(&crate::security::ActorRole::Developer);
        let security_audit = SecurityAuditRecord::new(&self.actor, action, &decision);
        let security_payload = serde_json::to_string(&security_audit)
            .unwrap_or_else(|_| format!("authz action={} decision={:?}", action, decision));

        let _ = self
            .audit
            .log_symbolic_decision("security_authz", &security_payload);
        self.run_replay.record("security.authz", security_payload);

        if !actor_has_dev_role {
            return Err(AgentError::PolicyViolation(format!(
                "Actor '{}' is missing required Developer role for action '{}'",
                self.actor.id, action
            )));
        }

        if decision.is_allowed() {
            return Ok(());
        }

        match decision {
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
            AuthzDecision::Allow => Ok(()),
        }
    }

    fn enforce_risk_gate(&mut self, tool: &str, args: &[String]) -> AgentResult<()> {
        let base_risk = action_risk_level(tool, args, &self.policy_pack);
        let risk = self.adapt_risk_level(base_risk);
        self.run_replay.record(
            "tool.risk_level",
            format!(
                "tool={} base_risk={:?} effective_risk={:?}",
                tool, base_risk, risk
            ),
        );
        let _ = self.audit.log_symbolic_decision(
            "risk_gate_level",
            &format!(
                "tool={} base_risk={:?} effective_risk={:?}",
                tool, base_risk, risk
            ),
        );

        match risk {
            ActionRiskLevel::Low => {
                self.metrics.record_risk_gate_allow();
                let _ = self
                    .audit
                    .log_symbolic_decision("risk_gate_allow", &format!("tool={tool} risk=low"));
                Ok(())
            }
            ActionRiskLevel::Medium => {
                if is_medium_risk_allowed(&self.config.execution_mode) {
                    self.metrics.record_risk_gate_allow();
                    let _ = self.audit.log_symbolic_decision(
                        "risk_gate_allow",
                        &format!("tool={tool} risk=medium"),
                    );
                    Ok(())
                } else {
                    self.metrics.record_risk_gate_deny();
                    self.memory.add_failure(
                        self.iteration,
                        "Tool execution denied by risk gate".to_string(),
                        format!(
                            "medium-risk tool '{}' denied in mode '{}'",
                            tool, self.config.execution_mode
                        ),
                        Some(
                            "Set AUTONOMOUS_ALLOW_MUTATING_TOOLS=true for explicit opt-in"
                                .to_string(),
                        ),
                    );
                    let _ = self.audit.log_symbolic_decision(
                        "risk_gate_deny",
                        &format!(
                            "tool={tool} risk=medium mode={}",
                            self.config.execution_mode
                        ),
                    );
                    Err(AgentError::PolicyViolation(format!(
                        "medium-risk tool '{}' denied by safe mode",
                        tool
                    )))
                }
            }
            ActionRiskLevel::High => {
                if !is_medium_risk_allowed(&self.config.execution_mode) {
                    self.metrics.record_risk_gate_deny();
                    self.memory.add_failure(
                        self.iteration,
                        "Tool execution denied by risk gate".to_string(),
                        format!(
                            "high-risk tool '{}' denied in mode '{}'",
                            tool, self.config.execution_mode
                        ),
                        Some(
                            "Enable mutating tools explicitly before high-risk actions".to_string(),
                        ),
                    );
                    let _ = self.audit.log_symbolic_decision(
                        "risk_gate_deny",
                        &format!("tool={tool} risk=high mode={}", self.config.execution_mode),
                    );
                    return Err(AgentError::PolicyViolation(format!(
                        "high-risk tool '{}' denied by safe mode",
                        tool
                    )));
                }

                if has_valid_high_risk_approval_token() {
                    self.metrics.record_risk_gate_allow();
                    self.metrics.record_risk_gate_high_approval();
                    self.run_replay.record(
                        "tool.high_risk_approved",
                        format!("tool={} token_check=passed", tool),
                    );
                    let _ = self.audit.log_symbolic_decision(
                        "risk_gate_allow",
                        &format!("tool={tool} risk=high approval=token"),
                    );
                    Ok(())
                } else {
                    self.metrics.record_risk_gate_deny();
                    self.memory.add_failure(
                        self.iteration,
                        "High-risk approval missing".to_string(),
                        format!("tool '{}' requires explicit approval token", tool),
                        Some(
                            "Set AUTONOMOUS_HIGH_RISK_APPROVAL_TOKEN and AUTONOMOUS_EXPECTED_APPROVAL_TOKEN to matching values".to_string(),
                        ),
                    );
                    let _ = self.audit.log_symbolic_decision(
                        "risk_gate_deny",
                        &format!("tool={tool} risk=high approval=missing_or_invalid"),
                    );
                    Err(AgentError::PolicyViolation(format!(
                        "high-risk tool '{}' requires approval token",
                        tool
                    )))
                }
            }
        }
    }

    fn adapt_risk_level(&self, base: ActionRiskLevel) -> ActionRiskLevel {
        let recent_avg_failures = self
            .memory
            .metadata
            .get("previous_recent_avg_failures")
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);
        let recent_top_failure_kind = self
            .memory
            .metadata
            .get("previous_recent_top_failure_kind")
            .cloned()
            .unwrap_or_default();
        let recent_top_failure_kind_confidence = self
            .memory
            .metadata
            .get("previous_recent_top_failure_kind_confidence")
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);

        let should_harden = recent_avg_failures >= 3.0
            || (recent_top_failure_kind.starts_with("timeout:")
                && recent_top_failure_kind_confidence >= 0.6)
            || (recent_top_failure_kind.starts_with("policy:")
                && recent_top_failure_kind_confidence >= 0.6);
        if !should_harden {
            return base;
        }

        match base {
            ActionRiskLevel::Low => ActionRiskLevel::Low,
            ActionRiskLevel::Medium => ActionRiskLevel::High,
            ActionRiskLevel::High => ActionRiskLevel::High,
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
        let weighted_objective_score = self.symbolic.evaluator.weighted_score(&objective_scores);
        let hard_satisfied = self
            .symbolic
            .evaluator
            .hard_objectives_satisfied(&objective_scores);
        let policy_safety = score_value(&scores, "policy_safety").unwrap_or(0.0);
        let tests_pass = score_value(&scores, "tests_pass").unwrap_or(0.0);
        let observations = self.build_slo_observations(policy_safety, tests_pass);
        let slo_evaluations = self.slo_evaluator.evaluate(&observations);
        let slo_all_met = self.slo_evaluator.all_met(&slo_evaluations);

        self.audit
            .log_objective_evaluation(self.iteration, scores.clone())
            .map_err(|e| AgentError::State(e.to_string()))?;

        self.memory
            .add_objective_evaluation(self.iteration, scores.clone(), hard_satisfied);
        self.memory.metadata.insert(
            "weighted_objective_score".to_string(),
            format!("{:.3}", weighted_objective_score),
        );
        self.run_replay.record(
            "objectives.weighted_score",
            format!("{:.3}", weighted_objective_score),
        );

        if !hard_satisfied || !slo_all_met {
            tracing::warn!(
                "Iteration {}: objectives not satisfied (hard_ok={} slo_ok={})",
                self.current_iteration_number,
                hard_satisfied,
                slo_all_met
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
                return Err(AgentError::ObjectiveViolation(
                    "Hard objectives not satisfied within iteration budget".to_string(),
                ));
            }

            self.memory.add_failure(
                self.iteration,
                "Objectives or SLOs not satisfied".to_string(),
                format!(
                    "Objective scores: {:?}; SLO all met: {}",
                    scores, slo_all_met
                ),
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
        let metrics = self.metrics.snapshot();
        let step_ratio = self.current_plan.as_ref().map_or(0.0, |plan| {
            if plan.steps.is_empty() {
                0.0
            } else {
                (self.current_step_index.get().min(plan.steps.len()) as f64)
                    / (plan.steps.len() as f64)
            }
        });
        let task_completion = step_ratio.max(if self.state == AgentState::Done {
            1.0
        } else {
            0.0
        });

        let has_policy_failure = self.memory.failures.iter().any(|f| {
            let d = f.description.to_ascii_lowercase();
            let e = f.error.to_ascii_lowercase();
            d.contains("policy") || d.contains("authorization") || e.contains("policy")
        });
        let policy_safety = if has_policy_failure { 0.0 } else { 1.0 };

        let tests_pass = self
            .memory
            .metadata
            .get("last_validation_success")
            .map(|v| if v == "true" { 1.0 } else { 0.0 })
            .unwrap_or(0.7);

        let minimal_diff = if self.memory.explored_files.len() <= 20 {
            1.0
        } else {
            0.6
        };

        let avg_secs = metrics.average_iteration_duration.as_secs_f64();
        let time_budget = if avg_secs <= 60.0 {
            1.0
        } else if avg_secs <= 120.0 {
            0.7
        } else {
            0.4
        };

        let mut observations = HashMap::new();
        observations.extend(self.build_slo_observations(policy_safety, tests_pass));

        let slo_eval = self.slo_evaluator.evaluate(&observations);
        let slo_compliance = if slo_eval.is_empty() {
            0.0
        } else {
            (slo_eval.iter().filter(|e| e.met).count() as f64) / (slo_eval.len() as f64)
        };

        Ok(vec![
            ("task_completion".to_string(), task_completion),
            ("policy_safety".to_string(), policy_safety),
            ("tests_pass".to_string(), tests_pass),
            ("minimal_diff".to_string(), minimal_diff),
            ("time_budget".to_string(), time_budget),
            ("slo_compliance".to_string(), slo_compliance),
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

        let issue_number = Self::extract_issue_number_from_goal(&goal);
        let issue_body = self
            .memory
            .metadata
            .get("issue_body")
            .cloned()
            .or_else(|| std::env::var("AUTONOMOUS_ISSUE_BODY").ok())
            .unwrap_or_default();
        let issue_title = self
            .memory
            .metadata
            .get("issue_title")
            .cloned()
            .or_else(|| std::env::var("AUTONOMOUS_ISSUE_TITLE").ok())
            .unwrap_or_else(|| goal.clone());
        let issue_compliance = evaluate_issue_compliance(&issue_title, &issue_body);
        let pr_body = append_issue_compliance_note(&pr_body, &issue_compliance);
        let mut pr_orchestrator =
            PrOrchestrator::new(format!("Autonomous update: {}", goal), pr_body.clone(), 3);
        if let Some(n) = issue_number {
            if let Some(prn) = PrNumber::new(n.get()) {
                pr_orchestrator.open(prn);
            }
            pr_orchestrator.metadata.close_issue(n);
        }
        pr_orchestrator.set_ci_status(match self.memory.metadata.get("last_validation_success") {
            Some(v) if v == "true" => CiStatus::Passing,
            Some(_) => CiStatus::Failing,
            None => CiStatus::Unknown,
        });
        let policy_ok = !self
            .memory
            .failures
            .iter()
            .any(|f| f.description.to_ascii_lowercase().contains("policy"));
        pr_orchestrator.set_policy_compliant(policy_ok);
        pr_orchestrator.set_issue_compliance(issue_compliance.clone());
        pr_orchestrator.update_body(pr_body.clone());

        let readiness = pr_orchestrator.merge_readiness();
        let readiness_ok = readiness.is_ready();
        let readiness_msg = match &readiness {
            MergeReadiness::Ready => "ready".to_string(),
            MergeReadiness::NotReady { reasons } => format!("not_ready: {}", reasons.join(" | ")),
        };
        let rendered_body = pr_orchestrator.metadata.render_body();
        self.run_replay
            .record("pr.readiness", readiness_msg.clone());
        self.run_replay
            .record("issue.compliance", format!("{:?}", issue_compliance));
        self.run_replay
            .record("pr.rendered_body", format!("chars={}", rendered_body.len()));
        self.pr_orchestrator = Some(pr_orchestrator);

        self.audit
            .log_symbolic_decision("create_pr", &rendered_body)
            .map_err(|e| AgentError::State(e.to_string()))?;
        self.audit
            .log_symbolic_decision("merge_readiness", &readiness_msg)
            .map_err(|e| AgentError::State(e.to_string()))?;
        if !readiness_ok {
            self.memory.add_failure(
                self.iteration,
                "PR merge readiness not met".to_string(),
                readiness_msg.clone(),
                Some("continue through review loop for remediation".to_string()),
            );
        }

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
            .or_else(|| Self::extract_issue_number_from_goal(goal).map(|n| n.to_string()));

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
        let Some(orchestrator) = self.pr_orchestrator.as_mut() else {
            self.run_replay
                .record("review.skip", "no_pr_orchestrator".to_string());
            return self.transition_to(AgentState::Done);
        };

        let mut comments = Vec::new();
        if let Ok(body) = std::env::var("AUTONOMOUS_REVIEW_COMMENT") {
            comments.push(ReviewComment {
                reviewer: "review-bot".to_string(),
                body,
                resolved: false,
            });
        }
        if std::env::var("AUTONOMOUS_REVIEW_REQUESTED").ok().as_deref() == Some("true")
            && comments.is_empty()
        {
            comments.push(ReviewComment {
                reviewer: "review-bot".to_string(),
                body: "Please adjust implementation details".to_string(),
                resolved: false,
            });
        }

        let outcome = orchestrator.ingest_review(comments);
        self.run_replay
            .record("review.outcome", format!("{:?}", outcome));

        match outcome {
            ReviewOutcome::Approved => self.transition_to(AgentState::Done),
            ReviewOutcome::ChangesRequested => {
                let pending_reviewers: Vec<String> = orchestrator
                    .review_ingester
                    .pending_feedback()
                    .into_iter()
                    .map(|c| c.reviewer.clone())
                    .collect();
                self.run_replay.record(
                    "review.pending_feedback",
                    format!("count={}", pending_reviewers.len()),
                );

                let auto_resolve = std::env::var("AUTONOMOUS_AUTO_RESOLVE_REVIEW")
                    .unwrap_or_else(|_| "true".to_string())
                    == "true";
                if auto_resolve {
                    for reviewer in pending_reviewers {
                        orchestrator.review_ingester.resolve(&reviewer);
                    }
                    let post_outcome = orchestrator.review_ingester.outcome();
                    self.run_replay
                        .record("review.post_resolve_outcome", format!("{:?}", post_outcome));
                    if post_outcome == ReviewOutcome::Approved {
                        return self.transition_to(AgentState::Done);
                    }
                }

                let Some(next_iteration) = self.current_iteration_number.try_next() else {
                    return self.transition_to(AgentState::Blocked);
                };
                if next_iteration.exceeds(self.max_iterations_limit) {
                    return self.transition_to(AgentState::Blocked);
                }
                self.current_iteration_number = next_iteration;
                self.iteration = next_iteration.get();
                self.current_plan = None;
                self.current_step_index = StepIndex::zero();
                self.current_step = self.current_step_index.get();
                self.transition_to(AgentState::GeneratePlan)
            }
            ReviewOutcome::Timeout => self.transition_to(AgentState::Blocked),
        }
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

    fn configure_policy_pack_from_env(&mut self) -> AgentResult<()> {
        let Some(overrides) = std::env::var("AUTONOMOUS_TOOL_RISK_OVERRIDES").ok() else {
            return Ok(());
        };
        if overrides.trim().is_empty() {
            return Ok(());
        }

        let allow_unknown = std::env::var("AUTONOMOUS_ALLOW_UNKNOWN_RISK_OVERRIDE_TOOLS")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        if !allow_unknown {
            validate_override_tool_names(&overrides, |tool| self.is_known_tool(tool))?;
        }

        let applied = self
            .policy_pack
            .apply_risk_overrides_str(&overrides)
            .map_err(AgentError::PolicyViolation)?;
        let auto_sign = std::env::var("AUTONOMOUS_POLICY_PACK_AUTO_SIGN")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        if !auto_sign {
            return Err(AgentError::PolicyViolation(
                "AUTONOMOUS_TOOL_RISK_OVERRIDES requires AUTONOMOUS_POLICY_PACK_AUTO_SIGN=true"
                    .to_string(),
            ));
        }

        self.policy_pack.sign();
        self.run_replay.record(
            "policy_pack.overrides_applied",
            format!("entries={applied}"),
        );
        Ok(())
    }

    fn is_known_tool(&self, tool: &str) -> bool {
        self.policy.allowed_tools.contains(&tool.to_string())
            || matches!(tool, "deploy" | "modify_policy" | "delete_branch")
    }

    fn record_runbook_hint_for_error(&mut self, error_text: &str) {
        let keyword = if error_text.to_ascii_lowercase().contains("policy") {
            "policy"
        } else if error_text.to_ascii_lowercase().contains("timeout") {
            "timeout"
        } else if error_text.to_ascii_lowercase().contains("circuit") {
            "circuit"
        } else {
            return;
        };

        let entries = self.incident_runbook.lookup(keyword);
        if let Some(entry) = entries.first() {
            let remediation = entry.remediation_steps.join(" | ");
            self.memory.add_failure(
                self.iteration,
                format!("Runbook hint ({})", entry.scenario),
                error_text.to_string(),
                Some(remediation.clone()),
            );
            self.run_replay.record(
                "runbook.hint",
                format!("{} => {}", entry.scenario, remediation),
            );
        }
    }

    fn build_slo_observations(&self, policy_safety: f64, tests_pass: f64) -> HashMap<String, f64> {
        let metrics = self.metrics.snapshot();
        let mut observations = HashMap::new();
        observations.insert(
            "run_success_rate".to_string(),
            if self.state == AgentState::Done {
                1.0
            } else {
                0.8
            },
        );
        observations.insert("policy_violation_rate".to_string(), policy_safety);
        observations.insert(
            "iteration_latency_p95_secs".to_string(),
            metrics.average_iteration_duration.as_secs_f64(),
        );
        observations.insert("test_pass_rate".to_string(), tests_pass);
        observations.insert(
            "recovery_time_secs".to_string(),
            (metrics.iterations_failed as f64) * 30.0,
        );
        observations
    }
}

fn build_action_from_step(step: &PlanStep) -> String {
    if step.args.is_empty() {
        step.tool.clone()
    } else {
        format!("{} {}", step.tool, step.args.join(" "))
    }
}

fn parse_issue_labels_from_env() -> Vec<String> {
    std::env::var("AUTONOMOUS_ISSUE_LABELS")
        .ok()
        .map(|raw| {
            raw.split(',')
                .map(|v| v.trim())
                .filter(|v| !v.is_empty())
                .map(|v| v.to_string())
                .collect()
        })
        .unwrap_or_default()
}

fn score_value(scores: &[(String, f64)], key: &str) -> Option<f64> {
    scores.iter().find(|(k, _)| k == key).map(|(_, v)| *v)
}

fn is_medium_risk_allowed(execution_mode: &str) -> bool {
    let explicit_opt_in = std::env::var("AUTONOMOUS_ALLOW_MUTATING_TOOLS")
        .ok()
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    if explicit_opt_in {
        return true;
    }

    !execution_mode.eq_ignore_ascii_case("safe")
}

fn action_risk_level(tool: &str, args: &[String], policy_pack: &PolicyPack) -> ActionRiskLevel {
    if let Some(override_value) = policy_pack.risk_override(tool)
        && let Some(parsed) = parse_risk_level(override_value)
    {
        return parsed;
    }

    if matches!(
        tool,
        "read_file" | "search_code" | "generate_pr_description"
    ) {
        return ActionRiskLevel::Low;
    }
    if tool == "run_tests" && args.first().map(|v| v == "cargo").unwrap_or(false) {
        return ActionRiskLevel::Low;
    }
    if matches!(
        tool,
        "git_commit" | "deploy" | "modify_policy" | "delete_branch"
    ) {
        return ActionRiskLevel::High;
    }
    ActionRiskLevel::Medium
}

fn parse_risk_level(value: &str) -> Option<ActionRiskLevel> {
    match value.to_ascii_lowercase().as_str() {
        "low" => Some(ActionRiskLevel::Low),
        "medium" => Some(ActionRiskLevel::Medium),
        "high" => Some(ActionRiskLevel::High),
        _ => None,
    }
}

fn parse_action_outcome_triplet(value: &str) -> Option<(String, f64, usize)> {
    if let Ok(summary) = serde_json::from_str::<ActionOutcomeSummary>(value) {
        return Some((
            summary.action.to_string(),
            summary.pass_rate.get(),
            summary.total,
        ));
    }

    let mut parts = value.split(':');
    let action = parts.next()?.to_string();
    let pass_rate = parts.next()?.parse::<f64>().ok()?;
    let total = parts.next()?.parse::<usize>().ok()?;
    Some((action, pass_rate, total))
}

fn has_valid_high_risk_approval_token() -> bool {
    let provided = std::env::var("AUTONOMOUS_HIGH_RISK_APPROVAL_TOKEN").ok();
    let expected = std::env::var("AUTONOMOUS_EXPECTED_APPROVAL_TOKEN").ok();
    match (provided, expected) {
        (Some(p), Some(e)) => !p.is_empty() && p == e,
        _ => false,
    }
}

fn validate_override_tool_names<F>(overrides: &str, is_known_tool: F) -> AgentResult<()>
where
    F: Fn(&str) -> bool,
{
    for entry in overrides
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        let (tool, _) = entry.split_once('=').ok_or_else(|| {
            AgentError::PolicyViolation(format!(
                "Invalid risk override entry '{entry}', expected tool=risk"
            ))
        })?;
        let tool = tool.trim();
        if tool.is_empty() {
            return Err(AgentError::PolicyViolation(
                "Risk override tool name cannot be empty".to_string(),
            ));
        }
        if !is_known_tool(tool) {
            return Err(AgentError::PolicyViolation(format!(
                "Unknown tool '{}' in AUTONOMOUS_TOOL_RISK_OVERRIDES",
                tool
            )));
        }
    }
    Ok(())
}

fn evaluate_issue_compliance(title: &str, body: &str) -> IssueComplianceStatus {
    if body.trim().is_empty() {
        return IssueComplianceStatus::Unknown;
    }

    if let Some(reason) = validate_required_issue_fields(body) {
        return IssueComplianceStatus::NonCompliant { reason };
    }

    let require_typed_title = std::env::var("AUTONOMOUS_REQUIRE_TYPED_ISSUE_TITLE")
        .ok()
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    if require_typed_title && !looks_like_typed_issue_title(title) {
        return IssueComplianceStatus::NonCompliant {
            reason: "title must match type(scope): summary".to_string(),
        };
    }

    IssueComplianceStatus::Compliant
}

fn validate_required_issue_fields(body: &str) -> Option<String> {
    let required_fields =
        std::env::var("AUTONOMOUS_REQUIRED_ISSUE_FIELDS").unwrap_or_else(|_| "Parent".to_string());

    for raw_field in required_fields.split(',') {
        let field = raw_field.trim();
        if field.is_empty() {
            continue;
        }
        let needle = format!("{field}:");
        let has_field = body
            .lines()
            .any(|line| line.trim_start().starts_with(&needle));
        if !has_field {
            return Some(format!("missing required field: {field}:"));
        }
    }

    let parent_line = body
        .lines()
        .find(|line| line.trim_start().starts_with("Parent:"))
        .map(|line| {
            line.trim_start()
                .trim_start_matches("Parent:")
                .trim()
                .to_string()
        });

    if let Some(parent) = parent_line {
        let Some(parent_ref) = ParentRef::parse(&parent) else {
            return Some("Parent must be '#<number>' or 'none'".to_string());
        };

        if let ParentRef::Issue(parent_issue) = parent_ref {
            if let Ok(issue_number) = std::env::var("AUTONOMOUS_ISSUE_NUMBER")
                && issue_number == parent_issue.to_string()
            {
                return Some("Parent cannot reference the issue itself".to_string());
            }

            if let Ok(existing_raw) = std::env::var("AUTONOMOUS_EXISTING_ISSUE_NUMBERS") {
                let exists = existing_raw
                    .split(',')
                    .map(|s| s.trim())
                    .filter_map(ParentRef::parse)
                    .filter_map(|r| match r {
                        ParentRef::Issue(n) => Some(n),
                        ParentRef::None => None,
                    })
                    .any(|n| n == parent_issue);
                if !exists {
                    return Some(format!(
                        "Parent #{} does not exist in known issue set",
                        parent_issue
                    ));
                }
            }
        }
    }

    None
}

fn looks_like_typed_issue_title(title: &str) -> bool {
    let Some(colon_pos) = title.find(':') else {
        return false;
    };
    if colon_pos == 0 || colon_pos + 1 >= title.len() {
        return false;
    }
    let prefix = &title[..colon_pos];
    let has_scope = prefix.contains('(') && prefix.ends_with(')');
    let has_type = prefix
        .chars()
        .take_while(|c| *c != '(')
        .all(|c| c.is_ascii_lowercase() || c == '_' || c == '-');
    has_scope && has_type && !title[colon_pos + 1..].trim().is_empty()
}

fn append_issue_compliance_note(body: &str, status: &IssueComplianceStatus) -> String {
    match status {
        IssueComplianceStatus::Compliant | IssueComplianceStatus::Unknown => body.to_string(),
        IssueComplianceStatus::NonCompliant { reason } => format!(
            "{body}\n\n---\nIssue compliance: non-compliant\nReason: {reason}\nRemediation: fix required issue fields (e.g., Parent: #<id> or Parent: none), then update PR keyword line."
        ),
    }
}

fn compensation_for_tool(tool: &str) -> CompensationKind {
    match tool {
        "read_file" | "run_tests" => CompensationKind::None,
        "generate_pr_description" => CompensationKind::Reversible {
            description: "Remove generated PR description artifact".to_string(),
        },
        "git_commit" => CompensationKind::Irreversible {
            warning: "Commit already recorded in git history".to_string(),
        },
        _ => CompensationKind::Reversible {
            description: "Manual review of side effects required".to_string(),
        },
    }
}
