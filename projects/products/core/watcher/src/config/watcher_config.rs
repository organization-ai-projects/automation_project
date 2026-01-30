use anyhow::bail;
use serde::Deserialize;
use std::{fs, path::Path};

use super::{ComponentConfig, LoggingConfig, PingConfig, RestartPolicy};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WatcherConfig {
    #[serde(default)]
    pub(crate) components: Vec<ComponentConfig>,

    #[serde(default)]
    pub(crate) logging: LoggingConfig,
}

impl WatcherConfig {
    pub(crate) fn load_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
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

    pub(crate) fn validate(&self) -> anyhow::Result<()> {
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
