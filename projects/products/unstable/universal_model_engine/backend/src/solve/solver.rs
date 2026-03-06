use crate::diagnostics::backend_error::BackendError;
use crate::model::state::State;
use crate::transitions::transition::Transition;
use crate::transitions::transition_engine::TransitionEngine;

pub struct Solver;

impl Solver {
    pub fn next_transition(
        step: u64,
        seed: u64,
        state: &State,
    ) -> Result<Transition, BackendError> {
        TransitionEngine::choose(step, seed, state.variable_ids())
    }
}
