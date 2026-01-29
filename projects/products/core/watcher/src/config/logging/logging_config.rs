use serde::Deserialize;
use std::path::PathBuf;

use super::LogLevel;

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoggingConfig {
    #[serde(default = "default_log_file")]
    pub(crate) log_file: PathBuf,

    #[serde(default = "default_log_level")]
    pub(crate) log_level: LogLevel,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            log_file: default_log_file(),
            log_level: default_log_level(),
        }
    }
}

fn default_log_file() -> PathBuf {
    PathBuf::from("watcher.log")
}

fn default_log_level() -> LogLevel {
    LogLevel::Info
}
