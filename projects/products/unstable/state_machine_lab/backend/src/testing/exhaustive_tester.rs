use crate::diagnostics::error::BackendError;
use crate::model::machine::Machine;
use crate::model::state_id::StateId;
use crate::testing::test_report::TestReport;
use std::collections::BTreeSet;

pub struct ExhaustiveTester;

impl ExhaustiveTester {
    pub fn test(machine: &Machine) -> Result<TestReport, BackendError> {
        let mut visited_states: BTreeSet<StateId> = BTreeSet::new();
        let mut visited_transitions: BTreeSet<String> = BTreeSet::new();
        let mut violations = Vec::new();
        let mut queue: Vec<StateId> = vec![machine.initial_state.clone()];

        while let Some(state) = queue.pop() {
            if visited_states.contains(&state) {
                continue;
            }
            visited_states.insert(state.clone());

            let mut has_any_transition = false;
            for event in &machine.events {
                let transitions = machine.get_transitions(&state, event);
                for t in transitions {
                    has_any_transition = true;
                    let key = format!("{}:{}:{}", state.0, event.0, t.target.0);
                    visited_transitions.insert(key);
                    if !visited_states.contains(&t.target) {
                        queue.push(t.target.clone());
                    }
                }
            }

            if !has_any_transition {
                violations.push(format!("deadlock: state '{}' has no outgoing transitions", state.0));
            }
        }

        let unreachable: Vec<&StateId> = machine
            .states
            .iter()
            .filter(|s| !visited_states.contains(s))
            .collect();
        for s in &unreachable {
            violations.push(format!("unreachable: state '{}' not reachable from initial state", s.0));
        }

        Ok(TestReport::exhaustive(
            visited_states.len(),
            visited_transitions.len(),
            violations,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::event_id::EventId;
    use crate::model::machine::{Machine, Transition};
    use crate::model::machine_id::MachineId;
    use std::collections::BTreeMap;

    fn toggle_machine() -> Machine {
        let mut transitions = BTreeMap::new();
        transitions.insert(
            Machine::transition_key(&StateId("off".into()), &EventId("flip".into())),
            vec![Transition {
                event: EventId("flip".into()),
                target: StateId("on".into()),
                guard: None,
                action: None,
            }],
        );
        transitions.insert(
            Machine::transition_key(&StateId("on".into()), &EventId("flip".into())),
            vec![Transition {
                event: EventId("flip".into()),
                target: StateId("off".into()),
                guard: None,
                action: None,
            }],
        );
        Machine {
            id: MachineId("toggle".into()),
            initial_state: StateId("off".into()),
            states: vec![StateId("off".into()), StateId("on".into())],
            events: vec![EventId("flip".into())],
            transitions,
            variables: BTreeMap::new(),
        }
    }

    #[test]
    fn exhaustive_all_reachable_no_deadlock() {
        let m = toggle_machine();
        let report = ExhaustiveTester::test(&m).unwrap();
        assert!(report.passed);
        assert_eq!(report.states_visited, 2);
        assert_eq!(report.transitions_fired, 2);
    }

    #[test]
    fn exhaustive_deterministic_ordering() {
        let m = toggle_machine();
        let r1 = ExhaustiveTester::test(&m).unwrap();
        let r2 = ExhaustiveTester::test(&m).unwrap();
        assert_eq!(r1, r2);
    }

    #[test]
    fn exhaustive_detects_deadlock() {
        let mut transitions = BTreeMap::new();
        transitions.insert(
            Machine::transition_key(&StateId("a".into()), &EventId("go".into())),
            vec![Transition {
                event: EventId("go".into()),
                target: StateId("b".into()),
                guard: None,
                action: None,
            }],
        );
        let m = Machine {
            id: MachineId("dead".into()),
            initial_state: StateId("a".into()),
            states: vec![StateId("a".into()), StateId("b".into())],
            events: vec![EventId("go".into())],
            transitions,
            variables: BTreeMap::new(),
        };
        let report = ExhaustiveTester::test(&m).unwrap();
        assert!(!report.passed);
        assert!(report.violations.iter().any(|v| v.contains("deadlock")));
    }
}
