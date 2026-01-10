// projects/products/code_agent_sandbox/src/actions.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Action {
    ReadFile {
        path: String,
    },
    ListDir {
        path: String,
        #[serde(default)]
        max_depth: usize,
    },
    WriteFile {
        path: String,
        contents: String,
        #[serde(default)]
        create_dirs: bool,
    },
    ApplyUnifiedDiff {
        path: String,
        unified_diff: String,
    },
    RunCargo {
        subcommand: String,
        #[serde(default)]
        args: Vec<String>,
    },
    GenerateCode {
        language: String,
        code: String,
    },
}

impl Action {
    pub fn estimated_file_touch_count(&self) -> usize {
        match self {
            Action::ReadFile { .. } => 1,
            Action::ListDir { .. } => 1,
            Action::WriteFile { .. } => 1,
            Action::ApplyUnifiedDiff { .. } => 1,
            Action::RunCargo { .. } => 0,
            Action::GenerateCode { .. } => 1,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionResult {
    pub ok: bool,
    pub kind: String,
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl ActionResult {
    pub fn success(
        kind: impl Into<String>,
        message: impl Into<String>,
        data: Option<serde_json::Value>,
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
