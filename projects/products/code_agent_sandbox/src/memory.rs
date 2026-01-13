use std::{fs::OpenOptions, io::Write, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryEvent {
    pub ts: String,
    pub run_id: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

pub fn append_event(path: &Path, ev: &MemoryEvent) -> Result<()> {
    let mut f = OpenOptions::new().create(true).append(true).open(path)?;
    let line = serde_json::to_string(ev)?;
    writeln!(f, "{line}")?;
    Ok(())
}
