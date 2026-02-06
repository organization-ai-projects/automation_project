// projects/products/unstable/auto_manager_ai/src/ai/planning_context.rs

use crate::adapters::{RepoContext, GhContext, CiContext};

/// Context for planning
pub struct PlanningContext {
    pub repo: RepoContext,
    pub gh: GhContext,
    pub ci: CiContext,
}
