use anyhow::bail;
// projects/products/core/watcher/src/config.rs
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WatcherConfig {
    #[serde(default)]
    pub components: Vec<ComponentConfig>,

    #[serde(default)]
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ComponentConfig {
    /// Logical name (for logs, UI, etc.)
    pub name: String,

    /// Interval between two checks (seconds)
    #[serde(default = "default_ping_interval")]
    pub ping_interval: u64,

    /// How to check if the component is alive
    #[serde(default)]
    pub ping: PingConfig,

    /// How to restart the component
    #[serde(default)]
    pub restart: RestartConfig,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum PingConfig {
    /// systemd: check `systemctl is-active --quiet <unit>`
    Systemd { unit: String },

    /// HTTP GET (expects a 2xx) e.g., http://127.0.0.1:3030/health
    Http { url: String },

    /// process: check via `pgrep -x <process_name>` (dev/fallback)
    Process { process_name: String },

    /// Disables ping (not recommended, but useful during bootstrap)
    Disabled,
}

impl Default for PingConfig {
    fn default() -> Self {
        // If you don't specify anything in the TOML, we assume systemd unit = name
        PingConfig::Systemd {
            unit: "CHANGE_ME.service".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
#[derive(Default)]
pub enum RestartPolicy {
    Always,
    #[default]
    OnFailure,
    Never,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct RestartConfig {
    #[serde(default)]
    pub policy: RestartPolicy,

    /// Optional: override systemd unit for restart
    pub systemd_unit: Option<String>,

    /// Exponential backoff: min/max (seconds)
    #[serde(default = "default_backoff_min")]
    pub backoff_min_secs: u64,
    #[serde(default = "default_backoff_max")]
    pub backoff_max_secs: u64,

    /// Anti-loop: max consecutive restarts before a long pause (optional)
    #[serde(default)]
    pub max_consecutive_restarts: Option<u32>,
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

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct LoggingConfig {
    #[serde(default = "default_log_file")]
    pub log_file: PathBuf,

    #[serde(default = "default_log_level")]
    pub log_level: LogLevel,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            log_file: default_log_file(),
            log_level: default_log_level(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn as_str(&self) -> &str {
        match self {
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        }
    }
}

fn default_ping_interval() -> u64 {
    5
}
fn default_backoff_min() -> u64 {
    1
}
fn default_backoff_max() -> u64 {
    60
}
fn default_log_file() -> PathBuf {
    PathBuf::from("watcher.log")
}
fn default_log_level() -> LogLevel {
    LogLevel::Info
}

impl WatcherConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        let mut config: WatcherConfig = toml::from_str(&content)?;
        config.normalize_defaults();
        config.validate()?;
        Ok(config)
    }

    fn normalize_defaults(&mut self) {
        // Makes the default PingConfig useful: if the user did not set ping,
        // we replace CHANGE_ME with the component name.
        for c in &mut self.components {
            if let PingConfig::Systemd { unit } = &mut c.ping
                && unit == "CHANGE_ME.service"
            {
                *unit = format!("{}.service", c.name);
            }
        }
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        // ---- components ----
        for c in &self.components {
            let name = c.name.trim();
            if name.is_empty() {
                bail!("Component name cannot be empty");
            }

            if c.ping_interval == 0 {
                bail!("Ping interval for component '{}' must be > 0", c.name);
            }

            if c.restart.backoff_min_secs == 0 || c.restart.backoff_max_secs == 0 {
                bail!("Backoff min/max must be > 0 for component '{}'", c.name);
            }

            if c.restart.backoff_min_secs > c.restart.backoff_max_secs {
                bail!("Backoff min > max for component '{}'", c.name);
            }

            // Validation ping
            match &c.ping {
                PingConfig::Systemd { unit } => {
                    if unit.trim().is_empty() {
                        bail!("systemd.unit is empty for component '{}'", c.name);
                    }
                }
                PingConfig::Http { url } => {
                    let u = url.trim();
                    if u.is_empty() {
                        bail!("ping.http.url is empty for component '{}'", c.name);
                    }
                    if !(u.starts_with("http://") || u.starts_with("https://")) {
                        bail!(
                            "ping.http.url must start with http:// or https:// for component '{}'",
                            c.name
                        );
                    }
                }
                PingConfig::Process { process_name } => {
                    if process_name.trim().is_empty() {
                        bail!(
                            "ping.process.process_name is empty for component '{}'",
                            c.name
                        );
                    }
                }
                PingConfig::Disabled => {}
            }

            // Policy consistency
            if matches!(c.restart.policy, RestartPolicy::Never)
                && (c.restart.max_consecutive_restarts.unwrap_or(0) > 0)
            {
                // not an error, but conceptually unnecessary
                // you can leave it as a warning in the logs later
            }
        }

        // ---- logging ----
        if self.logging.log_file.as_os_str().is_empty() {
            bail!("Log file path cannot be empty");
        }

        Ok(())
    }
}
