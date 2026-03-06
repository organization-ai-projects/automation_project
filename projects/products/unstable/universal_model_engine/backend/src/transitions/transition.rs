use crate::model::var_id::VarId;
use crate::transitions::transition_id::TransitionId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transition {
    pub id: TransitionId,
    pub target_var: VarId,
    pub delta: i64,
}

impl Transition {
    pub fn transition_label(&self) -> String {
        self.id.0.clone()
    }
}
