// projects/products/unstable/autonomy_orchestrator_ai/src/orchestrator.rs
use crate::adaptive_policy::{
    AdaptivePolicyConfig, maybe_increase_execution_budget, maybe_increase_remediation_cycles,
};
use crate::artifacts::{
    ExecutionPolicyOverrides, OrchestratorCycleMemory, ValidationInvocationArtifact,
    load_cycle_memory, load_next_actions, read_detected_validation_commands, save_cycle_memory,
    write_repo_context_artifact,
};
use crate::binary_runner::invoke_binary;
use crate::checkpoint_store::save_checkpoint;
use crate::decision_aggregator::{DecisionAggregatorConfig, aggregate};
use crate::domain::{
    BinaryInvocationSpec, CiGateStatus, CommandLineSpec, DecisionContribution, DeliveryOptions,
    FinalDecision, GateDecision, GateInputs, OrchestratorCheckpoint, OrchestratorConfig,
    PolicyGateStatus, ReviewGateStatus, RunReport, Stage, StageExecutionRecord,
    StageExecutionStatus, StageTransition, TerminalState,
};
use crate::planner_output::read_planner_output_from_artifacts;
use common_json::{Json, JsonAccess, from_str};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

pub struct Orchestrator {
    report: RunReport,
    simulate_blocked: bool,
    planning_invocation: Option<BinaryInvocationSpec>,
    execution_invocation: Option<BinaryInvocationSpec>,
    validation_invocation: Option<BinaryInvocationSpec>,
    execution_max_iterations: u32,
    reviewer_remediation_max_cycles: u32,
    timeout_ms: u64,
    repo_root: PathBuf,
    planning_context_artifact: Option<PathBuf>,
    validation_invocations: Vec<BinaryInvocationSpec>,
    validation_from_planning_context: bool,
    delivery_options: DeliveryOptions,
    decision_threshold: u8,
    decision_contributions: Vec<DecisionContribution>,
    decision_require_contributions: bool,
    adaptive_policy_config: AdaptivePolicyConfig,
    execution_budget_adapted: bool,
    remediation_budget_adapted: bool,
    gate_inputs: GateInputs,
    checkpoint: OrchestratorCheckpoint,
    checkpoint_path: Option<PathBuf>,
    cycle_memory_path: Option<PathBuf>,
    remediation_cycle: u32,
    remediation_steps: Vec<String>,
    planned_remediation_steps: Vec<String>,
}

impl Orchestrator {
    pub fn new(config: OrchestratorConfig, checkpoint: Option<OrchestratorCheckpoint>) -> Self {
        let mut execution_max_iterations = config.execution_policy.execution_max_iterations;
        let mut reviewer_remediation_max_cycles =
            config.execution_policy.reviewer_remediation_max_cycles;
        let mut validation_invocations = config.validation_invocations;
        let mut planned_remediation_steps = Vec::new();
        if let Some(path) = &config.cycle_memory_path
            && let Ok(memory) = load_cycle_memory(path)
        {
            if let Some(v) = memory.execution_policy_overrides.execution_max_iterations
                && v >= 1
            {
                execution_max_iterations = v;
            }
            if let Some(v) = memory
                .execution_policy_overrides
                .reviewer_remediation_max_cycles
            {
                reviewer_remediation_max_cycles = v;
            }
            planned_remediation_steps = memory.planned_remediation_steps;
            for command in memory.validation_commands {
                let spec = BinaryInvocationSpec {
                    stage: Stage::Validation,
                    command_line: command.command_line,
                    env: Vec::new(),
                    timeout_ms: config.timeout_ms,
                    expected_artifacts: Vec::new(),
                };
                if !validation_invocations.iter().any(|existing| {
                    existing.command_line.command == spec.command_line.command
                        && existing.command_line.args == spec.command_line.args
                }) {
                    validation_invocations.push(spec);
                }
            }
        }
        if planned_remediation_steps.is_empty()
            && let Some(path) = &config.next_actions_path
            && let Ok(next_actions) = load_next_actions(path)
            && !next_actions.recommended_actions.is_empty()
        {
            planned_remediation_steps = next_actions.recommended_actions;
        }
        let checkpoint = checkpoint.unwrap_or_else(|| {
            OrchestratorCheckpoint::new(config.run_id.clone(), unix_timestamp_secs())
        });
        Self {
            report: RunReport::new(config.run_id),
            simulate_blocked: config.simulate_blocked,
            planning_invocation: config.planning_invocation,
            execution_invocation: config.execution_invocation,
            validation_invocation: config.validation_invocation,
            execution_max_iterations,
            reviewer_remediation_max_cycles,
            timeout_ms: config.timeout_ms,
            repo_root: config.repo_root,
            planning_context_artifact: config.planning_context_artifact,
            validation_invocations,
            validation_from_planning_context: config.validation_from_planning_context,
            delivery_options: config.delivery_options,
            decision_threshold: config.decision_threshold,
            decision_contributions: config.decision_contributions,
            decision_require_contributions: config.decision_require_contributions,
            adaptive_policy_config: AdaptivePolicyConfig::default(),
            execution_budget_adapted: false,
            remediation_budget_adapted: false,
            gate_inputs: config.gate_inputs,
            checkpoint,
            checkpoint_path: config.checkpoint_path,
            cycle_memory_path: config.cycle_memory_path,
            remediation_cycle: 0,
            remediation_steps: Vec::new(),
            planned_remediation_steps,
        }
    }

