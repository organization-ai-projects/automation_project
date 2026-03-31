use crate::model::event_id::EventId;
use crate::model::machine_id::MachineId;
use crate::model::state_id::StateId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transition {
    pub event: EventId,
    pub target: StateId,
    pub guard: Option<String>,
    pub action: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Machine {
    pub id: MachineId,
    pub initial_state: StateId,
    pub states: Vec<StateId>,
    pub events: Vec<EventId>,
    pub transitions: BTreeMap<String, Vec<Transition>>,
    pub variables: BTreeMap<String, i64>,
}

impl Machine {
    pub fn transition_key(state: &StateId, event: &EventId) -> String {
        format!("{}:{}", state.0, event.0)
    }

    pub fn get_transitions(&self, state: &StateId, event: &EventId) -> Vec<&Transition> {
        let key = Self::transition_key(state, event);
        self.transitions
            .get(&key)
            .map(|ts| ts.iter().collect())
            .unwrap_or_default()
    }

    pub fn has_state(&self, state: &StateId) -> bool {
        self.states.contains(state)
    }

    pub fn has_event(&self, event: &EventId) -> bool {
        self.events.contains(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_machine() -> Machine {
        let mut transitions = BTreeMap::new();
        transitions.insert(
            Machine::transition_key(&StateId("idle".into()), &EventId("start".into())),
            vec![Transition {
                event: EventId("start".into()),
                target: StateId("running".into()),
                guard: None,
                action: None,
            }],
        );
        transitions.insert(
            Machine::transition_key(&StateId("running".into()), &EventId("stop".into())),
            vec![Transition {
                event: EventId("stop".into()),
                target: StateId("idle".into()),
                guard: None,
                action: None,
            }],
        );
        Machine {
            id: MachineId("test".into()),
            initial_state: StateId("idle".into()),
            states: vec![StateId("idle".into()), StateId("running".into())],
            events: vec![EventId("start".into()), EventId("stop".into())],
            transitions,
            variables: BTreeMap::new(),
        }
    }

    #[test]
    fn get_transitions_returns_matching() {
        let m = sample_machine();
        let ts = m.get_transitions(&StateId("idle".into()), &EventId("start".into()));
        assert_eq!(ts.len(), 1);
        assert_eq!(ts[0].target, StateId("running".into()));
    }

    #[test]
    fn get_transitions_returns_empty_for_missing() {
        let m = sample_machine();
        let ts = m.get_transitions(&StateId("idle".into()), &EventId("stop".into()));
        assert!(ts.is_empty());
    }

    #[test]
    fn has_state_and_event() {
        let m = sample_machine();
        assert!(m.has_state(&StateId("idle".into())));
        assert!(!m.has_state(&StateId("unknown".into())));
        assert!(m.has_event(&EventId("start".into())));
        assert!(!m.has_event(&EventId("unknown".into())));
    }
}
