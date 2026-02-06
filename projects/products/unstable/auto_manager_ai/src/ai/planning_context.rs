// projects/products/unstable/auto_manager_ai/src/ai/planning_context.rs

use crate::adapters::{CiContext, GhContext, RepoContext};

/// Context for planning
pub struct PlanningContext {
    pub repo: RepoContext,
    #[allow(dead_code)]
    pub gh: GhContext,
    #[allow(dead_code)]
    pub ci: CiContext,
}