    pub fn execute(mut self) -> RunReport {
        if self.checkpoint.terminal_state == Some(TerminalState::Done) {
            self.report.terminal_state = Some(TerminalState::Done);
            self.report.current_stage = Some(Stage::Closure);
            return self.report;
        }

        let planning_expected_artifacts = self
            .planning_invocation
            .as_ref()
            .map(|spec| spec.expected_artifacts.clone())
            .unwrap_or_default();
        let planning_was_completed = self.checkpoint.is_stage_completed(Stage::Planning);
        if !self.execute_stage(Stage::Planning, self.planning_invocation.clone()) {
            return self.report;
        }
        if !self.execute_planning_context_action(planning_was_completed) {
            return self.report;
        }
        if !self.apply_planner_output_from_artifacts(&planning_expected_artifacts) {
            return self.report;
        }

        if !self.execute_stage(Stage::PolicyIngestion, None) {
            return self.report;
        }

        if !self.execute_execution_validation_pipeline() {
            return self.report;
        }

        if !self.evaluate_fail_closed_gates() {
            self.report.terminal_state = Some(TerminalState::Blocked);
            self.mark_terminal_and_persist(TerminalState::Blocked);
            return self.report;
        }

        if self.simulate_blocked {
            self.report.terminal_state = Some(TerminalState::Blocked);
            self.mark_terminal_and_persist(TerminalState::Blocked);
            return self.report;
        }
        if !self.evaluate_final_decision() {
            self.report.terminal_state = Some(TerminalState::Blocked);
            self.mark_terminal_and_persist(TerminalState::Blocked);
            return self.report;
        }

        if !self.execute_stage(Stage::Closure, None) {
            return self.report;
        }
        if !self.execute_delivery_lifecycle() {
            return self.report;
        }
        self.report.terminal_state = Some(TerminalState::Done);
        self.mark_terminal_and_persist(TerminalState::Done);
        self.report
    }

    fn execute_execution_validation_pipeline(&mut self) -> bool {
        loop {
            if !self.execute_execution_stage() {
                return false;
            }

            if self.execute_validation_stage() {
                return true;
            }

            if !self.try_schedule_reviewer_remediation_cycle() {
                return false;
            }
        }
    }

    fn transition_to(&mut self, next_stage: Stage) {
        let transition = StageTransition {
            run_id: self.report.run_id.clone(),
            from_stage: self.report.current_stage,
            to_stage: next_stage,
            timestamp_unix_secs: unix_timestamp_secs(),
        };

        self.report.current_stage = Some(next_stage);
        self.report.transitions.push(transition);
    }

    fn execute_invocation_or_stop(&mut self, spec: BinaryInvocationSpec) -> bool {
        let execution = invoke_binary(&spec);
        let status = execution.status;
        self.report.stage_executions.push(execution);

        match status {
            StageExecutionStatus::Success | StageExecutionStatus::Skipped => true,
            StageExecutionStatus::Timeout => {
                self.report.terminal_state = Some(TerminalState::Timeout);
                false
            }
            StageExecutionStatus::Failed
            | StageExecutionStatus::SpawnFailed
            | StageExecutionStatus::ArtifactMissing => {
                self.report.terminal_state = Some(TerminalState::Failed);
                false
            }
        }
    }

    fn execute_stage(&mut self, stage: Stage, invocation: Option<BinaryInvocationSpec>) -> bool {
        self.transition_to(stage);

        if self.checkpoint.is_stage_completed(stage) {
            if let Some(spec) = invocation {
                self.report.stage_executions.push(StageExecutionRecord {
                    stage,
                    idempotency_key: Some(format!("stage:{:?}", stage)),
                    command: spec.command_line.command,
                    args: spec.command_line.args,
                    env_keys: spec.env.into_iter().map(|(k, _)| k).collect(),
                    started_at_unix_secs: unix_timestamp_secs(),
                    duration_ms: 0,
                    exit_code: None,
                    status: StageExecutionStatus::Skipped,
                    error: Some("Skipped due to checkpoint resume".to_string()),
                    missing_artifacts: Vec::new(),
                    malformed_artifacts: Vec::new(),
                });
            }
            return true;
        }

        if let Some(spec) = invocation
            && !self.execute_invocation_or_stop(spec)
        {
            self.persist_checkpoint();
            return false;
        }

        self.checkpoint
            .mark_stage_completed(stage, unix_timestamp_secs());
        self.persist_checkpoint();
        true
    }

