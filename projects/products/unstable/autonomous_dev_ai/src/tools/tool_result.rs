// projects/products/unstable/autonomous_dev_ai/src/tools/tool_result.rs
use serde::{Deserialize, Serialize};

/// Tool execution result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}
