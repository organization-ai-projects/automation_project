use crate::model::event_id::EventId;
use crate::model::machine::Transition;
use crate::model::machine_id::MachineId;
use crate::model::state_id::StateId;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct Ast {
    pub machine_id: Option<MachineId>,
    pub initial_state: Option<StateId>,
    pub states: Vec<StateId>,
    pub events: Vec<EventId>,
    pub transitions: Vec<(StateId, Transition)>,
    pub variables: BTreeMap<String, i64>,
}