    fn execute_execution_stage(&mut self) -> bool {
        self.transition_to(Stage::Execution);

        if self.checkpoint.is_stage_completed(Stage::Execution) {
            if let Some(spec) = self.execution_invocation.clone() {
                self.report.stage_executions.push(StageExecutionRecord {
                    stage: Stage::Execution,
                    idempotency_key: Some("stage:Execution".to_string()),
                    command: spec.command_line.command,
                    args: spec.command_line.args,
                    env_keys: spec.env.into_iter().map(|(k, _)| k).collect(),
                    started_at_unix_secs: unix_timestamp_secs(),
                    duration_ms: 0,
                    exit_code: None,
                    status: StageExecutionStatus::Skipped,
                    error: Some("Skipped due to checkpoint resume".to_string()),
                    missing_artifacts: Vec::new(),
                    malformed_artifacts: Vec::new(),
                });
            }
            return true;
        }

        let Some(base_spec) = self.execution_invocation.clone() else {
            self.checkpoint
                .mark_stage_completed(Stage::Execution, unix_timestamp_secs());
            self.persist_checkpoint();
            return true;
        };

        let mut attempt = 1u32;
        while attempt <= self.execution_max_iterations {
            let mut spec = base_spec.clone();
            spec.env.push((
                "ORCHESTRATOR_EXECUTION_ATTEMPT".to_string(),
                attempt.to_string(),
            ));
            if !self.remediation_steps.is_empty() {
                spec.env.push((
                    "ORCHESTRATOR_REMEDIATION_CYCLE".to_string(),
                    self.remediation_cycle.to_string(),
                ));
                spec.env.push((
                    "ORCHESTRATOR_REMEDIATION_STEPS".to_string(),
                    self.remediation_steps.join(" || "),
                ));
            }
            let mut execution = invoke_binary(&spec);
            execution.idempotency_key = Some(format!("stage:Execution:attempt:{attempt}"));
            let status = execution.status;
            self.report.stage_executions.push(execution);

            match status {
                StageExecutionStatus::Success => {
                    self.checkpoint
                        .mark_stage_completed(Stage::Execution, unix_timestamp_secs());
                    self.persist_checkpoint();
                    return true;
                }
                StageExecutionStatus::Timeout => {
                    self.report.terminal_state = Some(TerminalState::Timeout);
                    self.persist_checkpoint();
                    return false;
                }
                StageExecutionStatus::SpawnFailed | StageExecutionStatus::ArtifactMissing => {
                    self.report.terminal_state = Some(TerminalState::Failed);
                    self.persist_checkpoint();
                    return false;
                }
                StageExecutionStatus::Failed => {
                    if attempt < self.execution_max_iterations {
                        attempt += 1;
                        continue;
                    }
                    if self.try_adapt_execution_budget_after_failure() {
                        attempt += 1;
                        continue;
                    }
                    self.report.stage_executions.push(StageExecutionRecord {
                        stage: Stage::Execution,
                        idempotency_key: Some("stage:Execution:budget".to_string()),
                        command: "execution.iteration_budget".to_string(),
                        args: vec![
                            "max_iterations_exhausted".to_string(),
                            self.execution_max_iterations.to_string(),
                        ],
                        env_keys: Vec::new(),
                        started_at_unix_secs: unix_timestamp_secs(),
                        duration_ms: 0,
                        exit_code: None,
                        status: StageExecutionStatus::Failed,
                        error: Some(format!(
                            "Execution iteration budget exhausted after {} attempt(s)",
                            self.execution_max_iterations
                        )),
                        missing_artifacts: Vec::new(),
                        malformed_artifacts: Vec::new(),
                    });
                    self.report.terminal_state = Some(TerminalState::Failed);
                    self.persist_checkpoint();
                    return false;
                }
                StageExecutionStatus::Skipped => {
                    self.checkpoint
                        .mark_stage_completed(Stage::Execution, unix_timestamp_secs());
                    self.persist_checkpoint();
                    return true;
                }
            }
        }

        self.report.terminal_state = Some(TerminalState::Failed);
        self.persist_checkpoint();
        false
    }

    fn execute_validation_stage(&mut self) -> bool {
        self.transition_to(Stage::Validation);

        let has_native_validation_commands = match self.determine_validation_invocations() {
            Ok(invocations) => !invocations.is_empty(),
            Err(err) => {
                self.report.terminal_state = Some(TerminalState::Failed);
                self.report.stage_executions.push(StageExecutionRecord {
                    stage: Stage::Validation,
                    idempotency_key: Some("stage:Validation:commands".to_string()),
                    command: "validation.resolve_commands".to_string(),
                    args: Vec::new(),
                    env_keys: Vec::new(),
                    started_at_unix_secs: unix_timestamp_secs(),
                    duration_ms: 0,
                    exit_code: None,
                    status: StageExecutionStatus::Failed,
                    error: Some(err),
                    missing_artifacts: Vec::new(),
                    malformed_artifacts: Vec::new(),
                });
                self.persist_checkpoint();
                return false;
            }
        };

        if self.checkpoint.is_stage_completed(Stage::Validation) {
            if let Some(spec) = self.validation_invocation.clone() {
                self.report.stage_executions.push(StageExecutionRecord {
                    stage: Stage::Validation,
                    idempotency_key: Some("stage:Validation".to_string()),
                    command: spec.command_line.command,
                    args: spec.command_line.args,
                    env_keys: spec.env.into_iter().map(|(k, _)| k).collect(),
                    started_at_unix_secs: unix_timestamp_secs(),
                    duration_ms: 0,
                    exit_code: None,
                    status: StageExecutionStatus::Skipped,
                    error: Some("Skipped due to checkpoint resume".to_string()),
                    missing_artifacts: Vec::new(),
                    malformed_artifacts: Vec::new(),
                });
            } else if has_native_validation_commands {
                self.report.stage_executions.push(StageExecutionRecord {
                    stage: Stage::Validation,
                    idempotency_key: Some("stage:Validation:invocations".to_string()),
                    command: "validation.invocations".to_string(),
                    args: Vec::new(),
                    env_keys: Vec::new(),
                    started_at_unix_secs: unix_timestamp_secs(),
                    duration_ms: 0,
                    exit_code: None,
                    status: StageExecutionStatus::Skipped,
                    error: Some("Skipped due to checkpoint resume".to_string()),
                    missing_artifacts: Vec::new(),
                    malformed_artifacts: Vec::new(),
                });
            }
            return true;
        }

        if let Some(spec) = self.validation_invocation.clone() {
            let expected_artifacts = spec.expected_artifacts.clone();
            let succeeded = self.execute_invocation_or_stop(spec);
            self.collect_reviewer_next_steps(&expected_artifacts);
            if !succeeded {
                self.persist_checkpoint();
                return false;
            }
            self.checkpoint
                .mark_stage_completed(Stage::Validation, unix_timestamp_secs());
            self.persist_checkpoint();
            return true;
        }

        match self.determine_validation_invocations() {
            Ok(invocations) => {
                if !self.execute_native_validation_invocations(&invocations) {
                    self.persist_checkpoint();
                    return false;
                }
            }
            Err(err) => {
                self.report.terminal_state = Some(TerminalState::Failed);
                self.report.stage_executions.push(StageExecutionRecord {
                    stage: Stage::Validation,
                    idempotency_key: Some("stage:Validation:commands".to_string()),
                    command: "validation.resolve_commands".to_string(),
                    args: Vec::new(),
                    env_keys: Vec::new(),
                    started_at_unix_secs: unix_timestamp_secs(),
                    duration_ms: 0,
                    exit_code: None,
                    status: StageExecutionStatus::Failed,
                    error: Some(err),
                    missing_artifacts: Vec::new(),
                    malformed_artifacts: Vec::new(),
                });
                self.persist_checkpoint();
                return false;
            }
        }

        self.checkpoint
            .mark_stage_completed(Stage::Validation, unix_timestamp_secs());
        self.persist_checkpoint();
        true
    }

