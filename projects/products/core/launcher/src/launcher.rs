// projects/products/core/launcher/src/launcher.rs
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Launcher {
    #[serde(default = "default_startup_timeout")]
    pub startup_timeout_ms: u64,
    #[serde(default = "default_shutdown_grace")]
    pub shutdown_grace_ms: u64,
}

pub fn default_startup_timeout() -> u64 {
    15_000
}
pub fn default_shutdown_grace() -> u64 {
    2_500
}
