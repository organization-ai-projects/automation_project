// projects/products/unstable/autonomy_orchestrator_ai/src/orchestrator.rs

use crate::domain::{RunReport, Stage, StageTransition, TerminalState};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Orchestrator {
    report: RunReport,
    simulate_blocked: bool,
}

impl Orchestrator {
    pub fn new(run_id: String, simulate_blocked: bool) -> Self {
        Self {
            report: RunReport::new(run_id),
            simulate_blocked,
        }
    }

    pub fn execute(mut self) -> RunReport {
        self.transition_to(Stage::Planning);
        self.transition_to(Stage::PolicyIngestion);
        self.transition_to(Stage::Execution);
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
    use crate::domain::{Stage, TerminalState};

    #[test]
    fn executes_all_stages_and_finishes_done() {
        let report = Orchestrator::new("run_1".to_string(), false).execute();

        assert_eq!(report.terminal_state, Some(TerminalState::Done));
        assert_eq!(report.current_stage, Some(Stage::Closure));
        assert_eq!(report.transitions.len(), 5);
        assert_eq!(report.transitions[0].from_stage, None);
        assert_eq!(report.transitions[0].to_stage, Stage::Planning);
    }

    #[test]
    fn blocked_simulation_stops_before_closure() {
        let report = Orchestrator::new("run_2".to_string(), true).execute();

        assert_eq!(report.terminal_state, Some(TerminalState::Blocked));
        assert_eq!(report.current_stage, Some(Stage::Validation));
        assert_eq!(report.transitions.len(), 4);
    }
}