    fn collect_reviewer_next_steps(&mut self, artifact_paths: &[String]) {
        let mut collected = Vec::new();

        for artifact in artifact_paths {
            if !artifact.ends_with(".json") {
                continue;
            }
            let raw = match fs::read_to_string(artifact) {
                Ok(raw) => raw,
                Err(_) => continue,
            };
            let parsed: Json = match from_str(&raw) {
                Ok(parsed) => parsed,
                Err(_) => continue,
            };
            let steps = match parsed
                .get_field("next_step_plan")
                .and_then(|value| value.as_array_strict())
            {
                Ok(steps) => steps,
                Err(_) => continue,
            };

            for step in steps {
                let action = match step
                    .get_field("action")
                    .and_then(|value| value.as_str_strict())
                {
                    Ok(action) => action,
                    Err(_) => continue,
                };
                let priority = step
                    .get_field("priority")
                    .and_then(|value| value.as_u64_strict())
                    .ok();
                let code = step
                    .get_field("code")
                    .and_then(|value| value.as_str_strict())
                    .ok();
                let formatted = match (priority, code) {
                    (Some(priority), Some(code)) => {
                        format!("P{priority} [{code}] {action}")
                    }
                    (Some(priority), None) => format!("P{priority} {action}"),
                    (None, Some(code)) => format!("[{code}] {action}"),
                    (None, None) => action.to_string(),
                };
                if !collected.contains(&formatted) {
                    collected.push(formatted);
                }
            }
        }

        if !collected.is_empty() {
            self.report.reviewer_next_steps = collected;
        }
    }

    fn try_schedule_reviewer_remediation_cycle(&mut self) -> bool {
        if self.validation_invocation.is_none() {
            return false;
        }
        if self.execution_invocation.is_none() {
            return false;
        }
        if self.report.terminal_state != Some(TerminalState::Failed) {
            return false;
        }

        let remediation_steps = if self.report.reviewer_next_steps.is_empty() {
            self.planned_remediation_steps.clone()
        } else {
            self.report.reviewer_next_steps.clone()
        };
        if remediation_steps.is_empty() {
            return false;
        }
        if self.remediation_cycle >= self.reviewer_remediation_max_cycles
            && !self.try_adapt_remediation_budget_after_failure()
        {
            return false;
        }

        self.remediation_cycle += 1;
        self.remediation_steps = remediation_steps;
        self.report.terminal_state = None;
        self.reset_checkpoint_stage(Stage::Execution);
        self.reset_checkpoint_stage(Stage::Validation);
        self.persist_checkpoint();
        true
    }

    fn try_adapt_execution_budget_after_failure(&mut self) -> bool {
        if self.execution_budget_adapted {
            return false;
        }
        let Some(signature) = self.latest_failure_signature() else {
            return false;
        };
        let Some(decision) = maybe_increase_execution_budget(
            self.execution_max_iterations,
            &signature,
            self.adaptive_policy_config,
        ) else {
            return false;
        };
        self.execution_budget_adapted = true;
        self.execution_max_iterations = decision.new_value;
        self.report.adaptive_policy_decisions.push(decision.clone());
        self.report.stage_executions.push(StageExecutionRecord {
            stage: Stage::Execution,
            idempotency_key: Some("stage:Execution:adaptive_policy".to_string()),
            command: "execution.policy.adapt".to_string(),
            args: vec![
                decision.reason_code,
                decision.trigger_signature,
                decision.previous_value.to_string(),
                decision.new_value.to_string(),
            ],
            env_keys: Vec::new(),
            started_at_unix_secs: unix_timestamp_secs(),
            duration_ms: 0,
            exit_code: Some(0),
            status: StageExecutionStatus::Success,
            error: None,
            missing_artifacts: Vec::new(),
            malformed_artifacts: Vec::new(),
        });
        true
    }

