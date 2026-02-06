// projects/products/stable/core/launcher/src/launcher.rs
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub(crate) struct Launcher {
    #[serde(default = "default_startup_timeout")]
    pub(crate) startup_timeout_ms: u64,
    #[serde(default = "default_shutdown_grace")]
    pub(crate) shutdown_grace_ms: u64,
}

pub(crate) fn default_startup_timeout() -> u64 {
    15_000
}
pub(crate) fn default_shutdown_grace() -> u64 {
    2_500
}
