//projects/products/unstable/autonomous_dev_ai/src/lifecycle/lifecycle_manager.rs
// Agent lifecycle implementation - production-grade flow.
use super::{
    Checkpoint, CircuitBreaker, CompensationKind, ExecutionContext, IterationNumber,
    LifecycleError, LifecycleMetrics, LifecycleResult, MaxIterations, MetricsCollector,
    ResourceBudget, ResourceType, RetryStrategy, RollbackManager, StepIndex,
    validation_command_plan::select_validation_command,
};

use crate::audit_logger::AuditLogger;
use crate::error::{AgentError, AgentResult};
use crate::ids::{IssueNumber, PrNumber};
use crate::lifecycle::{IterationManager, LearningContext};
use crate::memory::FailureEntry;
use crate::memory_graph::MemoryGraph;
use crate::models::{AgentConfig, Version};
use crate::neural::{
    DriftDetector, IntentInterpretation, ModelEvaluationSnapshot, ModelGovernance, ModelVersion,
    NeuralLayer, NeuralModel,
};
use crate::objective_evaluator::ObjectiveEvaluator;
use crate::ops::{IncidentRunbook, RunReplay, SloEvaluator};
use crate::parsing::parse_review_comments_from_gh_json;
use crate::path_types::CheckpointPath;
use crate::persistence::Artifacts;
use crate::pr_flow::{CiStatus, PrOrchestrator, ReviewComment, ReviewOutcome};
use crate::runtime_lock::RuntimeLockGuard;
use crate::security::{ActorIdentity, AuthzDecision, AuthzEngine, PolicyPack, SecurityAuditRecord};
use crate::state::AgentState;
use crate::symbolic::{
    IssueClassificationInput, Plan, PlanStep, PolicyEngine, SymbolicController, Validator,
};
use crate::timeout::Timeout;
use crate::tools::{
    GitWrapper, PrDescriptionGenerator, RepoReader, TestRunner, ToolMetricSnapshot, ToolRegistry,
    ToolResult,
};
use crate::value_types::{ActionName, StateLabel};

use std::collections::HashMap;
use std::process::{self, Command};
use std::time::{Duration, Instant};
use std::{env, fs, thread};

/// Agent lifecycle manager.
pub struct LifecycleManager<'a> {
    // Public fields preserved for compatibility with existing callers/tests.
    pub(crate) state: AgentState,
    pub(crate) memory: MemoryGraph,
    pub(crate) symbolic: SymbolicController,
    pub(crate) model_governance: ModelGovernance,
    pub(crate) policy: PolicyEngine,
    pub(crate) authz: AuthzEngine,
    pub(crate) pr_orchestrator: Option<PrOrchestrator<'a>>,
    pub(crate) tools: ToolRegistry,
    pub(crate) audit: AuditLogger,
    pub(crate) iteration: usize,
    pub(crate) current_plan: Option<Plan>,
    pub(crate) current_step: usize,
    pub(crate) max_iterations: usize,

    // Typed execution state.
    pub(crate) current_step_index: StepIndex,
    pub(crate) execution_context: Option<ExecutionContext>,

    // Resilience and observability.
    pub(crate) circuit_breakers: HashMap<String, CircuitBreaker>,
    pub(crate) retry_strategy: RetryStrategy,
    pub(crate) metrics: MetricsCollector,
    pub(crate) slo_evaluator: SloEvaluator,
    pub(crate) incident_runbook: IncidentRunbook,
    pub(crate) policy_pack: PolicyPack,
    pub(crate) resource_budget: ResourceBudget,
    pub(crate) rollback_manager: RollbackManager,
    pub(crate) last_intent: Option<IntentInterpretation>,
    pub(crate) drift_detector: DriftDetector,
    pub(crate) neural_eval_source: String,
    pub(crate) active_neural_model_name: String,

    // Timeouts.
    pub(crate) global_timeout: Timeout,
    pub(crate) iteration_timeout: Timeout,
    pub(crate) tool_timeout: Timeout,
    pub(crate) iteration_manager: IterationManager,
    pub(crate) artifacts: Artifacts,
}

