use super::error::ExpertError;
use super::execution_context::ExecutionContext;
use super::expert_id::ExpertId;
use super::expert_metadata::ExpertMetadata;
use super::expert_output::ExpertOutput;
use super::task::Task;

pub trait Expert: Send + Sync {
    fn id(&self) -> &ExpertId;
    fn metadata(&self) -> &ExpertMetadata;
    fn can_handle(&self, task: &Task) -> bool;
    fn execute(&self, task: &Task, context: &ExecutionContext)
    -> Result<ExpertOutput, ExpertError>;
}
