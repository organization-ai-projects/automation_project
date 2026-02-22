// projects/products/unstable/autonomy_orchestrator_ai/src/orchestrator.rs

use crate::binary_runner::invoke_binary;
use crate::domain::{
    BinaryInvocationSpec, RunReport, Stage, StageExecutionStatus, StageTransition, TerminalState,
};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Orchestrator {
    report: RunReport,
    simulate_blocked: bool,
    planning_invocation: Option<BinaryInvocationSpec>,
    execution_invocation: Option<BinaryInvocationSpec>,
}

impl Orchestrator {
    pub fn new(
        run_id: String,
        simulate_blocked: bool,
        planning_invocation: Option<BinaryInvocationSpec>,
        execution_invocation: Option<BinaryInvocationSpec>,
    ) -> Self {
        Self {
            report: RunReport::new(run_id),
            simulate_blocked,
            planning_invocation,
            execution_invocation,
        }
    }

    pub fn execute(mut self) -> RunReport {
        self.transition_to(Stage::Planning);
        if let Some(spec) = self.planning_invocation.clone()
            && !self.execute_invocation_or_stop(spec)
        {
            return self.report;
        }

        self.transition_to(Stage::PolicyIngestion);
        self.transition_to(Stage::Execution);
        if let Some(spec) = self.execution_invocation.clone()
            && !self.execute_invocation_or_stop(spec)
        {
            return self.report;
        }

        self.transition_to(Stage::Validation);

        if self.simulate_blocked {
            self.report.terminal_state = Some(TerminalState::Blocked);
            return self.report;
        }

        self.transition_to(Stage::Closure);
        self.report.terminal_state = Some(TerminalState::Done);
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
            StageExecutionStatus::Success => true,
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
    use crate::domain::{BinaryInvocationSpec, Stage, TerminalState};

    #[test]
    fn executes_all_stages_and_finishes_done() {
        let report = Orchestrator::new("run_1".to_string(), false, None, None).execute();

        assert_eq!(report.terminal_state, Some(TerminalState::Done));
        assert_eq!(report.current_stage, Some(Stage::Closure));
        assert_eq!(report.transitions.len(), 5);
        assert_eq!(report.transitions[0].from_stage, None);
        assert_eq!(report.transitions[0].to_stage, Stage::Planning);
        assert!(report.stage_executions.is_empty());
    }

    #[test]
    fn blocked_simulation_stops_before_closure() {
        let report = Orchestrator::new("run_2".to_string(), true, None, None).execute();

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

        let report = Orchestrator::new("run_3".to_string(), false, Some(planning_invocation), None)
            .execute();

        assert_eq!(report.terminal_state, Some(TerminalState::Failed));
        assert_eq!(report.transitions.len(), 1);
        assert_eq!(report.current_stage, Some(Stage::Planning));
        assert_eq!(report.stage_executions.len(), 1);
    }
}
