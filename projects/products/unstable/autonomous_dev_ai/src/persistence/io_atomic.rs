// projects/products/unstable/autonomous_dev_ai/src/persistence/io_atomic.rs
use std::fs;
use std::path::Path;

use serde::Serialize;

use crate::error::{AgentError, AgentResult};

pub(crate) fn write_json_atomic<P: AsRef<Path>, T: Serialize>(
    path: P,
    value: &T,
) -> AgentResult<()> {
    let content = serde_json::to_string_pretty(value)
        .map_err(|e| AgentError::Serialization(e.to_string()))?;
    write_string_atomic(path, &content)
}

pub(crate) fn write_string_atomic<P: AsRef<Path>>(path: P, content: &str) -> AgentResult<()> {
    write_bytes_atomic(path, content.as_bytes())
}

pub(crate) fn write_bytes_atomic<P: AsRef<Path>>(path: P, bytes: &[u8]) -> AgentResult<()> {
    let path = path.as_ref();
    let tmp_path = path.with_extension(format!(
        "{}.tmp",
        path.extension().and_then(|e| e.to_str()).unwrap_or("data")
    ));
    fs::write(&tmp_path, bytes)?;
    fs::rename(&tmp_path, path)?;
    Ok(())
}
