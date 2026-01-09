#[cfg(test)]
mod tests {
    use watcher::config::{
        ComponentConfig, LogLevel, LoggingConfig, RestartConfig, RestartPolicy, WatcherConfig,
    };

    #[test]
    fn test_valid_config() {
        let config = WatcherConfig {
            components: vec![ComponentConfig {
                name: "engine".to_string(),
                ping_interval: 10,
                ping: watcher::config::PingConfig::Systemd {
                    unit: "engine.service".to_string(),
                },
                restart: RestartConfig {
                    policy: RestartPolicy::Always,
                    systemd_unit: Some("engine.service".to_string()),
                    backoff_min_secs: 1,
                    backoff_max_secs: 60,
                    max_consecutive_restarts: None,
                },
            }],
            logging: LoggingConfig {
                log_file: "watcher.log".to_string().into(),
                log_level: LogLevel::Info,
            },
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_ping_interval() {
        let config = WatcherConfig {
            components: vec![ComponentConfig {
                name: "engine".to_string(),
                ping_interval: 0,
                ping: watcher::config::PingConfig::Systemd {
                    unit: "engine.service".to_string(),
                },
                restart: RestartConfig {
                    policy: RestartPolicy::Always,
                    systemd_unit: Some("engine.service".to_string()),
                    backoff_min_secs: 1,
                    backoff_max_secs: 60,
                    max_consecutive_restarts: None,
                },
            }],
            logging: LoggingConfig {
                log_file: "watcher.log".to_string().into(),
                log_level: LogLevel::Info,
            },
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_restart_policy() {
        let config = WatcherConfig {
            components: vec![ComponentConfig {
                name: "engine".to_string(),
                ping_interval: 10,
                ping: watcher::config::PingConfig::Systemd {
                    unit: "engine.service".to_string(),
                },
                restart: RestartConfig {
                    policy: RestartPolicy::OnFailure,
                    systemd_unit: Some("engine.service".to_string()),
                    backoff_min_secs: 1,
                    backoff_max_secs: 60,
                    max_consecutive_restarts: None,
                },
            }],
            logging: LoggingConfig {
                log_file: "watcher.log".to_string().into(),
                log_level: LogLevel::Info,
            },
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_log_level() {
        let config = WatcherConfig {
            components: vec![ComponentConfig {
                name: "engine".to_string(),
                ping_interval: 10,
                ping: watcher::config::PingConfig::Systemd {
                    unit: "engine.service".to_string(),
                },
                restart: RestartConfig {
                    policy: RestartPolicy::Always,
                    systemd_unit: Some("engine.service".to_string()),
                    backoff_min_secs: 1,
                    backoff_max_secs: 60,
                    max_consecutive_restarts: None,
                },
            }],
            logging: LoggingConfig {
                log_file: "watcher.log".to_string().into(),
                log_level: LogLevel::Debug, // Utilisation d'un niveau valide ici
            },
        };

        assert!(config.validate().is_ok());
    }
}
