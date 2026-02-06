// projects/products/code_agent_sandbox/src/memory.rs
use std::{fs::OpenOptions, io::Write, path::Path};

use anyhow::Result;
use common_json::{Json, to_json_string};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MemoryEvent {
    pub(crate) ts: String,
    pub(crate) run_id: String,
    pub(crate) kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) metadata: Option<Json>,
}

pub(crate) fn append_event(path: &Path, ev: &MemoryEvent) -> Result<()> {
    let mut f = OpenOptions::new().create(true).append(true).open(path)?;
    let line = to_json_string(ev)?;
    writeln!(f, "{line}")?;
    Ok(())
}
