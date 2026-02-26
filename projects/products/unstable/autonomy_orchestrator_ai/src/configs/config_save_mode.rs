// projects/products/unstable/autonomy_orchestrator_ai/src/configs/config_save_mode.rs
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum ConfigSaveMode {
    Auto(PathBuf),
    Ron(PathBuf),
    Bin(PathBuf),
    Json(PathBuf),
}
