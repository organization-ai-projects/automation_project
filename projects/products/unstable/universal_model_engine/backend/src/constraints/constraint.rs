use crate::constraints::constraint_id::ConstraintId;
use crate::model::var_id::VarId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constraint {
    pub id: ConstraintId,
    pub target_var: VarId,
    pub min_value: i64,
}
