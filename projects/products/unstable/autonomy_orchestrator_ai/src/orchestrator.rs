// projects/products/unstable/autonomy_orchestrator_ai/src/orchestrator.rs

use crate::binary_runner::invoke_binary;
use crate::checkpoint_store::save_checkpoint;
use crate::domain::{
    BinaryInvocationSpec, CiGateStatus, GateDecision, GateInputs, OrchestratorCheckpoint,
    PolicyGateStatus, ReviewGateStatus, RunReport, Stage, StageExecutionRecord,
    StageExecutionStatus, StageTransition, TerminalState,
};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Orchestrator {
    report: RunReport,
    simulate_blocked: bool,
    planning_invocation: Option<BinaryInvocationSpec>,
    execution_invocation: Option<BinaryInvocationSpec>,
    gate_inputs: GateInputs,
    checkpoint: OrchestratorCheckpoint,
    checkpoint_path: Option<PathBuf>,
}

impl Orchestrator {
    pub fn new(
        run_id: String,
        simulate_blocked: bool,
        planning_invocation: Option<BinaryInvocationSpec>,
        execution_invocation: Option<BinaryInvocationSpec>,
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
            gate_inputs,
            checkpoint,
            checkpoint_path,
        }
    }

    pub fn execute(mut self) -> RunReport {
        if self.checkpoint.terminal_state == Some(TerminalState::Done) {
            self.report.terminal_state = Some(TerminalState::Done);
            self.report.current_stage = Some(Stage::Closure);
            return self.report;
        }

        if !self.execute_stage(Stage::Planning, self.planning_invocation.clone()) {
            return self.report;
        }

        if !self.execute_stage(Stage::PolicyIngestion, None) {
            return self.report;
        }

        if !self.execute_stage(Stage::Execution, self.execution_invocation.clone()) {
            return self.report;
        }

        if !self.execute_stage(Stage::Validation, None) {
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

    fn mark_terminal_and_persist(&mut self, terminal_state: TerminalState) {
        self.checkpoint
            .mark_terminal(terminal_state, unix_timestamp_secs());
        self.persist_checkpoint();
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

#[cfg(test)]
mod tests {
    use super::Orchestrator;
    use crate::domain::{
        BinaryInvocationSpec, GateInputs, OrchestratorCheckpoint, PolicyGateStatus, Stage,
        StageExecutionStatus, TerminalState,
    };

    #[test]
    fn executes_all_stages_and_finishes_done() {
        let report = Orchestrator::new(
            "run_1".to_string(),
            false,
            None,
            None,
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
}
