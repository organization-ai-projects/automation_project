// projects/products/unstable/autonomy_orchestrator_ai/src/orchestrator.rs

use crate::binary_runner::invoke_binary;
use crate::checkpoint_store::save_checkpoint;
use crate::domain::{
    BinaryInvocationSpec, CiGateStatus, GateDecision, GateInputs, OrchestratorCheckpoint,
    PolicyGateStatus, ReviewGateStatus, RunReport, Stage, StageExecutionRecord,
    StageExecutionStatus, StageTransition, TerminalState,
};
use crate::repo_context_artifact::{
    ValidationInvocationArtifact, read_detected_validation_commands, write_repo_context_artifact,
};
use common_json::{Json, JsonAccess, from_str};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
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
    gate_inputs: GateInputs,
    checkpoint: OrchestratorCheckpoint,
    checkpoint_path: Option<PathBuf>,
    remediation_cycle: u32,
    remediation_steps: Vec<String>,
}

impl Orchestrator {
    pub fn new(
        run_id: String,
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
        gate_inputs: GateInputs,
        checkpoint: Option<OrchestratorCheckpoint>,
        checkpoint_path: Option<PathBuf>,
    ) -> Self {
        let checkpoint = checkpoint
            .unwrap_or_else(|| OrchestratorCheckpoint::new(run_id.clone(), unix_timestamp_secs()));
        Self {
            report: RunReport::new(run_id),
            simulate_blocked,
            planning_invocation,
            execution_invocation,
            validation_invocation,
            execution_max_iterations,
            reviewer_remediation_max_cycles,
            timeout_ms,
            repo_root,
            planning_context_artifact,
            validation_invocations,
            validation_from_planning_context,
            gate_inputs,
            checkpoint,
            checkpoint_path,
            remediation_cycle: 0,
            remediation_steps: Vec::new(),
        }
    }

