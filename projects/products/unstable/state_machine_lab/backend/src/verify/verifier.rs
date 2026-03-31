use crate::diagnostics::error::BackendError;
use crate::model::machine::Machine;
use crate::model::state_id::StateId;
use crate::verify::invariant::Invariant;
use std::collections::BTreeSet;

pub struct Verifier;

impl Verifier {
    pub fn check(machine: &Machine, invariants: &[Invariant]) -> Result<Vec<String>, BackendError> {
        let mut violations = Vec::new();
        for inv in invariants {
            match inv {
                Invariant::NoDeadlock => {
                    let dead = Self::find_deadlock_states(machine);
                    for s in dead {
                        violations.push(format!("deadlock: state '{}' has no outgoing transitions", s.0));
                    }
                }
                Invariant::StateReachable(target) => {
                    let reachable = Self::reachable_states(machine);
                    if !reachable.contains(target) {
                        violations.push(format!("unreachable: state '{}' is not reachable from initial state", target.0));
                    }
                }
                Invariant::VariableBound { var, min, max } => {
                    if let Some(val) = machine.variables.get(var) {
                        if *val < *min || *val > *max {
                            violations.push(format!(
                                "variable bound: '{var}' value {val} out of [{min}, {max}]"
                            ));
                        }
                    }
                }
            }
        }
        Ok(violations)
    }

    pub fn validate_machine(machine: &Machine) -> Result<Vec<String>, BackendError> {
        let mut issues = Vec::new();
        if machine.states.is_empty() {
            issues.push("machine has no states".to_string());
        }
        if !machine.has_state(&machine.initial_state) {
            issues.push(format!(
                "initial state '{}' is not a declared state",
                machine.initial_state.0
            ));
        }
        for (key, transitions) in &machine.transitions {
            for t in transitions {
                if !machine.has_state(&t.target) {
                    issues.push(format!(
                        "transition in '{key}' targets undeclared state '{}'",
                        t.target.0
                    ));
                }
                if !machine.has_event(&t.event) {
                    issues.push(format!(
                        "transition in '{key}' uses undeclared event '{}'",
                        t.event.0
                    ));
                }
            }
        }
        Ok(issues)
    }

    fn find_deadlock_states(machine: &Machine) -> Vec<StateId> {
        let mut deadlocks = Vec::new();
        for state in &machine.states {
            let has_outgoing = machine.events.iter().any(|event| {
                !machine.get_transitions(state, event).is_empty()
            });
            if !has_outgoing {
                deadlocks.push(state.clone());
            }
        }
        deadlocks
    }

    fn reachable_states(machine: &Machine) -> BTreeSet<StateId> {
        let mut reachable = BTreeSet::new();
        let mut queue = vec![machine.initial_state.clone()];
        while let Some(state) = queue.pop() {
            if reachable.contains(&state) {
                continue;
            }
            reachable.insert(state.clone());
            for event in &machine.events {
                for t in machine.get_transitions(&state, event) {
                    if !reachable.contains(&t.target) {
                        queue.push(t.target.clone());
                    }
                }
            }
        }
        reachable
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::event_id::EventId;
    use crate::model::machine::{Machine, Transition};
    use crate::model::machine_id::MachineId;
    use std::collections::BTreeMap;

    fn simple_machine() -> Machine {
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
        Machine {
            id: MachineId("m".into()),
            initial_state: StateId("a".into()),
            states: vec![StateId("a".into()), StateId("b".into())],
            events: vec![EventId("go".into())],
            transitions,
            variables: BTreeMap::new(),
        }
    }

    #[test]
    fn detects_deadlock() {
        let m = simple_machine();
        let violations = Verifier::check(&m, &[Invariant::NoDeadlock]).unwrap();
        assert!(violations.iter().any(|v| v.contains("deadlock") && v.contains("b")));
    }

    #[test]
    fn detects_reachability() {
        let mut m = simple_machine();
        m.states.push(StateId("c".into()));
        let violations = Verifier::check(&m, &[Invariant::StateReachable(StateId("c".into()))]).unwrap();
        assert!(violations.iter().any(|v| v.contains("unreachable")));
    }

    #[test]
    fn validates_machine_ok() {
        let m = simple_machine();
        let issues = Verifier::validate_machine(&m).unwrap();
        assert!(issues.is_empty());
    }
}
