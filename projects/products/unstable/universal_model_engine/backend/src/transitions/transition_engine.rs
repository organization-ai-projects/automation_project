use crate::diagnostics::backend_error::BackendError;
use crate::model::var_id::VarId;
use crate::transitions::transition::Transition;
use crate::transitions::transition_id::TransitionId;

pub struct TransitionEngine;

impl TransitionEngine {
    pub fn choose(step: u64, seed: u64, variables: &[VarId]) -> Result<Transition, BackendError> {
        if variables.is_empty() {
            return Err(BackendError::Engine(
                "cannot choose transition without variables".to_string(),
            ));
        }

        let index = ((step ^ seed) as usize) % variables.len();
        let target_var = variables[index].clone();
        let delta = if ((step + seed) & 1) == 0 { 1 } else { -1 };

        Ok(Transition {
            id: TransitionId(format!("t{}", step)),
            target_var,
            delta,
        })
    }
}