    pub fn execute(mut self) -> RunReport {
        if self.checkpoint.terminal_state == Some(TerminalState::Done) {
            self.report.terminal_state = Some(TerminalState::Done);
            self.report.current_stage = Some(Stage::Closure);
            return self.report;
        }

        let planning_was_completed = self.checkpoint.is_stage_completed(Stage::Planning);
        if !self.execute_stage(Stage::Planning, self.planning_invocation.clone()) {
            return self.report;
        }
        if !self.execute_planning_context_action(planning_was_completed) {
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

        if !self.execute_stage(Stage::Closure, None) {
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
                    command: spec.command,
                    args: spec.args,
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
                    command: spec.command,
                    args: spec.args,
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

        for attempt in 1..=self.execution_max_iterations {
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
                    command: spec.command,
                    args: spec.args,
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
        if self.remediation_cycle >= self.reviewer_remediation_max_cycles {
            return false;
        }
        if self.report.reviewer_next_steps.is_empty() {
            return false;
        }
        if self.execution_invocation.is_none() {
            return false;
        }
        if self.report.terminal_state != Some(TerminalState::Failed) {
            return false;
        }

        self.remediation_cycle += 1;
        self.remediation_steps = self.report.reviewer_next_steps.clone();
        self.report.terminal_state = None;
        self.reset_checkpoint_stage(Stage::Execution);
        self.reset_checkpoint_stage(Stage::Validation);
        self.persist_checkpoint();
        true
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
                if !invocations
                    .iter()
                    .any(|existing| existing.command == spec.command && existing.args == spec.args)
                {
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
            command: artifact.command,
            args: artifact.args,
            env: Vec::new(),
            timeout_ms: self.timeout_ms,
            expected_artifacts: Vec::new(),
        }
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

    fn persist_checkpoint(&mut self) {
        let Some(path) = self.checkpoint_path.clone() else {
            return;
        };
        if let Err(err) = save_checkpoint(path.as_path(), &self.checkpoint) {
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

#[cfg(test)]
mod tests {
    use super::Orchestrator;
    use crate::domain::{
        BinaryInvocationSpec, GateInputs, OrchestratorCheckpoint, PolicyGateStatus, Stage,
        StageExecutionStatus, TerminalState,
    };
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn executes_all_stages_and_finishes_done() {
        let report = Orchestrator::new(
            "run_1".to_string(),
            false,
            None,
            None,
            None,
            1,
            0,
            30_000,
            PathBuf::from("."),
            None,
            Vec::new(),
            false,
            GateInputs::passing(),
            None,
            None,
        )
        .execute();

        assert_eq!(report.terminal_state, Some(TerminalState::Done));
        assert_eq!(report.current_stage, Some(Stage::Closure));
        assert_eq!(report.transitions.len(), 5);
        assert_eq!(report.transitions[0].from_stage, None);
        assert_eq!(report.transitions[0].to_stage, Stage::Planning);
        assert!(report.stage_executions.is_empty());
    }

    #[test]
    fn blocked_simulation_stops_before_closure() {
        let report = Orchestrator::new(
            "run_2".to_string(),
            true,
            None,
            None,
            None,
            1,
            0,
            30_000,
            PathBuf::from("."),
            None,
            Vec::new(),
            false,
            GateInputs::passing(),
            None,
            None,
        )
        .execute();

        assert_eq!(report.terminal_state, Some(TerminalState::Blocked));
        assert_eq!(report.current_stage, Some(Stage::Validation));
        assert_eq!(report.transitions.len(), 4);
    }

    #[test]
    fn spawn_failure_stops_pipeline_as_failed() {
        let planning_invocation = BinaryInvocationSpec {
            stage: Stage::Planning,
            command: "__missing_binary__".to_string(),
            args: Vec::new(),
            env: Vec::new(),
            timeout_ms: 100,
            expected_artifacts: Vec::new(),
        };

        let report = Orchestrator::new(
            "run_3".to_string(),
            false,
            Some(planning_invocation),
            None,
            None,
            1,
            0,
            30_000,
            PathBuf::from("."),
            None,
            Vec::new(),
            false,
            GateInputs::passing(),
            None,
            None,
        )
        .execute();

        assert_eq!(report.terminal_state, Some(TerminalState::Failed));
        assert_eq!(report.transitions.len(), 1);
        assert_eq!(report.current_stage, Some(Stage::Planning));
        assert_eq!(report.stage_executions.len(), 1);
    }

    #[test]
    fn resume_skips_completed_invocation_stage() {
        let checkpoint = OrchestratorCheckpoint {
            run_id: "run_4".to_string(),
            completed_stages: vec![Stage::Planning],
            terminal_state: None,
            updated_at_unix_secs: 1,
        };
        let planning_invocation = BinaryInvocationSpec {
            stage: Stage::Planning,
            command: "__unused__".to_string(),
            args: vec!["x".to_string()],
            env: vec![("KEY".to_string(), "VALUE".to_string())],
            timeout_ms: 100,
            expected_artifacts: Vec::new(),
        };

        let report = Orchestrator::new(
            "run_4".to_string(),
            false,
            Some(planning_invocation),
            None,
            None,
            1,
            0,
            30_000,
            PathBuf::from("."),
            None,
            Vec::new(),
            false,
            GateInputs::passing(),
            Some(checkpoint),
            None,
        )
        .execute();

        assert_eq!(report.terminal_state, Some(TerminalState::Done));
        assert_eq!(report.stage_executions.len(), 1);
        assert_eq!(
            report.stage_executions[0].status,
            StageExecutionStatus::Skipped
        );
    }

    #[test]
    fn fail_closed_policy_gate_blocks_pipeline_with_reason_code() {
        let report = Orchestrator::new(
            "run_5".to_string(),
            false,
            None,
            None,
            None,
            1,
            0,
            30_000,
            PathBuf::from("."),
            None,
            Vec::new(),
            false,
            GateInputs {
                policy_status: PolicyGateStatus::Unknown,
                ..GateInputs::passing()
            },
            None,
            None,
        )
        .execute();

        assert_eq!(report.terminal_state, Some(TerminalState::Blocked));
        assert!(
            report
                .blocked_reason_codes
                .contains(&"GATE_POLICY_DENIED_OR_UNKNOWN".to_string())
        );
        assert_eq!(report.current_stage, Some(Stage::Validation));
    }

    #[test]
    fn collects_reviewer_next_steps_from_review_report_artifact() {
        let mut orchestrator = Orchestrator::new(
            "run_review_steps".to_string(),
            false,
            None,
            None,
            None,
            1,
            0,
            30_000,
            PathBuf::from("."),
            None,
            Vec::new(),
            false,
            GateInputs::passing(),
            None,
            None,
        );
        let temp_file = std::env::temp_dir().join(format!(
            "orchestrator_review_steps_{}_{}.json",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        fs::write(
            &temp_file,
            r#"{"next_step_plan":[{"priority":1,"code":"A","action":"Fix A"},{"priority":2,"code":"B","action":"Fix B"}]}"#,
        )
        .expect("write review artifact");

        orchestrator.collect_reviewer_next_steps(&[temp_file.display().to_string()]);

        assert_eq!(orchestrator.report.reviewer_next_steps.len(), 2);
        assert!(
            orchestrator
                .report
                .reviewer_next_steps
                .iter()
                .any(|s| s.contains("P1 [A] Fix A"))
        );
        assert!(
            orchestrator
                .report
                .reviewer_next_steps
                .iter()
                .any(|s| s.contains("P2 [B] Fix B"))
        );

        fs::remove_file(temp_file).ok();
    }

    #[test]
    fn schedules_reviewer_remediation_cycle_when_configured() {
        let mut orchestrator = Orchestrator::new(
            "run_review_remediation".to_string(),
            false,
            None,
            Some(BinaryInvocationSpec {
                stage: Stage::Execution,
                command: "true".to_string(),
                args: Vec::new(),
                env: Vec::new(),
                timeout_ms: 100,
                expected_artifacts: Vec::new(),
            }),
            Some(BinaryInvocationSpec {
                stage: Stage::Validation,
                command: "false".to_string(),
                args: Vec::new(),
                env: Vec::new(),
                timeout_ms: 100,
                expected_artifacts: Vec::new(),
            }),
            1,
            1,
            30_000,
            PathBuf::from("."),
            None,
            Vec::new(),
            false,
            GateInputs::passing(),
            None,
            None,
        );
        orchestrator.report.terminal_state = Some(TerminalState::Failed);
        orchestrator
            .report
            .reviewer_next_steps
            .push("P1 [FIX_A] Apply fix A".to_string());
        orchestrator
            .checkpoint
            .mark_stage_completed(Stage::Execution, 1);
        orchestrator
            .checkpoint
            .mark_stage_completed(Stage::Validation, 1);

        let scheduled = orchestrator.try_schedule_reviewer_remediation_cycle();

        assert!(scheduled);
        assert_eq!(orchestrator.remediation_cycle, 1);
        assert_eq!(orchestrator.report.terminal_state, None);
        assert!(
            !orchestrator.checkpoint.is_stage_completed(Stage::Execution),
            "execution stage should be reset for remediation rerun"
        );
        assert!(
            !orchestrator
                .checkpoint
                .is_stage_completed(Stage::Validation),
            "validation stage should be reset for remediation rerun"
        );
    }
}
