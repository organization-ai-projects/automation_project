use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum PingConfig {
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
