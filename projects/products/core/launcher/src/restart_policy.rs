// projects/products/core/launcher/src/restart_policy.rs
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum RestartPolicy {
    Never,
    OnFailure,
    Always,
}
pub(crate) fn default_restart() -> RestartPolicy {
    RestartPolicy::OnFailure
}
pub(crate) fn default_backoff() -> u64 {
    500
}
