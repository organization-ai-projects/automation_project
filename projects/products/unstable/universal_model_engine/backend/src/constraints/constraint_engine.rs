use crate::constraints::constraint::Constraint;
use crate::diagnostics::backend_error::BackendError;
use crate::model::state::State;
use crate::transitions::transition::Transition;

pub struct ConstraintEngine;

impl ConstraintEngine {
    pub fn apply(
        state: &mut State,
        transition: &Transition,
        constraints: &[Constraint],
    ) -> Result<i64, BackendError> {
        let updated_value = state.apply_delta(&transition.target_var, transition.delta);
        let value_after = state.get(&transition.target_var).ok_or_else(|| {
            BackendError::Engine(format!(
                "updated variable '{}' not found in state",
                transition.target_var.0
            ))
        })?;
        if value_after != updated_value {
            return Err(BackendError::Engine(
                "state update mismatch after transition".to_string(),
            ));
        }

        for constraint in constraints {
            if constraint.target_var == transition.target_var && value_after < constraint.min_value
            {
                return Err(BackendError::Constraint(format!(
                    "constraint {} violated for {}: {} < {}",
                    constraint.id.0, constraint.target_var.0, value_after, constraint.min_value
                )));
            }
        }

        Ok(value_after)
    }
}
