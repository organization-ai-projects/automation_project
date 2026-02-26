// projects/products/unstable/autonomy_orchestrator_ai/src/configs/config_load_mode.rs
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum ConfigLoadMode {
    Auto(PathBuf),
    Ron(PathBuf),
    Bin(PathBuf),
    Json(PathBuf),
}