    fn try_adapt_remediation_budget_after_failure(&mut self) -> bool {
        if self.remediation_budget_adapted {
            return false;
        }
        let Some(signature) = self.latest_failure_signature() else {
            return false;
        };
        let Some(decision) = maybe_increase_remediation_cycles(
            self.reviewer_remediation_max_cycles,
            &signature,
            self.adaptive_policy_config,
        ) else {
            return false;
        };
        self.remediation_budget_adapted = true;
        self.reviewer_remediation_max_cycles = decision.new_value;
        self.report.adaptive_policy_decisions.push(decision.clone());
        self.report.stage_executions.push(StageExecutionRecord {
            stage: Stage::Validation,
            idempotency_key: Some("stage:Validation:adaptive_policy".to_string()),
            command: "validation.policy.adapt".to_string(),
            args: vec![
                decision.reason_code,
                decision.trigger_signature,
                decision.previous_value.to_string(),
                decision.new_value.to_string(),
            ],
            env_keys: Vec::new(),
            started_at_unix_secs: unix_timestamp_secs(),
            duration_ms: 0,
            exit_code: Some(0),
            status: StageExecutionStatus::Success,
            error: None,
            missing_artifacts: Vec::new(),
            malformed_artifacts: Vec::new(),
        });
        true
    }

