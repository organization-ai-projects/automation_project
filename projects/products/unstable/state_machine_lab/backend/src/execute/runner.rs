use crate::diagnostics::error::BackendError;
use crate::execute::step_result::StepResult;
use crate::model::event_id::EventId;
use crate::model::machine::Machine;
use crate::model::state_id::StateId;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Runner {
    pub machine: Machine,
    pub current_state: StateId,
    pub variables: BTreeMap<String, i64>,
    pub step_count: u64,
    pub history: Vec<StepResult>,
}

impl Runner {
    pub fn new(machine: Machine) -> Self {
        let current_state = machine.initial_state.clone();
        let variables = machine.variables.clone();
        Self {
            machine,
            current_state,
            variables,
            step_count: 0,
            history: Vec::new(),
        }
    }

    pub fn step(&mut self, event: &EventId) -> Result<StepResult, BackendError> {
        let transitions = self.machine.get_transitions(&self.current_state, event);
        let transition = transitions.first().ok_or_else(|| {
            BackendError::Engine(format!(
                "no transition from state '{}' on event '{}'",
                self.current_state.0, event.0
            ))
        })?;
        let guard = transition.guard.clone();
        let action = transition.action.clone();
        let target = transition.target.clone();
        if let Some(ref guard) = guard {
            if !self.evaluate_guard(guard) {
                return Err(BackendError::Engine(format!(
                    "guard '{guard}' failed in state '{}'",
                    self.current_state.0
                )));
            }
        }
        if let Some(ref action) = action {
            self.apply_action(action);
        }
        let previous_state = self.current_state.clone();
        self.current_state = target;
        self.step_count += 1;
        let result = StepResult {
            step: self.step_count,
            previous_state,
            event: event.clone(),
            next_state: self.current_state.clone(),
            variables: self.variables.clone(),
        };
        self.history.push(result.clone());
        Ok(result)
    }

    pub fn run_events(&mut self, events: &[EventId]) -> Result<Vec<StepResult>, BackendError> {
        let mut results = Vec::new();
        for event in events {
            results.push(self.step(event)?);
        }
        Ok(results)
    }

    fn evaluate_guard(&self, guard: &str) -> bool {
        // Simple guard evaluation: "var<val" or "var>val"
        if let Some((var_name, val_str)) = guard.split_once('<') {
            if let (Some(current), Ok(limit)) =
                (self.variables.get(var_name), val_str.parse::<i64>())
            {
                return *current < limit;
            }
        }
        if let Some((var_name, val_str)) = guard.split_once('>') {
            if let (Some(current), Ok(limit)) =
                (self.variables.get(var_name), val_str.parse::<i64>())
            {
                return *current > limit;
            }
        }
        true
    }

    fn apply_action(&mut self, action: &str) {
        // Simple action: "var+=val" or "var-=val" or "var=val"
        if let Some((var_name, val_str)) = action.split_once("+=") {
            if let Ok(delta) = val_str.parse::<i64>() {
                let entry = self.variables.entry(var_name.to_string()).or_insert(0);
                *entry += delta;
            }
        } else if let Some((var_name, val_str)) = action.split_once("-=") {
            if let Ok(delta) = val_str.parse::<i64>() {
                let entry = self.variables.entry(var_name.to_string()).or_insert(0);
                *entry -= delta;
            }
        } else if let Some((var_name, val_str)) = action.split_once('=') {
            if let Ok(val) = val_str.parse::<i64>() {
                self.variables.insert(var_name.to_string(), val);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::machine::{Machine, Transition};
    use crate::model::machine_id::MachineId;

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
    fn runner_steps_through_toggle() {
        let mut runner = Runner::new(toggle_machine());
        assert_eq!(runner.current_state, StateId("off".into()));
        let r = runner.step(&EventId("flip".into())).unwrap();
        assert_eq!(r.next_state, StateId("on".into()));
        let r2 = runner.step(&EventId("flip".into())).unwrap();
        assert_eq!(r2.next_state, StateId("off".into()));
    }

    #[test]
    fn runner_rejects_invalid_event() {
        let mut runner = Runner::new(toggle_machine());
        let err = runner.step(&EventId("unknown".into()));
        assert!(err.is_err());
    }

    #[test]
    fn run_events_deterministic() {
        let events = vec![
            EventId("flip".into()),
            EventId("flip".into()),
            EventId("flip".into()),
        ];
        let mut r1 = Runner::new(toggle_machine());
        let mut r2 = Runner::new(toggle_machine());
        let res1 = r1.run_events(&events).unwrap();
        let res2 = r2.run_events(&events).unwrap();
        assert_eq!(res1, res2);
    }
}
