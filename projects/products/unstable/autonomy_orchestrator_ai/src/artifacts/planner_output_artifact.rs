// projects/products/unstable/autonomy_orchestrator_ai/src/artifacts/planner_output_artifact.rs
use crate::planner_output::PlannerOutput;

#[derive(Debug, Clone)]
pub struct PlannerOutputArtifact {
    pub source_path: String,
    pub payload: PlannerOutput,
}
