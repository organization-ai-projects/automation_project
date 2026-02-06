use serde::Deserialize;

use super::{PingConfig, RestartConfig};

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct ComponentConfig {
    /// Logical name (for logs, UI, etc.)
    pub(crate) name: String,

    /// Interval between two checks (seconds)
    #[serde(default = "default_ping_interval")]
    pub(crate) ping_interval: u64,

    /// How to check if the component is alive
    #[serde(default)]
    pub(crate) ping: PingConfig,

    /// How to restart the component
    #[serde(default)]
    pub(crate) restart: RestartConfig,
}

fn default_ping_interval() -> u64 {
    5
}