    fn latest_failure_signature(&self) -> Option<String> {
        self.report
            .stage_executions
            .iter()
            .rev()
            .find(|execution| {
                matches!(
                    execution.status,
                    StageExecutionStatus::Failed
                        | StageExecutionStatus::SpawnFailed
                        | StageExecutionStatus::ArtifactMissing
                        | StageExecutionStatus::Timeout
                )
            })
            .map(|execution| {
                format!(
                    "stage={:?};status={:?};command={};exit={}",
                    execution.stage,
                    execution.status,
                    execution.command,
                    execution
                        .exit_code
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "none".to_string())
                )
            })
    }

    fn reset_checkpoint_stage(&mut self, stage: Stage) {
        self.checkpoint.completed_stages.retain(|s| *s != stage);
    }

    fn determine_validation_invocations(&self) -> Result<Vec<BinaryInvocationSpec>, String> {
        let mut invocations = self.validation_invocations.clone();
        if self.validation_from_planning_context
            && let Some(path) = &self.planning_context_artifact
        {
            let from_artifact = read_detected_validation_commands(path)?;
            for item in from_artifact {
                let spec = self.validation_artifact_to_spec(item);
                if !invocations.iter().any(|existing| {
                    existing.command_line.command == spec.command_line.command
                        && existing.command_line.args == spec.command_line.args
                }) {
                    invocations.push(spec);
                }
            }
        }
        Ok(invocations)
    }

    fn execute_native_validation_invocations(
        &mut self,
        invocations: &[BinaryInvocationSpec],
    ) -> bool {
        for (index, base_spec) in invocations.iter().enumerate() {
            let mut spec = base_spec.clone();
            spec.env.push((
                "ORCHESTRATOR_VALIDATION_COMMAND_INDEX".to_string(),
                (index + 1).to_string(),
            ));
            let mut execution = invoke_binary(&spec);
            execution.idempotency_key = Some(format!("stage:Validation:command:{}", index + 1));
            let status = execution.status;
            self.report.stage_executions.push(execution);
            match status {
                StageExecutionStatus::Success | StageExecutionStatus::Skipped => {}
                StageExecutionStatus::Timeout => {
                    self.report.terminal_state = Some(TerminalState::Timeout);
                    return false;
                }
                StageExecutionStatus::Failed
                | StageExecutionStatus::SpawnFailed
                | StageExecutionStatus::ArtifactMissing => {
                    self.report.terminal_state = Some(TerminalState::Failed);
                    return false;
                }
            }
        }
        true
    }

    fn validation_artifact_to_spec(
        &self,
        artifact: ValidationInvocationArtifact,
    ) -> BinaryInvocationSpec {
        BinaryInvocationSpec {
            stage: Stage::Validation,
            command_line: artifact.command_line,
            env: Vec::new(),
            timeout_ms: self.timeout_ms,
            expected_artifacts: Vec::new(),
        }
    }

    fn execute_delivery_lifecycle(&mut self) -> bool {
        if !self.delivery_options.enabled {
            return true;
        }
        let mut steps: Vec<(String, String, Vec<String>)> = Vec::new();
        if let Some(branch) = self.delivery_options.branch.clone() {
            steps.push((
                "delivery.git.switch_branch".to_string(),
                "git".to_string(),
                vec!["switch".to_string(), "-c".to_string(), branch],
            ));
        }
        steps.push((
            "delivery.git.add".to_string(),
            "git".to_string(),
            vec!["add".to_string(), "-A".to_string()],
        ));
        if let Some(message) = self.delivery_options.commit_message.clone() {
            steps.push((
                "delivery.git.commit".to_string(),
                "git".to_string(),
                vec!["commit".to_string(), "-m".to_string(), message],
            ));
        }
        if self.delivery_options.pr_enabled {
            if let Some(number) = self.delivery_options.pr_number.clone() {
                let mut args = vec!["pr".to_string(), "edit".to_string(), number];
                if let Some(base) = self.delivery_options.pr_base.clone() {
                    args.push("--base".to_string());
                    args.push(base);
                }
                if let Some(title) = self.delivery_options.pr_title.clone() {
                    args.push("--title".to_string());
                    args.push(title);
                }
                if let Some(body) = self.delivery_options.pr_body.clone() {
                    args.push("--body".to_string());
                    args.push(body);
                }
                steps.push(("delivery.gh.pr.update".to_string(), "gh".to_string(), args));
            } else {
                let mut args = vec!["pr".to_string(), "create".to_string()];
                if let Some(base) = self.delivery_options.pr_base.clone() {
                    args.push("--base".to_string());
                    args.push(base);
                }
                if let Some(title) = self.delivery_options.pr_title.clone() {
                    args.push("--title".to_string());
                    args.push(title);
                }
                if let Some(body) = self.delivery_options.pr_body.clone() {
                    args.push("--body".to_string());
                    args.push(body);
                }
                if let Some(head) = self.delivery_options.branch.clone() {
                    args.push("--head".to_string());
                    args.push(head);
                }
                steps.push(("delivery.gh.pr.create".to_string(), "gh".to_string(), args));
            }
        }

        for (index, (step_id, tool, args)) in steps.into_iter().enumerate() {
            if self.delivery_options.dry_run {
                self.report.stage_executions.push(StageExecutionRecord {
                    stage: Stage::Closure,
                    idempotency_key: Some(format!("stage:Closure:delivery:{}", index + 1)),
                    command: format!("{step_id}.dry_run"),
                    args,
                    env_keys: Vec::new(),
                    started_at_unix_secs: unix_timestamp_secs(),
                    duration_ms: 0,
                    exit_code: Some(0),
                    status: StageExecutionStatus::Success,
                    error: None,
                    missing_artifacts: Vec::new(),
                    malformed_artifacts: Vec::new(),
                });
                continue;
            }

            let started_at_unix_secs = unix_timestamp_secs();
            let started = Instant::now();
            let status = Command::new(&tool)
                .args(&args)
                .current_dir(&self.repo_root)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
            match status {
                Ok(exit) if exit.success() => {
                    self.report.stage_executions.push(StageExecutionRecord {
                        stage: Stage::Closure,
                        idempotency_key: Some(format!("stage:Closure:delivery:{}", index + 1)),
                        command: step_id,
                        args,
                        env_keys: Vec::new(),
                        started_at_unix_secs,
                        duration_ms: duration_to_u64_ms(started.elapsed()),
                        exit_code: exit.code(),
                        status: StageExecutionStatus::Success,
                        error: None,
                        missing_artifacts: Vec::new(),
                        malformed_artifacts: Vec::new(),
                    });
                }
                Ok(exit) => {
                    self.report.stage_executions.push(StageExecutionRecord {
                        stage: Stage::Closure,
                        idempotency_key: Some(format!("stage:Closure:delivery:{}", index + 1)),
                        command: step_id,
                        args,
                        env_keys: Vec::new(),
                        started_at_unix_secs,
                        duration_ms: duration_to_u64_ms(started.elapsed()),
                        exit_code: exit.code(),
                        status: StageExecutionStatus::Failed,
                        error: Some("Delivery command failed".to_string()),
                        missing_artifacts: Vec::new(),
                        malformed_artifacts: Vec::new(),
                    });
                    self.report.terminal_state = Some(TerminalState::Failed);
                    self.persist_checkpoint();
                    return false;
                }
                Err(err) => {
                    self.report.stage_executions.push(StageExecutionRecord {
                        stage: Stage::Closure,
                        idempotency_key: Some(format!("stage:Closure:delivery:{}", index + 1)),
                        command: step_id,
                        args,
                        env_keys: Vec::new(),
                        started_at_unix_secs,
                        duration_ms: duration_to_u64_ms(started.elapsed()),
                        exit_code: None,
                        status: StageExecutionStatus::SpawnFailed,
                        error: Some(format!("Delivery command spawn failed: {err}")),
                        missing_artifacts: Vec::new(),
                        malformed_artifacts: Vec::new(),
                    });
                    self.report.terminal_state = Some(TerminalState::Failed);
                    self.persist_checkpoint();
                    return false;
                }
            }
        }

        true
    }

    fn mark_terminal_and_persist(&mut self, terminal_state: TerminalState) {
        self.checkpoint
            .mark_terminal(terminal_state, unix_timestamp_secs());
        self.persist_checkpoint();
    }

    fn execute_planning_context_action(&mut self, planning_was_completed: bool) -> bool {
        let Some(artifact_path) = self.planning_context_artifact.clone() else {
            return true;
        };
        if planning_was_completed {
            self.report.stage_executions.push(StageExecutionRecord {
                stage: Stage::Planning,
                idempotency_key: Some("stage:Planning:repo_context".to_string()),
                command: "planning.repo_context".to_string(),
                args: vec![
                    self.repo_root.display().to_string(),
                    artifact_path.display().to_string(),
                ],
                env_keys: Vec::new(),
                started_at_unix_secs: unix_timestamp_secs(),
                duration_ms: 0,
                exit_code: None,
                status: StageExecutionStatus::Skipped,
                error: Some("Skipped due to checkpoint resume".to_string()),
                missing_artifacts: Vec::new(),
                malformed_artifacts: Vec::new(),
            });
            return true;
        }

        let started = Instant::now();
        let started_at_unix_secs = unix_timestamp_secs();
        match write_repo_context_artifact(Path::new(&self.repo_root), Path::new(&artifact_path)) {
            Ok(()) => {
                self.report.stage_executions.push(StageExecutionRecord {
                    stage: Stage::Planning,
                    idempotency_key: Some("stage:Planning:repo_context".to_string()),
                    command: "planning.repo_context".to_string(),
                    args: vec![
                        self.repo_root.display().to_string(),
                        artifact_path.display().to_string(),
                    ],
                    env_keys: Vec::new(),
                    started_at_unix_secs,
                    duration_ms: duration_to_u64_ms(started.elapsed()),
                    exit_code: Some(0),
                    status: StageExecutionStatus::Success,
                    error: None,
                    missing_artifacts: Vec::new(),
                    malformed_artifacts: Vec::new(),
                });
                true
            }
            Err(err) => {
                self.report.stage_executions.push(StageExecutionRecord {
                    stage: Stage::Planning,
                    idempotency_key: Some("stage:Planning:repo_context".to_string()),
                    command: "planning.repo_context".to_string(),
                    args: vec![
                        self.repo_root.display().to_string(),
                        artifact_path.display().to_string(),
                    ],
                    env_keys: Vec::new(),
                    started_at_unix_secs,
                    duration_ms: duration_to_u64_ms(started.elapsed()),
                    exit_code: None,
                    status: StageExecutionStatus::Failed,
                    error: Some(err),
                    missing_artifacts: Vec::new(),
                    malformed_artifacts: Vec::new(),
                });
                self.report.terminal_state = Some(TerminalState::Failed);
                self.persist_checkpoint();
                false
            }
        }
    }

    fn apply_planner_output_from_artifacts(&mut self, artifacts: &[String]) -> bool {
        let parsed = match read_planner_output_from_artifacts(artifacts) {
            Ok(parsed) => parsed,
            Err(err) => {
                self.report.terminal_state = Some(TerminalState::Failed);
                self.report.stage_executions.push(StageExecutionRecord {
                    stage: Stage::Planning,
                    idempotency_key: Some("stage:Planning:planner_output".to_string()),
                    command: "planning.planner_output.apply".to_string(),
                    args: Vec::new(),
                    env_keys: Vec::new(),
                    started_at_unix_secs: unix_timestamp_secs(),
                    duration_ms: 0,
                    exit_code: None,
                    status: StageExecutionStatus::Failed,
                    error: Some(err),
                    missing_artifacts: Vec::new(),
                    malformed_artifacts: Vec::new(),
                });
                self.persist_checkpoint();
                return false;
            }
        };

        let Some(parsed) = parsed else {
            return true;
        };

        let source_path = parsed.source_path;
        let payload = parsed.payload;
        if matches!(payload.execution_max_iterations, Some(0)) {
            self.report.terminal_state = Some(TerminalState::Failed);
            self.report.stage_executions.push(StageExecutionRecord {
                stage: Stage::Planning,
                idempotency_key: Some("stage:Planning:planner_output".to_string()),
                command: "planning.planner_output.apply".to_string(),
                args: vec![source_path],
                env_keys: Vec::new(),
                started_at_unix_secs: unix_timestamp_secs(),
                duration_ms: 0,
                exit_code: None,
                status: StageExecutionStatus::Failed,
                error: Some(
                    "Planner output execution_max_iterations must be >= 1 when provided"
                        .to_string(),
                ),
                missing_artifacts: Vec::new(),
                malformed_artifacts: Vec::new(),
            });
            self.persist_checkpoint();
            return false;
        }

        if let Some(max_iterations) = payload.execution_max_iterations {
            self.execution_max_iterations = max_iterations;
        }
        if let Some(remediation_cycles) = payload.reviewer_remediation_max_cycles {
            self.reviewer_remediation_max_cycles = remediation_cycles;
        }
        if !payload.remediation_steps.is_empty() {
            self.planned_remediation_steps = payload.remediation_steps;
        }
        for command in payload.validation_commands {
            let spec = self.validation_artifact_to_spec(command);
            if !self.validation_invocations.iter().any(|existing| {
                existing.command_line.command == spec.command_line.command
                    && existing.command_line.args == spec.command_line.args
            }) {
                self.validation_invocations.push(spec);
            }
        }

        self.report.stage_executions.push(StageExecutionRecord {
            stage: Stage::Planning,
            idempotency_key: Some("stage:Planning:planner_output".to_string()),
            command: "planning.planner_output.apply".to_string(),
            args: vec![source_path],
            env_keys: Vec::new(),
            started_at_unix_secs: unix_timestamp_secs(),
            duration_ms: 0,
            exit_code: Some(0),
            status: StageExecutionStatus::Success,
            error: None,
            missing_artifacts: Vec::new(),
            malformed_artifacts: Vec::new(),
        });
        true
    }

    fn persist_checkpoint(&mut self) {
        if let Some(path) = self.checkpoint_path.clone()
            && let Err(err) = save_checkpoint(path.as_path(), &self.checkpoint)
        {
            self.report.terminal_state = Some(TerminalState::Failed);
            self.report.stage_executions.push(StageExecutionRecord {
                stage: self.report.current_stage.unwrap_or(Stage::Planning),
                idempotency_key: None,
                command: "checkpoint.persist".to_string(),
                args: vec![path.display().to_string()],
                env_keys: Vec::new(),
                started_at_unix_secs: unix_timestamp_secs(),
                duration_ms: 0,
                exit_code: None,
                status: StageExecutionStatus::Failed,
                error: Some(err),
                missing_artifacts: Vec::new(),
                malformed_artifacts: Vec::new(),
            });
        }

        if let Some(path) = self.cycle_memory_path.clone() {
            let memory = OrchestratorCycleMemory {
                execution_policy_overrides: ExecutionPolicyOverrides {
                    execution_max_iterations: Some(self.execution_max_iterations),
                    reviewer_remediation_max_cycles: Some(self.reviewer_remediation_max_cycles),
                },
                planned_remediation_steps: self.planned_remediation_steps.clone(),
                validation_commands: self
                    .validation_invocations
                    .iter()
                    .map(|spec| ValidationInvocationArtifact {
                        command_line: CommandLineSpec {
                            command: spec.command_line.command.clone(),
                            args: spec.command_line.args.clone(),
                        },
                    })
                    .collect(),
                updated_at_unix_secs: unix_timestamp_secs(),
            };
            if let Err(err) = save_cycle_memory(path.as_path(), &memory) {
                self.report.stage_executions.push(StageExecutionRecord {
                    stage: self.report.current_stage.unwrap_or(Stage::Planning),
                    idempotency_key: None,
                    command: "cycle_memory.persist".to_string(),
                    args: vec![path.display().to_string()],
                    env_keys: Vec::new(),
                    started_at_unix_secs: unix_timestamp_secs(),
                    duration_ms: 0,
                    exit_code: None,
                    status: StageExecutionStatus::Failed,
                    error: Some(err),
                    missing_artifacts: Vec::new(),
                    malformed_artifacts: Vec::new(),
                });
            }
        }
    }

    fn evaluate_fail_closed_gates(&mut self) -> bool {
        let mut decisions = Vec::new();
        let mut blocked_reason_codes = Vec::new();

        let (policy_passed, policy_reason) = match self.gate_inputs.policy_status {
            PolicyGateStatus::Allow => (true, None),
            PolicyGateStatus::Deny | PolicyGateStatus::Unknown => {
                (false, Some("GATE_POLICY_DENIED_OR_UNKNOWN".to_string()))
            }
        };
        decisions.push(GateDecision {
            gate: "policy".to_string(),
            status: match self.gate_inputs.policy_status {
                PolicyGateStatus::Allow => "allow",
                PolicyGateStatus::Deny => "deny",
                PolicyGateStatus::Unknown => "unknown",
            }
            .to_string(),
            passed: policy_passed,
            reason_code: policy_reason.clone(),
        });
        if let Some(code) = policy_reason {
            blocked_reason_codes.push(code);
        }

        let (ci_passed, ci_reason) = match self.gate_inputs.ci_status {
            CiGateStatus::Success => (true, None),
            CiGateStatus::Pending | CiGateStatus::Failure | CiGateStatus::Missing => {
                (false, Some("GATE_CI_NOT_SUCCESS".to_string()))
            }
        };
        decisions.push(GateDecision {
            gate: "ci".to_string(),
            status: match self.gate_inputs.ci_status {
                CiGateStatus::Success => "success",
                CiGateStatus::Pending => "pending",
                CiGateStatus::Failure => "failure",
                CiGateStatus::Missing => "missing",
            }
            .to_string(),
            passed: ci_passed,
            reason_code: ci_reason.clone(),
        });
        if let Some(code) = ci_reason {
            blocked_reason_codes.push(code);
        }

        let (review_passed, review_reason) = match self.gate_inputs.review_status {
            ReviewGateStatus::Approved => (true, None),
            ReviewGateStatus::ChangesRequested | ReviewGateStatus::Missing => {
                (false, Some("GATE_REVIEW_NOT_APPROVED".to_string()))
            }
        };
        decisions.push(GateDecision {
            gate: "review".to_string(),
            status: match self.gate_inputs.review_status {
                ReviewGateStatus::Approved => "approved",
                ReviewGateStatus::ChangesRequested => "changes_requested",
                ReviewGateStatus::Missing => "missing",
            }
            .to_string(),
            passed: review_passed,
            reason_code: review_reason.clone(),
        });
        if let Some(code) = review_reason {
            blocked_reason_codes.push(code);
        }

        self.report.gate_decisions = decisions;
        self.report.blocked_reason_codes = blocked_reason_codes;
        self.report.blocked_reason_codes.is_empty()
    }

    fn evaluate_final_decision(&mut self) -> bool {
        if self.decision_contributions.is_empty() && !self.decision_require_contributions {
            self.report.final_decision = None;
            self.report.decision_confidence = None;
            self.report.decision_rationale_codes.clear();
            self.report.decision_contributions.clear();
            self.report.decision_threshold = None;
            return true;
        }

        let summary = aggregate(
            &self.decision_contributions,
            &DecisionAggregatorConfig {
                min_confidence_to_proceed: self.decision_threshold,
            },
        );
        self.report.final_decision = Some(summary.final_decision);
        self.report.decision_confidence = Some(summary.decision_confidence);
        self.report.decision_rationale_codes = summary.decision_rationale_codes.clone();
        self.report.decision_contributions = summary.contributions;
        self.report.decision_threshold = Some(summary.threshold);

        for code in &summary.decision_rationale_codes {
            if !self.report.blocked_reason_codes.contains(code) {
                self.report.blocked_reason_codes.push(code.clone());
            }
        }

        summary.final_decision == FinalDecision::Proceed
    }
}

fn unix_timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn duration_to_u64_ms(duration: std::time::Duration) -> u64 {
    u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
}
