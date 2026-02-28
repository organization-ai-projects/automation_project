#![allow(dead_code)]
use crate::diagnostics::error::ToolingError;
use crate::generate::pack_template::PackTemplate;
use std::path::Path;

pub fn generate_pack(pack_name: &str, out_dir: &Path) -> Result<(), ToolingError> {
    let template = PackTemplate::new(pack_name);
    let pack_dir = out_dir.join(pack_name.to_lowercase());
    std::fs::create_dir_all(&pack_dir).map_err(|e| ToolingError::Io(e.to_string()))?;
    let src_dir = pack_dir.join("src");
    std::fs::create_dir_all(&src_dir).map_err(|e| ToolingError::Io(e.to_string()))?;
    std::fs::write(src_dir.join("main.rs"), template.main_rs()).map_err(|e| ToolingError::Io(e.to_string()))?;
    std::fs::write(pack_dir.join("Cargo.toml"), template.cargo_toml()).map_err(|e| ToolingError::Io(e.to_string()))?;
    Ok(())
}
