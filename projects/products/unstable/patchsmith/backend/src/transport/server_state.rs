use crate::plan::patch_plan::PatchPlan;

pub struct ServerState {
    pub current_plan: Option<PatchPlan>,
}
