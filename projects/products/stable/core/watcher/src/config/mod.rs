// projects/products/core/watcher/src/config/mod.rs
mod component_config;
mod logging;
mod ping_config;
mod restart_config;
mod restart_policy;
mod watcher_config;

pub(crate) use component_config::ComponentConfig;
pub(crate) use logging::{LoggingConfig, initialize_logger};
pub(crate) use ping_config::PingConfig;
pub(crate) use restart_config::RestartConfig;
pub(crate) use restart_policy::RestartPolicy;
pub(crate) use watcher_config::WatcherConfig;