impl<'a> LifecycleManager<'a> {
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
        let active_neural_model_name = std::env::var("AUTONOMOUS_NEURAL_MODEL_NAME")
            .ok()
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| "default-neural".to_string());
        model_governance.registry.register(ModelVersion::new(
            active_neural_model_name.clone(),
            Version::new(0, 1, 0),
            "builtin://heuristic",
            0.7,
        ));
        let offline_min = env_f64_or("AUTONOMOUS_NEURAL_OFFLINE_MIN_SCORE", 0.8);
        let online_min = env_f64_or("AUTONOMOUS_NEURAL_ONLINE_MIN_SCORE", 0.85);
        let confidence_min = env_f64_or("AUTONOMOUS_NEURAL_MIN_CONFIDENCE", 0.7);
        model_governance.offline_eval_min_score = offline_min;
        model_governance.online_eval_min_score = online_min;
        model_governance.confidence_gate.min_confidence = confidence_min;

        let mut neural_eval_source = "env".to_string();
        let mut offline_score = env_f64_or("AUTONOMOUS_NEURAL_OFFLINE_SCORE", 1.0);
        let mut online_score = env_f64_or("AUTONOMOUS_NEURAL_ONLINE_SCORE", 1.0);
        if let Ok(path) = env::var("AUTONOMOUS_NEURAL_EVAL_FILE")
            && !path.trim().is_empty()
        {
            match ModelEvaluationSnapshot::load_scores_for_model(&path, &active_neural_model_name) {
                Ok(Some((offline, online))) => {
                    offline_score = offline;
                    online_score = online;
                    neural_eval_source = format!("file:{path}");
                }
                Ok(None) => {
                    tracing::warn!(
                        "No neural evaluation snapshot found for model '{}' in '{}', using env scores",
                        active_neural_model_name,
                        path
                    );
                }
                Err(err) => {
                    tracing::warn!(
                        "Failed to load AUTONOMOUS_NEURAL_EVAL_FILE '{}': {}. Falling back to env scores.",
                        path,
                        err
                    );
                }
            }
        }

        let _ = model_governance.evaluate_offline(&active_neural_model_name, offline_score);
        let _ = model_governance.promote_after_offline_gate(&active_neural_model_name);

        let _ = model_governance.evaluate_online(&active_neural_model_name, online_score);
        let _ = model_governance.promote_after_online_gate(&active_neural_model_name);

        let policy = PolicyEngine::new();
        let authz = AuthzEngine::new();
        let actor = ActorIdentity::from_env_or_default();
        let run_replay = RunReplay::new(actor.run_id.clone());
        let slo_evaluator = SloEvaluator::new(SloEvaluator::default_slos());
        let incident_runbook = IncidentRunbook::default_runbook();
        let policy_pack = PolicyPack::default();
        let max_cpu_seconds = env_u64_or(
            "AUTONOMOUS_MAX_CPU_SECONDS",
            global_timeout.duration.as_secs(),
        );
        let max_rss_mb = env_u64_or("AUTONOMOUS_MAX_RSS_MB", 2048);
        let max_rss_bytes = usize::try_from(max_rss_mb)
            .ok()
            .and_then(|mb| mb.checked_mul(1024 * 1024))
            .unwrap_or(usize::MAX);
        let resource_budget = ResourceBudget::new(
            global_timeout.duration,
            Duration::from_secs(max_cpu_seconds),
            max_rss_bytes,
            max_iterations_limit.get(),
            500,
        );
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

        let iteration_manager = IterationManager::new(IterationNumber::first(), StepIndex::zero());

        Self {
            state: AgentState::Idle,
            memory: MemoryGraph::new(),
            symbolic,
            model_governance,
            policy,
            authz,
            pr_orchestrator: None,
            tools,
            audit: AuditLogger::new(audit_log_path),
            iteration: IterationNumber::first().get(),
            current_plan: None,
            current_step: StepIndex::zero().get(),
            max_iterations: max_iterations_limit.get(),
            current_step_index: StepIndex::zero(),
            execution_context: None,
            circuit_breakers: HashMap::new(),
            retry_strategy: RetryStrategy::default()
                .with_delays(Duration::from_millis(200), Duration::from_secs(5)),
            metrics: MetricsCollector::new(),
            slo_evaluator,
            incident_runbook,
            policy_pack,
            resource_budget,
            rollback_manager,
            last_intent: None,
            drift_detector,
            neural_eval_source,
            active_neural_model_name,
            global_timeout,
            iteration_timeout: Timeout::from_secs(300),
            tool_timeout: Timeout::from_secs(30),
            iteration_manager,
            artifacts: Artifacts {
                run_replay,
                memory: MemoryGraph::default(),
                metrics: MetricsCollector::new(),
                state: "initial".to_string(),
                actor: actor.clone(),
                config: config.clone(),
                neural: NeuralLayer::new(
                    config.neural.enabled,
                    config.neural.prefer_gpu,
                    config.neural.cpu_fallback,
                ),
                current_iteration_number: IterationNumber::first(),
                max_iterations_limit,
                checkpoint_path: checkpoint_path.clone(),
            },
        }
    }

    /// Run the lifecycle with typed errors, retries, and metrics.
    pub fn run(&mut self, goal: &str) -> LifecycleResult<()> {
        let start_time = Instant::now();
        let runtime_lock_path = env::var("AUTONOMOUS_RUNTIME_LOCK_PATH")
            .ok()
            .filter(|v| !v.trim().is_empty())
            .unwrap_or_else(|| format!("{}.runtime.lock", self.artifacts.checkpoint_path.as_str()));
        let run_id = self.artifacts.actor.run_id.to_string();
        let _runtime_lock =
            RuntimeLockGuard::acquire(&runtime_lock_path, &run_id).map_err(|e| {
                LifecycleError::Fatal {
                    iteration: 0,
                    error: AgentError::State(format!(
                        "Failed to acquire runtime lock '{}': {}",
                        runtime_lock_path, e
                    )),
                    context: "Runtime lock acquisition failed".to_string(),
                }
            })?;

        self.artifacts.run_replay = RunReplay::new(self.artifacts.actor.run_id.clone());
        let checkpoint_path = self.artifacts.checkpoint_path.clone();
        let config = self.artifacts.config.clone();
        let resumed_from_checkpoint = self
            .memory
            .metadata
            .get("checkpoint.loaded")
            .map(|v| v == "true")
            .unwrap_or(false);
        if !resumed_from_checkpoint {
            self.set_iteration_number(IterationNumber::first());
        }
        self.current_plan = None;
        self.reset_step_index();
        self.pr_orchestrator = None;
        self.rollback_manager = RollbackManager::new();
        self.last_intent = None;
        self.drift_detector = DriftDetector::default();
        self.run_replay.record("lifecycle.start", goal);
        if resumed_from_checkpoint {
            let resumed_iteration = self
                .memory
                .metadata
                .get("checkpoint.loaded_iteration")
                .cloned()
                .unwrap_or_else(|| self.artifacts.current_iteration_number.get().to_string());
            self.artifacts.run_replay.record(
                "checkpoint.loaded",
                format!("resume=true iteration={}", resumed_iteration),
            );
        }
        self.artifacts.run_replay.record(
            "runtime.lock.acquired",
            _runtime_lock.path().display().to_string(),
        );
        self.memory.metadata.insert(
            "runtime_lock_path".to_string(),
            _runtime_lock.path().display().to_string(),
        );
        if let Some(hold_ms) = env::var("AUTONOMOUS_HOLD_LOCK_MS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .filter(|v| *v > 0)
        {
            thread::sleep(Duration::from_millis(hold_ms));
        }
        self.artifacts.run_replay.record(
            "lifecycle.checkpoint_path",
            self.artifacts.checkpoint_path.as_str(),
        );
        self.artifacts.run_replay.record(
            "symbolic.mode",
            format!(
                "strict_validation={} deterministic={}",
                self.symbolic.strict_validation, self.symbolic.deterministic
            ),
        );
        if let Some(state) = self
            .model_governance
            .registry
            .state(&self.active_neural_model_name)
        {
            self.artifacts
                .run_replay
                .record("neural.rollout_state", format!("{:?}", state));
        }
        self.artifacts.run_replay.record(
            "neural.rollout_gates",
            format!(
                "model={} offline_passed={} online_passed={}",
                self.active_neural_model_name,
                self.model_governance
                    .registry
                    .offline_gate_passed(&self.active_neural_model_name),
                self.model_governance
                    .registry
                    .online_gate_passed(&self.active_neural_model_name)
            ),
        );
        self.artifacts.run_replay.record(
            "neural.rollout_eval_source",
            self.neural_eval_source.clone(),
        );
        self.configure_policy_pack_from_env()
            .map_err(|e| LifecycleError::Fatal {
                iteration: 0,
                error: e,
                context: "Failed to configure policy pack from runtime overrides".to_string(),
            })?;
        self.validate_runtime_requirements()
            .map_err(|e| LifecycleError::Fatal {
                iteration: 0,
                error: e,
                context: "Failed runtime requirement validation".to_string(),
            })?;
        self.artifacts
            .run_replay
            .record("policy_pack.fingerprint", self.policy_pack.fingerprint());

        tracing::info!("=== Starting Agent Lifecycle ===");
        tracing::info!("Goal: {}", goal);
        tracing::info!("Max iterations: {}", self.artifacts.max_iterations_limit.get());
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

        self.artifacts.persist_run_replay_artifacts();
        self.artifacts.persist_run_report_artifact();

        tracing::info!("=== Agent Lifecycle Complete ===");
        tracing::info!("Final state: {:?}", self.state);
        tracing::info!("Total iterations: {}", self.iteration);
        tracing::info!("Total duration: {:?}", start_time.elapsed());
        let reversible = self.rollback_manager.reversible_actions().len();
        let irreversible = self.rollback_manager.irreversible_actions().len();
        self.artifacts.run_replay.record(
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
        match Checkpoint::load(&self.artifacts.checkpoint_path) {
            Ok(checkpoint) => {
                self.set_iteration_number(
                    IterationNumber::from_usize(checkpoint.iteration)
                        .unwrap_or_else(IterationNumber::first),
                );
                self.memory
                    .metadata
                    .insert("checkpoint.loaded".to_string(), "true".to_string());
                self.memory.metadata.insert(
                    "checkpoint.loaded_iteration".to_string(),
                    checkpoint.iteration.to_string(),
                );
                self.artifacts.run_replay.record(
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
                self.artifacts.checkpoint_path.as_str(),
                err
            ))),
        }
    }

    fn set_iteration_number(&mut self, value: IterationNumber) {
        self.artifacts.current_iteration_number = value;
        self.iteration = value.get();
    }

    fn execute_main_loop(&mut self, start_time: Instant) -> LifecycleResult<()> {
        let mut recoverable_attempts = 0usize;

        while !self.state.is_terminal() {
            self.check_resource_budgets(start_time)?;

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

    fn validate_runtime_requirements(&mut self) -> AgentResult<()> {
        let non_interactive_profile = env::var("AUTONOMOUS_NON_INTERACTIVE_PROFILE")
            .ok()
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty());
        let create_pr_enabled = env::var("AUTONOMOUS_CREATE_PR")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let create_pr_required = env::var("AUTONOMOUS_CREATE_PR_REQUIRED")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let require_real_pr_creation = env::var("AUTONOMOUS_REQUIRE_REAL_PR_CREATION")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let fetch_review_from_gh = env::var("AUTONOMOUS_FETCH_REVIEW_FROM_GH")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let require_gh_review_source = env::var("AUTONOMOUS_REQUIRE_GH_REVIEW_SOURCE")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let fetch_pr_ci_status = env::var("AUTONOMOUS_FETCH_PR_CI_STATUS_FROM_GH")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let fetch_pr_ci_status_required = env::var("AUTONOMOUS_FETCH_PR_CI_STATUS_REQUIRED")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let fetch_issue_context_from_gh = env::var("AUTONOMOUS_FETCH_ISSUE_CONTEXT_FROM_GH")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let fetch_issue_context_required = env::var("AUTONOMOUS_FETCH_ISSUE_CONTEXT_REQUIRED")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let require_issue_compliance = env::var("AUTONOMOUS_REQUIRE_ISSUE_COMPLIANCE")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let review_required = env::var("AUTONOMOUS_REVIEW_REQUIRED")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let require_pr_number = env::var("AUTONOMOUS_REQUIRE_PR_NUMBER")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        if create_pr_required && !create_pr_enabled {
            return Err(AgentError::State(
                "AUTONOMOUS_CREATE_PR_REQUIRED=true requires AUTONOMOUS_CREATE_PR=true".to_string(),
            ));
        }
        if require_real_pr_creation && !create_pr_enabled {
            return Err(AgentError::State(
                "AUTONOMOUS_REQUIRE_REAL_PR_CREATION=true requires AUTONOMOUS_CREATE_PR=true"
                    .to_string(),
            ));
        }
        if require_gh_review_source && !fetch_review_from_gh {
            return Err(AgentError::State(
                "AUTONOMOUS_REQUIRE_GH_REVIEW_SOURCE=true requires AUTONOMOUS_FETCH_REVIEW_FROM_GH=true"
                    .to_string(),
            ));
        }
        if fetch_pr_ci_status_required && !fetch_pr_ci_status {
            return Err(AgentError::State(
                "AUTONOMOUS_FETCH_PR_CI_STATUS_REQUIRED=true requires AUTONOMOUS_FETCH_PR_CI_STATUS_FROM_GH=true"
                    .to_string(),
            ));
        }
        if fetch_issue_context_required && !fetch_issue_context_from_gh {
            return Err(AgentError::State(
                "AUTONOMOUS_FETCH_ISSUE_CONTEXT_REQUIRED=true requires AUTONOMOUS_FETCH_ISSUE_CONTEXT_FROM_GH=true"
                    .to_string(),
            ));
        }
        if let Some(profile) = &non_interactive_profile {
            if profile != "orchestrator_v1" {
                return Err(AgentError::State(format!(
                    "Unsupported AUTONOMOUS_NON_INTERACTIVE_PROFILE='{}'. Supported profile: orchestrator_v1",
                    profile
                )));
            }
            require_env_non_empty("AUTONOMOUS_RUN_REPORT_PATH")?;
            require_env_non_empty("AUTONOMOUS_RUN_REPLAY_PATH")?;
            require_env_non_empty("AUTONOMOUS_RUN_REPLAY_TEXT_PATH")?;
            if !require_pr_number {
                return Err(AgentError::State(
                    "AUTONOMOUS_NON_INTERACTIVE_PROFILE=orchestrator_v1 requires AUTONOMOUS_REQUIRE_PR_NUMBER=true"
                        .to_string(),
                ));
            }
            if !fetch_pr_ci_status_required {
                return Err(AgentError::State(
                    "AUTONOMOUS_NON_INTERACTIVE_PROFILE=orchestrator_v1 requires AUTONOMOUS_FETCH_PR_CI_STATUS_REQUIRED=true"
                        .to_string(),
                ));
            }
            if !review_required {
                return Err(AgentError::State(
                    "AUTONOMOUS_NON_INTERACTIVE_PROFILE=orchestrator_v1 requires AUTONOMOUS_REVIEW_REQUIRED=true"
                        .to_string(),
                ));
            }
            if !require_issue_compliance {
                return Err(AgentError::State(
                    "AUTONOMOUS_NON_INTERACTIVE_PROFILE=orchestrator_v1 requires AUTONOMOUS_REQUIRE_ISSUE_COMPLIANCE=true"
                        .to_string(),
                ));
            }
        }

        self.artifacts.run_replay.record(
            "runtime.requirements",
            format!(
                "non_interactive_profile={:?} create_pr_enabled={} create_pr_required={} require_real_pr_creation={} fetch_review_from_gh={} require_gh_review_source={} fetch_pr_ci_status={} fetch_pr_ci_status_required={} fetch_issue_context_from_gh={} fetch_issue_context_required={} require_issue_compliance={} review_required={} require_pr_number={}",
                non_interactive_profile,
                create_pr_enabled,
                create_pr_required,
                require_real_pr_creation,
                fetch_review_from_gh,
                require_gh_review_source,
                fetch_pr_ci_status,
                fetch_pr_ci_status_required,
                fetch_issue_context_from_gh,
                fetch_issue_context_required,
                require_issue_compliance,
                review_required,
                require_pr_number
            ),
        );
        if let Some(profile) = non_interactive_profile {
            self.memory
                .metadata
                .insert("non_interactive_profile".to_string(), profile);
        }
        self.memory.metadata.insert(
            "runtime_requirements.validated".to_string(),
            "true".to_string(),
        );
        Ok(())
    }

    fn record_resource_budget_failure(&mut self, error: String, recovery: Option<String>) {
        self.memory.add_failure(
            self.iteration,
            "Resource budget exceeded".to_string(),
            error,
            recovery,
        );
    }

    fn transition_to_failed_state(&mut self) -> LifecycleResult<()> {
        self.transition_to(AgentState::Failed)
            .map_err(|e| LifecycleError::Fatal {
                iteration: self.iteration,
                error: e,
                context: "Failed to transition to Failed state".to_string(),
            })
    }

    fn check_iteration_budget(&mut self) -> LifecycleResult<()> {
        if !self
            .artifacts
            .current_iteration_number
            .exceeds(self.artifacts.max_iterations_limit)
        {
            return Ok(());
        }

        tracing::error!(
            "Maximum iterations exceeded: {} > {}",
            self.artifacts.current_iteration_number,
            self.artifacts.max_iterations_limit.get()
        );

        self.memory.add_failure(
            self.iteration,
            "Maximum iterations exceeded".to_string(),
            format!(
                "Agent exceeded maximum allowed iterations ({})",
                self.artifacts.max_iterations_limit.get()
            ),
            None,
        );

        self.transition_to_failed_state()?;

        Err(LifecycleError::ResourceExhausted {
            resource: ResourceType::Iterations,
            limit: self.artifacts.max_iterations_limit.get(),
            current: self.iteration,
        })
    }

    fn record_replay(&mut self, kind: &str, payload: impl Into<String>) {
        self.artifacts.run_replay.record(kind, payload);
    }

    fn log_symbolic_decision_safe(&self, decision: &str, details: &str) -> AgentResult<()> {
        self.audit
            .log_symbolic_decision(decision, details)
            .map_err(|e| AgentError::State(e.to_string()))
    }

    fn execute_current_state(&mut self) -> LifecycleResult<()> {
        self.execution_context = Some(ExecutionContext::new(
            self.artifacts.current_iteration_number,
            self.iteration_timeout,
        ));

        tracing::debug!(
            "Executing state {:?} (iteration {})",
            self.state,
            self.artifacts.current_iteration_number
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
                "Iteration {} timed out: {:?} > {:?} (remaining {:?}ms)",
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
            self.artifacts.current_iteration_number
        );

        self.audit
            .log_state_transition(&old_state, &new_state_label.to_string())
            .map_err(|e| AgentError::State(e.to_string()))?;
        self.artifacts.run_replay.record(
            "state.transition",
            format!("{old_state} -> {new_state_label}"),
        );

        self.metrics.record_state_transition();
        self.state = new_state;

        let checkpoint = Checkpoint::new(
            self.artifacts.actor.run_id.clone(),
            self.artifacts.current_iteration_number.get(),
            new_state_label,
        );
        if let Err(e) = checkpoint.save(&self.artifacts.checkpoint_path) {
            tracing::warn!(
                "Failed to write checkpoint '{}': {}",
                self.artifacts.checkpoint_path.as_str(),
                e
            );
        }
        Ok(())
    }

    fn explore_repository(&mut self) -> AgentResult<()> {
        self.check_timeout()?;

        tracing::info!(
            "Iteration {}: Exploring repository",
            self.artifacts.current_iteration_number
        );
        let repo_root = env::var("AUTONOMOUS_REPO_ROOT").unwrap_or_else(|_| ".".to_string());
        let max_entries = env::var("AUTONOMOUS_EXPLORE_MAX_ENTRIES")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(64);
        match list_repository_entries(&repo_root, max_entries) {
            Ok(entries) => {
                let require_explored_files = env::var("AUTONOMOUS_REQUIRE_EXPLORED_FILES")
                    .ok()
                    .map(|v| v.eq_ignore_ascii_case("true"))
                    .unwrap_or(false);
                if entries.is_empty() {
                    self.memory.add_failure(
                        self.iteration,
                        "Repository exploration returned no entries".to_string(),
                        format!("root='{}'", repo_root),
                        Some("Check AUTONOMOUS_REPO_ROOT path".to_string()),
                    );
                    if require_explored_files {
                        return Err(AgentError::State(format!(
                            "AUTONOMOUS_REQUIRE_EXPLORED_FILES=true but no entries were discovered in '{}'",
                            repo_root
                        )));
                    }
                }
                for entry in entries {
                    self.memory.add_explored_file(entry);
                }
            }
            Err(e) => {
                return Err(AgentError::State(format!(
                    "Failed to explore repository root '{}': {}",
                    repo_root, e
                )));
            }
        }

        self.transition_to(AgentState::GeneratePlan)
    }

    fn generate_plan(&mut self) -> AgentResult<()> {
        self.check_timeout()?;

        tracing::info!(
            "Iteration {}: Generating execution plan",
            self.artifacts.current_iteration_number
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
                format!(
                    "max_iterations={}",
                    self.artifacts.max_iterations_limit.get()
                ),
                format!("tool_timeout_secs={}", self.tool_timeout.as_secs()),
                format!("global_timeout_secs={}", self.global_timeout.as_secs()),
            ],
            confidence: if self.artifacts.neural.enabled {
                0.75
            } else {
                1.0
            },
        };
        self.last_intent = Some(intent.clone());
        self.memory.metadata.insert(
            "last_intent_confidence".to_string(),
            format!("{:.2}", intent.confidence),
        );
        self.artifacts.run_replay.record(
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
        self.artifacts.run_replay.record(
            "issue.classification",
            format!(
                "category={:?} source={:?} confidence={:.2}",
                issue_category.category, issue_category.source, issue_category.confidence
            ),
        );

        let neural_suggestion = if self.artifacts.neural.enabled {
            match self.artifacts.neural.infer(&goal) {
                Ok(suggested) => {
                    let uncertainty = self
                        .artifacts
                        .neural
                        .estimate_uncertainty(&goal)
                        .unwrap_or(0.5);
                    let cpu_fallback = self.artifacts.neural.use_cpu_fallback();
                    let model_confidence = self.artifacts.neural.confidence();
                    let detected_drift = self.drift_detector.observe(suggested.confidence);
                    let rolling_avg = self.drift_detector.rolling_average().unwrap_or(0.0);
                    self.artifacts.run_replay.record(
                            "neural.runtime",
                            format!(
                                "confidence={:.3} model_confidence={:.3} uncertainty={:.3} prefer_gpu={} cpu_fallback={} drift_detected={} rolling_avg={:.3}",
                                suggested.confidence,
                                model_confidence,
                                uncertainty,
                                self.artifacts.neural.prefer_gpu,
                                cpu_fallback,
                                detected_drift,
                                rolling_avg
                            ),
                        );

                    let governance_ok = self
                        .model_governance
                        .accept(&self.active_neural_model_name, suggested.confidence);
                    let validation = self.symbolic.validate_proposal(&suggested)?;
                    let rollout_state = self
                        .model_governance
                        .registry
                        .state(&self.active_neural_model_name)
                        .map(|s| format!("{:?}", s))
                        .unwrap_or_else(|| "Unknown".to_string());
                    let rollback_reason = self
                        .model_governance
                        .registry
                        .rollback_reason(&self.active_neural_model_name)
                        .unwrap_or("none");

                    self.artifacts.run_replay.record(
                        "neural.governance",
                        format!(
                            "accepted={} symbolic_valid={} rollout_state={} rollback_reason={} issues={}",
                            governance_ok,
                            validation.is_valid,
                            rollout_state,
                            rollback_reason,
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
        self.artifacts.run_replay.record(
            "plan.generated",
            format!(
                "iteration={} steps={}",
                self.artifacts.current_iteration_number,
                plan.steps.len()
            ),
        );

        let plan_description = format!(
            "Plan for iteration {} ({} steps)",
            self.artifacts.current_iteration_number,
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
            self.artifacts.current_iteration_number
        );

        self.current_plan = Some(plan);
        self.reset_step_index();

        self.transition_to(AgentState::ExecuteStep)
    }

    fn build_plan_steps(
        &self,
        plan: &mut Plan,
        goal: &str,
        neural_suggestion: Option<&symbolic::NeuralProposal>,
        issue_category: &str,
    ) -> AgentResult<()> {
        plan.add_step(PlanStep {
            description: format!(
                "Read repository structure (iteration {})",
                self.artifacts.current_iteration_number
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

        if let Some(command_plan) = select_validation_command(
            goal,
            &self.artifacts.config.agent_name,
            &self.artifacts.config.execution_mode,
        ) {
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
                args: self.cargo_clippy_args(),
                verification: "security_validation_passes".to_string(),
            });
        }

        Ok(())
    }

    fn apply_learning_adaptations(&mut self, plan: &mut Plan) {
        let learning_ctx = LearningContext::from_metadata(&self.memory.metadata);
        let previous_failures = learning_ctx.previous_failures;
        let previous_max_iteration = learning_ctx.previous_max_iteration;
        let top_failure_kind = learning_ctx.top_failure_kind;
        let top_failure_tool = learning_ctx.top_failure_tool;
        let top_decision_action = learning_ctx.top_decision_action;
        let recent_avg_failures = learning_ctx.recent_avg_failures;
        let recent_top_failure_kind = learning_ctx.recent_top_failure_kind;
        let recent_top_failure_kind_confidence = learning_ctx.recent_top_failure_kind_confidence;
        let worst_action_outcome = learning_ctx.worst_action_outcome;

        if previous_failures > 0 {
            plan.add_step(PlanStep {
                description: format!(
                    "Learning adaptation: rerun deterministic validation (previous_failures={})",
                    previous_failures
                ),
                tool: "run_tests".to_string(),
                args: self.cargo_check_args(),
                verification: "learning_validation_passes".to_string(),
            });
        }

        if previous_failures >= 3 || previous_max_iteration >= self.max_iterations_limit.get() / 2 {
            plan.add_step(PlanStep {
                description: "Learning adaptation: strict lint gate after unstable runs"
                    .to_string(),
                tool: "run_tests".to_string(),
                args: self.cargo_clippy_args(),
                verification: "learning_strict_lint_passes".to_string(),
            });
        }
        if top_failure_kind.starts_with("policy:") {
            plan.add_step(PlanStep {
                description: "Learning adaptation: policy-focused validation after policy failures"
                    .to_string(),
                tool: "run_tests".to_string(),
                args: self.cargo_clippy_args(),
                verification: "learning_policy_gate_passes".to_string(),
            });
        }
        if top_failure_tool.starts_with("run_tests:") {
            plan.add_step(PlanStep {
                description: "Learning adaptation: stabilize test harness execution path"
                    .to_string(),
                tool: "run_tests".to_string(),
                args: self.cargo_test_args(),
                verification: "learning_test_harness_stable".to_string(),
            });
        }
        if top_decision_action.starts_with("read_file:") && previous_failures > 0 {
            plan.add_step(PlanStep {
                description: "Learning adaptation: prioritize validation over repeated exploration"
                    .to_string(),
                tool: "run_tests".to_string(),
                args: self.cargo_check_args(),
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
                args: self.cargo_check_args(),
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
                    args: self.cargo_check_args(),
                    verification: "learning_action_preflight".to_string(),
                });
            } else if action == "read_file" {
                plan.add_step(PlanStep {
                    description:
                        "Learning adaptation: reduce exploration-heavy loop with deterministic validation"
                            .to_string(),
                    tool: "run_tests".to_string(),
                    args: self.cargo_check_args(),
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

    fn cargo_check_args(&self) -> Vec<String> {
        vec![
            "cargo".to_string(),
            "check".to_string(),
            "-p".to_string(),
            self.artifacts.config.agent_name.clone(),
            "--bin".to_string(),
            self.artifacts.config.agent_name.clone(),
        ]
    }

    fn cargo_clippy_args(&self) -> Vec<String> {
        vec![
            "cargo".to_string(),
            "clippy".to_string(),
            "-p".to_string(),
            self.artifacts.config.agent_name.clone(),
            "--bin".to_string(),
            self.artifacts.config.agent_name.clone(),
        ]
    }

    fn cargo_test_args(&self) -> Vec<String> {
        vec![
            "cargo".to_string(),
            "test".to_string(),
            "-p".to_string(),
            self.artifacts.config.agent_name.clone(),
            "--bin".to_string(),
            self.artifacts.config.agent_name.clone(),
            "--no-fail-fast".to_string(),
        ]
    }

    fn validate_plan(&mut self, plan: &Plan) -> AgentResult<()> {
        let validator = Validator::new(self.artifacts.config.symbolic.strict_validation);
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

            self.enforce_authz_for_action(&step.tool, &step.args)?;
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
                self.artifacts.current_iteration_number,
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
            self.artifacts.current_iteration_number,
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

        self.enforce_authz_for_action(&step.tool, &step.args)?;
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
        self.memory
            .metadata
            .insert("last_tool_name".to_string(), step.tool.clone());
        let result = self.execute_tool_with_timeout(&step.tool, &step.args)?;
        validate_tool_result_contract(&step.tool, &result)?;
        let tool_duration = tool_start.elapsed();
        self.artifacts.run_replay.record(
            "tool.execute",
            format!(
                "tool={} state={:?} iteration={} step={} success={} exit_code={:?} duration_ms={}",
                step.tool,
                self.state,
                self.artifacts.current_iteration_number,
                self.current_step_index.get(),
                result.success,
                result.exit_code,
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
            let error_class = classify_tool_failure(&error_detail);
            self.artifacts.run_replay.record(
                "tool.failure_class",
                format!("tool={} class={}", step.tool, error_class),
            );
            if step.tool == "run_tests" {
                self.memory
                    .metadata
                    .insert("last_validation_success".to_string(), "false".to_string());
            }
            self.memory.metadata.insert(
                "last_tool_failure_class".to_string(),
                format!("{}:{}", step.tool, error_class),
            );
            if let Some(exit_code) = result.exit_code {
                self.memory
                    .metadata
                    .insert("last_tool_exit_code".to_string(), exit_code.to_string());
            } else {
                self.memory
                    .metadata
                    .insert("last_tool_exit_code".to_string(), "none".to_string());
            }

            tracing::warn!(
                "Tool '{}' failed [{}]: {} (duration: {:?})",
                step.tool,
                error_class,
                error_detail,
                tool_duration
            );

            self.memory.add_failure(
                self.iteration,
                format!("Tool execution failed [{}]: {}", error_class, step.tool),
                error_detail.clone(),
                Some("Will retry in next iteration".to_string()),
            );

            return Err(AgentError::Tool(format!(
                "Tool '{}' execution failed [{}]: {}",
                step.tool, error_class, error_detail
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
            ActionName::new("tool_executed_successfully")
                .expect("static action name must be valid"),
        );

        self.advance_step_index();

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



    fn verify_step(&mut self) -> AgentResult<()> {
        self.check_timeout()?;

        tracing::info!(
            "Iteration {}: Verifying step {} execution",
            self.artifacts.current_iteration_number,
            self.current_step_index.get()
        );

        let has_remaining_steps = self
            .current_plan
            .as_ref()
            .map(|plan| self.current_step_index.get() < plan.steps.len())
            .unwrap_or(false);

        if has_remaining_steps {
            return self.transition_to(AgentState::ExecuteStep);
        }

        self.transition_to(AgentState::EvaluateObjectives)
    }

    fn evaluate_objectives(&mut self) -> AgentResult<()> {
        self.check_timeout()?;

        tracing::info!(
            "Iteration {}: Evaluating objectives",
            self.artifacts.current_iteration_number
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
        let enforce_slo_during_objective_eval =
            std::env::var("AUTONOMOUS_ENFORCE_SLO_DURING_OBJECTIVE_EVAL")
                .ok()
                .map(|v| v.eq_ignore_ascii_case("true"))
                .unwrap_or(false);

        self.audit
            .log_objective_evaluation(self.iteration, scores.clone())
            .map_err(|e| AgentError::State(e.to_string()))?;

        self.memory
            .add_objective_evaluation(self.iteration, scores.clone(), hard_satisfied);
        self.memory.metadata.insert(
            "weighted_objective_score".to_string(),
            format!("{:.3}", weighted_objective_score),
        );
        self.artifacts.run_replay.record(
            "objectives.weighted_score",
            format!("{:.3}", weighted_objective_score),
        );

        if !hard_satisfied || (enforce_slo_during_objective_eval && !slo_all_met) {
            tracing::warn!(
                "Iteration {}: objectives not satisfied (hard_ok={} slo_ok={} enforce_slo={})",
                self.artifacts.current_iteration_number,
                hard_satisfied,
                slo_all_met,
                enforce_slo_during_objective_eval
            );

            let Some(next_iteration) = self.artifacts.current_iteration_number.try_next() else {
                self.transition_to(AgentState::Failed)?;
                return Err(AgentError::State("Iteration counter overflow".to_string()));
            };

            if next_iteration.exceeds(self.artifacts.max_iterations_limit) {
                self.memory.add_failure(
                    self.iteration,
                    "Hard objectives not satisfied - max iterations reached".to_string(),
                    format!(
                        "Failed after {} iterations with scores: {:?}",
                        self.artifacts.current_iteration_number.get(),
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
                    "Objective scores: {:?}; SLO all met: {}; enforce_slo_during_objective_eval={}",
                    scores, slo_all_met, enforce_slo_during_objective_eval
                ),
                Some(format!(
                    "Retry with adjusted approach (attempt {}/{})",
                    next_iteration,
                    self.artifacts.max_iterations_limit.get()
                )),
            );

            self.set_iteration_number(next_iteration);
            self.current_plan = None;
            self.reset_step_index();

            self.transition_to(AgentState::GeneratePlan)?;
        } else {
            tracing::info!(
                "Iteration {}: All hard objectives satisfied",
                self.artifacts.current_iteration_number
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

        let assume_validation_pass_without_test_step =
            std::env::var("AUTONOMOUS_ASSUME_VALIDATION_PASS_WHEN_NO_TEST_STEP")
                .ok()
                .map(|v| v.eq_ignore_ascii_case("true"))
                .unwrap_or(false);
        let has_run_tests_step = self
            .current_plan
            .as_ref()
            .map(|p| p.steps.iter().any(|s| s.tool == "run_tests"))
            .unwrap_or(false);
        let tests_pass = self
            .memory
            .metadata
            .get("last_validation_success")
            .map(|v| if v == "true" { 1.0 } else { 0.0 })
            .unwrap_or_else(|| {
                if assume_validation_pass_without_test_step && !has_run_tests_step {
                    1.0
                } else {
                    0.7
                }
            });

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
            self.artifacts.current_iteration_number,
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
            self.record_replay(
                "pr.description.fallback",
                "policy_disallowed_generate_pr_description".to_string(),
            );
            self.memory.metadata.insert(
                "pr_description_source".to_string(),
                "default_policy_disallowed".to_string(),
            );
            return Some(default_body.to_string());
        }
        if let Err(error) = self
            .enforce_authz_for_action(tool_name, &[])
            .and_then(|_| self.enforce_risk_gate(tool_name, &[]))
        {
            self.record_replay("pr.description.fallback", format!("gated: {}", error));
            self.memory.add_failure(
                self.iteration,
                "PR description generation blocked by policy gate".to_string(),
                error.to_string(),
                Some("Using default PR body".to_string()),
            );
            self.memory.metadata.insert(
                "pr_description_source".to_string(),
                "default_gated".to_string(),
            );
            return Some(default_body.to_string());
        }

        let Some(main_pr_number) = main_pr_number else {
            self.record_replay(
                "pr.description.fallback",
                "missing_main_pr_reference".to_string(),
            );
            self.memory.metadata.insert(
                "pr_description_source".to_string(),
                "default_missing_main_pr_reference".to_string(),
            );
            return Some(default_body.to_string());
        };
        let tool_args = vec![main_pr_number, output_file.clone()];

        match self.generate_pr_description(&tool_args, &output_file) {
            Ok(generated) => {
                self.record_replay(
                    "pr.description.generated",
                    format!("output_file={}", output_file),
                );
                self.memory
                    .metadata
                    .insert("pr_description_source".to_string(), "generated".to_string());
                Some(generated)
            }
            Err(err) => {
                self.record_replay(
                    "pr.description.fallback",
                    format!("generation_failed: {}", err),
                );
                self.memory.add_failure(
                    self.iteration,
                    "PR description generation failed".to_string(),
                    err.to_string(),
                    Some("Using default PR body".to_string()),
                );
                self.memory.metadata.insert(
                    "pr_description_source".to_string(),
                    "default_generation_failed".to_string(),
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
        validate_tool_result_contract(tool_name, &result)?;
        let tool_duration = tool_start.elapsed();
        self.memory
            .metadata
            .insert("last_tool_name".to_string(), tool_name.to_string());

        self.metrics
            .record_tool_execution(tool_name, result.success, tool_duration);
        self.artifacts.run_replay.record(
            "tool.execute",
            format!(
                "tool={} state={:?} iteration={} step=na success={} exit_code={:?} duration_ms={}",
                tool_name,
                self.state,
                self.artifacts.current_iteration_number,
                result.success,
                result.exit_code,
                tool_duration.as_millis()
            ),
        );

        self.audit
            .log_tool_execution(tool_name, tool_args, result.success)
            .map_err(|e| AgentError::State(e.to_string()))?;

        if !result.success {
            let details = result.error.unwrap_or_else(|| "Unknown error".to_string());
            let failure_class = classify_tool_failure(&details);
            self.artifacts.run_replay.record(
                "tool.failure_class",
                format!("tool={} class={}", tool_name, failure_class),
            );
            self.memory.metadata.insert(
                "last_tool_failure_class".to_string(),
                format!("{}:{}", tool_name, failure_class),
            );
            if let Some(exit_code) = result.exit_code {
                self.memory
                    .metadata
                    .insert("last_tool_exit_code".to_string(), exit_code.to_string());
            } else {
                self.memory
                    .metadata
                    .insert("last_tool_exit_code".to_string(), "none".to_string());
            }
            return Err(AgentError::Tool(details));
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
            self.artifacts.current_iteration_number
        );
        let review_required = std::env::var("AUTONOMOUS_REVIEW_REQUIRED")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let pr_number = self
            .pr_orchestrator
            .as_ref()
            .and_then(|o| o.metadata.pr_number);
        let (comments, review_input_source) = self.load_review_comments(pr_number)?;
        let require_gh_review_source = std::env::var("AUTONOMOUS_REQUIRE_GH_REVIEW_SOURCE")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        if require_gh_review_source && review_input_source != "gh_pr_view" {
            self.artifacts.run_replay.record(
                "review.blocked",
                format!("required_gh_source_but_got_{}", review_input_source),
            );
            self.memory.add_failure(
                self.iteration,
                "Review source requirement not satisfied".to_string(),
                format!(
                    "AUTONOMOUS_REQUIRE_GH_REVIEW_SOURCE=true but source was '{}'",
                    review_input_source
                ),
                Some(
                    "Enable AUTONOMOUS_FETCH_REVIEW_FROM_GH=true and ensure PR number is available"
                        .to_string(),
                ),
            );
            self.memory.metadata.insert(
                "last_review_outcome".to_string(),
                "blocked_review_source_requirement".to_string(),
            );
            return self.transition_to(AgentState::Blocked);
        }
        self.artifacts
            .run_replay
            .record("review.input_source", review_input_source.clone());
        self.memory
            .metadata
            .insert("last_review_input_source".to_string(), review_input_source);

        let Some(orchestrator) = self.pr_orchestrator.as_mut() else {
            if review_required {
                self.artifacts.run_replay.record(
                    "review.blocked",
                    "review_required_but_no_pr_orchestrator".to_string(),
                );
                self.memory.add_failure(
                    self.iteration,
                    "Review feedback required".to_string(),
                    "AUTONOMOUS_REVIEW_REQUIRED=true but no PR orchestrator is available"
                        .to_string(),
                    Some("Ensure PR creation stage completed before review stage".to_string()),
                );
                self.memory.metadata.insert(
                    "last_review_outcome".to_string(),
                    "blocked_no_pr_orchestrator".to_string(),
                );
                return self.transition_to(AgentState::Blocked);
            }
            self.artifacts
                .run_replay
                .record("review.skip", "no_pr_orchestrator".to_string());
            self.memory.metadata.insert(
                "last_review_outcome".to_string(),
                "skipped_no_pr_orchestrator".to_string(),
            );
            return self.transition_to(AgentState::Done);
        };
        if comments.is_empty() {
            if review_required {
                self.artifacts.run_replay.record(
                    "review.blocked",
                    "review_required_but_no_feedback".to_string(),
                );
                self.memory.add_failure(
                    self.iteration,
                    "Review feedback required".to_string(),
                    "AUTONOMOUS_REVIEW_REQUIRED=true but no review comments were provided"
                        .to_string(),
                    Some(
                        "Provide AUTONOMOUS_REVIEW_COMMENT/AUTONOMOUS_REVIEW_COMMENTS_JSON or enable AUTONOMOUS_FETCH_REVIEW_FROM_GH=true".to_string(),
                    ),
                );
                self.memory.metadata.insert(
                    "last_review_outcome".to_string(),
                    "blocked_no_feedback".to_string(),
                );
                return self.transition_to(AgentState::Blocked);
            }
            self.artifacts
                .run_replay
                .record("review.skip", "no_feedback_provided".to_string());
            self.memory.metadata.insert(
                "last_review_outcome".to_string(),
                "skipped_no_feedback".to_string(),
            );
            return self.transition_to(AgentState::Done);
        }
        let outcome = orchestrator.ingest_review(comments);
        self.artifacts
            .run_replay
            .record("review.outcome", format!("{:?}", outcome));
        self.memory
            .metadata
            .insert("last_review_outcome".to_string(), format!("{:?}", outcome));

        match outcome {
            ReviewOutcome::Approved => self.transition_to(AgentState::Done),
            ReviewOutcome::ChangesRequested => {
                let pending_reviewers: Vec<String> = orchestrator
                    .review_ingester
                    .pending_feedback()
                    .into_iter()
                    .map(|c| c.reviewer.clone())
                    .collect();
                self.artifacts.run_replay.record(
                    "review.pending_feedback",
                    format!("count={}", pending_reviewers.len()),
                );

                let auto_resolve = std::env::var("AUTONOMOUS_AUTO_RESOLVE_REVIEW")
                    .unwrap_or_else(|_| "false".to_string())
                    == "true";
                if auto_resolve {
                    for reviewer in pending_reviewers {
                        orchestrator.review_ingester.resolve(&reviewer);
                    }
                    let post_outcome = orchestrator.review_ingester.outcome();
                    self.artifacts
                        .run_replay
                        .record("review.post_resolve_outcome", format!("{:?}", post_outcome));
                    if post_outcome == ReviewOutcome::Approved {
                        return self.transition_to(AgentState::Done);
                    }
                }

                let Some(next_iteration) = self.artifacts.current_iteration_number.try_next()
                else {
                    return self.transition_to(AgentState::Blocked);
                };
                if next_iteration.exceeds(self.artifacts.max_iterations_limit) {
                    return self.transition_to(AgentState::Blocked);
                }
                self.set_iteration_number(next_iteration);
                self.current_plan = None;
                self.reset_step_index();
                self.transition_to(AgentState::GeneratePlan)
            }
            ReviewOutcome::Timeout => self.transition_to(AgentState::Blocked),
        }
    }

    fn load_review_comments(
        &mut self,
        pr_number: Option<PrNumber>,
    ) -> AgentResult<(Vec<ReviewComment>, String)> {
        let (comments, source) = load_review_comments_from_env()?;
        if !comments.is_empty() {
            return Ok((comments, source.to_string()));
        }

        let fetch_from_gh = std::env::var("AUTONOMOUS_FETCH_REVIEW_FROM_GH")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        if !fetch_from_gh {
            return Ok((Vec::new(), "none".to_string()));
        }

        let Some(pr_number) = pr_number else {
            self.artifacts.run_replay.record(
                "review.fetch.skip",
                "gh_enabled_but_missing_pr_number".to_string(),
            );
            return Ok((Vec::new(), "gh_skip_missing_pr_number".to_string()));
        };

        match self.fetch_review_comments_from_gh(pr_number) {
            Ok(comments) => Ok((comments, "gh_pr_view".to_string())),
            Err(error) => {
                self.memory.add_failure(
                    self.iteration,
                    "Review feedback retrieval failed".to_string(),
                    error.to_string(),
                    Some("Provide review comments via env/file fallback".to_string()),
                );
                self.artifacts
                    .run_replay
                    .record("review.fetch.failed", error.to_string());
                Ok((Vec::new(), "gh_fetch_failed".to_string()))
            }
        }
    }

    fn fetch_review_comments_from_gh(
        &mut self,
        pr_number: PrNumber,
    ) -> AgentResult<Vec<ReviewComment>> {
        let mut command = Command::new("gh");
        command
            .arg("pr")
            .arg("view")
            .arg(pr_number.to_string())
            .arg("--json")
            .arg("reviews,comments");
        let mut audit_args = vec![
            "pr".to_string(),
            "view".to_string(),
            pr_number.to_string(),
            "--json".to_string(),
            "reviews,comments".to_string(),
        ];
        if let Ok(repo) = std::env::var("AUTONOMOUS_REPO")
            && !repo.trim().is_empty()
        {
            command.arg("--repo").arg(&repo);
            audit_args.push("--repo".to_string());
            audit_args.push(repo);
        }

        let output =
            run_command_with_timeout(command, self.tool_timeout.duration, "gh pr view reviews")?;
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        let _ = self.audit.log_tool_execution(
            "fetch_review_comments",
            &audit_args,
            output.status.success(),
        );

        if !output.status.success() {
            let details = if stderr.is_empty() {
                stdout.clone()
            } else {
                stderr.clone()
            };
            let failure_class = classify_tool_failure(&details);
            self.record_replay(
                "tool.failure_class",
                format!("tool=fetch_review_comments class={}", failure_class),
            );
            self.memory.metadata.insert(
                "last_tool_failure_class".to_string(),
                format!("fetch_review_comments:{}", failure_class),
            );
            return Err(AgentError::Tool(format!(
                "gh pr view failed (status={}): {}",
                output.status, details
            )));
        }

        parse_review_comments_from_gh_json(&stdout)
    }

    fn fetch_pr_ci_status_from_gh(&mut self, pr_number: PrNumber) -> AgentResult<CiStatus> {
        let mut command = Command::new("gh");
        command
            .arg("pr")
            .arg("view")
            .arg(pr_number.to_string())
            .arg("--json")
            .arg("statusCheckRollup");
        let mut audit_args = vec![
            "pr".to_string(),
            "view".to_string(),
            pr_number.to_string(),
            "--json".to_string(),
            "statusCheckRollup".to_string(),
        ];
        if let Ok(repo) = std::env::var("AUTONOMOUS_REPO")
            && !repo.trim().is_empty()
        {
            command.arg("--repo").arg(&repo);
            audit_args.push("--repo".to_string());
            audit_args.push(repo);
        }

        let output =
            run_command_with_timeout(command, self.tool_timeout.duration, "gh pr view checks")?;
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        let _ = self.audit.log_tool_execution(
            "fetch_pr_ci_status",
            &audit_args,
            output.status.success(),
        );

        if !output.status.success() {
            let details = if stderr.is_empty() {
                stdout.clone()
            } else {
                stderr.clone()
            };
            let failure_class = classify_tool_failure(&details);
            self.record_replay(
                "tool.failure_class",
                format!("tool=fetch_pr_ci_status class={}", failure_class),
            );
            self.memory.metadata.insert(
                "last_tool_failure_class".to_string(),
                format!("fetch_pr_ci_status:{}", failure_class),
            );
            return Err(AgentError::Tool(format!(
                "gh pr view (statusCheckRollup) failed (status={}): {}",
                output.status, details
            )));
        }

        infer_ci_status_from_gh_json(&stdout)
    }

    pub fn metrics(&self) -> LifecycleMetrics {
        self.metrics.snapshot()
    }

    pub fn current_state(&self) -> AgentState {
        self.state
    }

    pub fn current_iteration(&self) -> usize {
        self.artifacts.current_iteration_number.get()
    }

    fn configure_policy_pack_from_env(&mut self) -> AgentResult<()> {
        let Some(overrides) = env::var("AUTONOMOUS_TOOL_RISK_OVERRIDES").ok() else {
            return Ok(());
        };
        if overrides.trim().is_empty() {
            return Ok(());
        }

        let allow_unknown = env::var("AUTONOMOUS_ALLOW_UNKNOWN_RISK_OVERRIDE_TOOLS")
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
        let auto_sign = env::var("AUTONOMOUS_POLICY_PACK_AUTO_SIGN")
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
        self.artifacts.run_replay.record(
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
            self.artifacts.run_replay.record(
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

fn env_f64_or(key: &str, default: f64) -> f64 {
    env::var(key)
        .ok()
        .and_then(|raw| raw.trim().parse::<f64>().ok())
        .unwrap_or(default)
}

fn env_u64_or(key: &str, default: u64) -> u64 {
    env::var(key)
        .ok()
        .and_then(|raw| raw.trim().parse::<u64>().ok())
        .unwrap_or(default)
}

fn require_env_non_empty(key: &str) -> AgentResult<String> {
    let value = env::var(key).map_err(|_| {
        AgentError::State(format!(
            "AUTONOMOUS_NON_INTERACTIVE_PROFILE=orchestrator_v1 requires {} to be set",
            key
        ))
    })?;
    if value.trim().is_empty() {
        return Err(AgentError::State(format!(
            "AUTONOMOUS_NON_INTERACTIVE_PROFILE=orchestrator_v1 requires {} to be non-empty",
            key
        )));
    }
    Ok(value)
}

fn build_tool_metric_snapshots(metrics: &LifecycleMetrics) -> HashMap<String, ToolMetricSnapshot> {
    let mut result = HashMap::new();
    for (tool, durations) in &metrics.tool_execution_times {
        let executions = metrics
            .tool_execution_counts
            .get(tool)
            .copied()
            .unwrap_or(0);
        let failures = metrics
            .tool_execution_failures
            .get(tool)
            .copied()
            .unwrap_or(0);

        if durations.is_empty() {
            result.insert(
                tool.clone(),
                ToolMetricSnapshot {
                    executions,
                    failures,
                    avg_duration_ms: 0,
                    p95_duration_ms: 0,
                    max_duration_ms: 0,
                },
            );
            continue;
        }

        let mut millis: Vec<u128> = durations.iter().map(|d| d.as_millis()).collect();
        millis.sort_unstable();
        let sum: u128 = millis.iter().copied().sum();
        let avg_duration_ms = sum / (millis.len() as u128);
        let max_duration_ms = *millis.last().unwrap_or(&0);
        let p95_idx = ((millis.len().saturating_sub(1)) * 95) / 100;
        let p95_duration_ms = millis.get(p95_idx).copied().unwrap_or(0);

        result.insert(
            tool.clone(),
            ToolMetricSnapshot {
                executions,
                failures,
                avg_duration_ms,
                p95_duration_ms,
                max_duration_ms,
            },
        );
    }
    result
}

fn list_repository_entries(root: &str, max_entries: usize) -> std::io::Result<Vec<String>> {
    let mut entries = Vec::new();
    let mut dir_entries = fs::read_dir(root)?
        .flatten()
        .map(|e| e.path())
        .collect::<Vec<_>>();
    dir_entries.sort();

    for path in dir_entries.into_iter().take(max_entries.max(1)) {
        let rel = path
            .strip_prefix(root)
            .ok()
            .and_then(|p| p.to_str())
            .map(|s| {
                s.trim_start_matches('/')
                    .trim_start_matches('\\')
                    .to_string()
            })
            .filter(|s| !s.is_empty())
            .or_else(|| path.to_str().map(|s| s.to_string()));
        if let Some(value) = rel {
            entries.push(value);
        }
    }

    Ok(entries)
}

fn has_valid_escalation_approval(required_role: &str) -> bool {
    let approved_role = env::var("AUTONOMOUS_ESCALATION_APPROVAL_ROLE").ok();
    let provided_token = env::var("AUTONOMOUS_ESCALATION_APPROVAL_TOKEN").ok();
    let expected_token = env::var("AUTONOMOUS_EXPECTED_ESCALATION_TOKEN").ok();

    match (approved_role, provided_token, expected_token) {
        (Some(role), Some(provided), Some(expected)) => {
            !role.is_empty()
                && role == required_role
                && !provided.is_empty()
                && provided == expected
        }
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

pub(crate) fn compensation_for_tool(tool: &str) -> CompensationKind {
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

pub(crate) fn classify_tool_failure(error_text: &str) -> &'static str {
    let lower = error_text.to_ascii_lowercase();
    if lower.contains("timed out") || lower.contains("timeout") {
        return "timeout";
    }
    if lower.contains("not allowed") || lower.contains("policy") || lower.contains("forbidden") {
        return "policy";
    }
    if lower.contains("not found") || lower.contains("no such file") || lower.contains("spawn") {
        return "environment";
    }
    if lower.contains("exited with code") {
        return "execution";
    }
    "unknown"
}

fn classify_failure_entry(entry: &FailureEntry) -> String {
    let text = format!(
        "{} {}",
        entry.description.to_ascii_lowercase(),
        entry.error.to_ascii_lowercase()
    );
    if text.contains("policy") || text.contains("authorization") {
        return "policy".to_string();
    }
    if text.contains("timeout") {
        return "timeout".to_string();
    }
    if text.contains("circuit") {
        return "circuit_breaker".to_string();
    }
    if text.contains("resource") || text.contains("budget") {
        return "resource".to_string();
    }
    if text.contains("test") || text.contains("validation") {
        return "validation".to_string();
    }
    if text.contains("tool") {
        return "tool".to_string();
    }
    "other".to_string()
}

fn validate_tool_result_contract(tool: &str, result: &ToolResult) -> AgentResult<()> {
    match (result.success, result.exit_code) {
        (true, Some(code)) if code != 0 => {
            return Err(AgentError::Tool(format!(
                "tool '{}' returned success=true with non-zero exit code {}",
                tool, code
            )));
        }
        (false, Some(0)) => {
            return Err(AgentError::Tool(format!(
                "tool '{}' returned success=false with exit code 0",
                tool
            )));
        }
        _ => {}
    }

    if !result.success && result.error.as_deref().unwrap_or("").trim().is_empty() {
        return Err(AgentError::Tool(format!(
            "tool '{}' returned success=false without error details",
            tool
        )));
    }

    Ok(())
}

fn run_command_with_timeout(
    mut command: Command,
    timeout: Duration,
    label: &str,
) -> AgentResult<process::Output> {
    let mut child = command
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn()
        .map_err(|e| AgentError::Tool(format!("failed to spawn '{label}': {e}")))?;

    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(_)) => {
                return child.wait_with_output().map_err(|e| {
                    AgentError::Tool(format!("wait_with_output for '{label}' failed: {e}"))
                });
            }
            Ok(None) => {
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait_with_output();
                    return Err(AgentError::Tool(format!(
                        "'{label}' timed out after {}s",
                        timeout.as_secs()
                    )));
                }
                std::thread::sleep(Duration::from_millis(25));
            }
            Err(e) => {
                return Err(AgentError::Tool(format!(
                    "try_wait for '{label}' failed: {e}"
                )));
            }
        }
    }
}

fn load_review_comments_from_env() -> AgentResult<(Vec<ReviewComment>, &'static str)> {
    if let Ok(path) = env::var("AUTONOMOUS_REVIEW_COMMENTS_FILE") {
        let raw = fs::read_to_string(&path).map_err(|e| {
            AgentError::State(format!(
                "Failed to read AUTONOMOUS_REVIEW_COMMENTS_FILE '{}': {}",
                path, e
            ))
        })?;
        let comments: Vec<ReviewComment> = common_json::from_str(&raw).map_err(|e| {
            AgentError::State(format!(
                "AUTONOMOUS_REVIEW_COMMENTS_FILE must contain a valid JSON array of review comments: {e}"
            ))
        })?;
        return Ok((comments, "env_file"));
    }

    if let Ok(raw_json) = env::var("AUTONOMOUS_REVIEW_COMMENTS_JSON") {
        let comments: Vec<ReviewComment> = common_json::from_str(&raw_json).map_err(|e| {
            AgentError::State(format!(
                "AUTONOMOUS_REVIEW_COMMENTS_JSON must be valid JSON array of review comments: {e}"
            ))
        })?;
        return Ok((comments, "env_json"));
    }

    if let Ok(body) = env::var("AUTONOMOUS_REVIEW_COMMENT") {
        return Ok((
            vec![ReviewComment {
                reviewer: "review-bot".to_string(),
                body,
                resolved: false,
            }],
            "env_single",
        ));
    }

    Ok((Vec::new(), "none"))
}

fn infer_ci_status_from_gh_json(raw: &str) -> AgentResult<CiStatus> {
    let root = common_json::from_str(raw).map_err(|e| {
        AgentError::State(format!(
            "Failed to parse gh statusCheckRollup payload as JSON: {}",
            e
        ))
    })?;

    let Some(items) = root.get("statusCheckRollup").and_then(|v| v.as_array()) else {
        return Ok(CiStatus::Unknown);
    };
    if items.is_empty() {
        return Ok(CiStatus::Unknown);
    }

    let mut any_pending = false;
    let mut any_failing = false;
    let mut any_success = false;
    let mut any_known = false;

    for item in items {
        for key in ["state", "status", "conclusion"] {
            let Some(value) = item.get(key).and_then(|v| v.as_str()) else {
                continue;
            };
            any_known = true;
            let normalized = value.to_ascii_uppercase();
            if matches!(
                normalized.as_str(),
                "FAILURE"
                    | "FAILED"
                    | "ERROR"
                    | "TIMED_OUT"
                    | "CANCELLED"
                    | "ACTION_REQUIRED"
                    | "STARTUP_FAILURE"
            ) {
                any_failing = true;
            } else if matches!(
                normalized.as_str(),
                "PENDING" | "IN_PROGRESS" | "QUEUED" | "WAITING" | "REQUESTED" | "EXPECTED"
            ) {
                any_pending = true;
            } else if matches!(
                normalized.as_str(),
                "SUCCESS" | "PASSED" | "COMPLETED" | "NEUTRAL" | "SKIPPED"
            ) {
                any_success = true;
            }
        }
    }

    if any_failing {
        return Ok(CiStatus::Failing);
    }
    if any_pending {
        return Ok(CiStatus::Pending);
    }
    if any_success {
        return Ok(CiStatus::Passing);
    }
    if any_known {
        return Ok(CiStatus::Unknown);
    }

    Ok(CiStatus::Unknown)
}
