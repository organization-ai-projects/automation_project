// projects/products/unstable/protocol_builder/backend/src/io/fs_writer.rs
use anyhow::Result;
use std::path::Path;

/// Writes content atomically using a temp-file + rename pattern.
pub fn write_atomic(out_dir: &str, name: &str, content: &str) -> Result<()> {
    let dir = Path::new(out_dir);
    std::fs::create_dir_all(dir)?;
    let pid = std::process::id();
    let tmp_name = format!(".{}.tmp-{}", name, pid);
    let tmp_path = dir.join(&tmp_name);
    let final_path = dir.join(name);
    std::fs::write(&tmp_path, content.as_bytes())?;
    std::fs::rename(&tmp_path, &final_path)?;
    Ok(())
}
