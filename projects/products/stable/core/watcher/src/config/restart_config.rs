use serde::Deserialize;

use super::RestartPolicy;

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct RestartConfig {
    #[serde(default)]
    pub(crate) policy: RestartPolicy,

    /// Optional: override systemd unit for restart
    pub(crate) systemd_unit: Option<String>,

    /// Exponential backoff: min/max (seconds)
    #[serde(default = "default_backoff_min")]
    pub(crate) backoff_min_secs: u64,
    #[serde(default = "default_backoff_max")]
    pub(crate) backoff_max_secs: u64,

    /// Anti-loop: max consecutive restarts before a long pause (optional)
    #[serde(default)]
    pub(crate) max_consecutive_restarts: Option<u32>,
}

impl Default for RestartConfig {
    fn default() -> Self {
        Self {
            policy: RestartPolicy::OnFailure,
            systemd_unit: None,
            backoff_min_secs: default_backoff_min(),
            backoff_max_secs: default_backoff_max(),
            max_consecutive_restarts: Some(20),
        }
    }
}

fn default_backoff_min() -> u64 {
    1
}

fn default_backoff_max() -> u64 {
    60
}
