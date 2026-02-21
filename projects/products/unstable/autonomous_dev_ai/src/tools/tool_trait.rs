//projects/products/unstable/autonomous_dev_ai/src/tools/tool_trait.rs
use super::ToolResult;
use crate::error::AgentResult;

/// Tool trait - all tools must implement this.
pub trait Tool {
    fn name(&self) -> &str;
    fn execute(&self, args: &[String]) -> AgentResult<ToolResult>;
    fn is_reversible(&self) -> bool;
}
