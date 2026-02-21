// projects/products/unstable/autonomous_dev_ai/src/config_loader.rs
use crate::agent_config::AgentConfig;
use crate::error::AgentResult;
use crate::persistence::{load_bin, load_ron, save_bin, save_ron};
use std::path::Path;

/// Load configuration with fallback mechanism
/// 1. Try to load .bin file
/// 2. If missing or incompatible, load .ron file
/// 3. Rebuild .bin deterministically
pub fn load_config<P: AsRef<Path>>(base_path: P) -> AgentResult<AgentConfig> {
    let base = base_path.as_ref();
    let bin_path = base.with_extension("bin");
    let ron_path = base.with_extension("ron");

    // Try loading .bin first
    if bin_path.exists() {
        match load_bin(&bin_path) {
            Ok(config) => return Ok(config),
            Err(e) => {
                tracing::warn!("Failed to load .bin config: {}, falling back to .ron", e);
            }
        }
    }

    // Fall back to .ron
    if ron_path.exists() {
        let config = load_ron(&ron_path)?;

        // Try to rebuild .bin
        if let Err(e) = save_bin(&bin_path, &config) {
            tracing::warn!("Failed to save .bin config: {}", e);
        }

        Ok(config)
    } else {
        // No config found, use default
        let config = AgentConfig::default();

        // Save both formats
        save_ron(&ron_path, &config)?;
        save_bin(&bin_path, &config)?;

        Ok(config)
    }
}
