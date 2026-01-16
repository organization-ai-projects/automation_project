// projects/products/code_agent_sandbox/src/actions/action_result.rs
use serde::Serialize;

use common_json::Json;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionResult {
    pub ok: bool,
    pub kind: String,
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Json>,
}

impl ActionResult {
    pub fn success(
        kind: impl Into<String>,
        message: impl Into<String>,
        data: Option<Json>,
    ) -> Self {
        Self {
            ok: true,
            kind: kind.into(),
            message: message.into(),
            data,
        }
    }

    pub fn error(kind: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            ok: false,
            kind: kind.into(),
            message: message.into(),
            data: None,
        }
    }
}
