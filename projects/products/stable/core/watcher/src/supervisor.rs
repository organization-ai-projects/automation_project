// projects/products/core/watcher/src/supervisor.rs
use std::sync::Arc;
use std::time::Duration;

use log::{error, info, warn};
use reqwest::Client;
use tokio::{process::Command, time::sleep};
use tokio_util::sync::CancellationToken;

use crate::config::{ComponentConfig, PingConfig, RestartPolicy};

const COOLDOWN_ON_RESTART_LOOP_SECS: u64 = 120; // 2 min: anti "restart storm"
const SYSTEMCTL: &str = "systemctl";
const PGREP: &str = "pgrep";

#[derive(Clone)]
pub(crate) struct Supervisor {
    client: Arc<Client>,
}

impl Supervisor {
    pub(crate) fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(2))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client: Arc::new(client),
        }
    }

    pub(crate) async fn supervise_component(
        &self,
        component: ComponentConfig,
        shutdown: CancellationToken,
    ) {
        let mut consecutive_failures: u32 = 0;
        let mut consecutive_restarts: u32 = 0;

        let mut backoff_secs = component.restart.backoff_min_secs.max(1);

        loop {
            let alive = self.ping_component(&component).await;

            if alive {
                if consecutive_failures > 0 || consecutive_restarts > 0 {
                    info!(
                        "component={} event=back_online failures={} restarts={}",
                        component.name, consecutive_failures, consecutive_restarts
                    );
                }

                consecutive_failures = 0;
                consecutive_restarts = 0;
                backoff_secs = component.restart.backoff_min_secs.max(1);

                tokio::select! {
                    _ = shutdown.cancelled() => {
                        log::info!("component={} event=shutdown_requested", component.name);
                        break;
                    }
                    _ = sleep(Duration::from_secs(component.ping_interval.max(1))) => {}
                }
                continue;
            }

            consecutive_failures = consecutive_failures.saturating_add(1);
            warn!(
                "component={} event=ping_failed failures={}",
                component.name, consecutive_failures
            );

            // Decide whether to restart
            let should_restart = match component.restart.policy {
                RestartPolicy::Never => false,
                RestartPolicy::Always => true,
                RestartPolicy::OnFailure => true,
            };

            if !should_restart {
                tokio::select! {
                    _ = shutdown.cancelled() => {
                        log::info!("component={} event=shutdown_requested", component.name);
                        break;
                    }
                    _ = sleep(Duration::from_secs(component.ping_interval.max(1))) => {}
                }
                continue;
            }

            // Anti-loop: if we exceed a threshold of consecutive restarts, cooldown
            if let Some(max) = component.restart.max_consecutive_restarts
                && consecutive_restarts >= max
            {
                error!(
                    "component={} event=too_many_restarts restarts={} cooldown={}s",
                    component.name, consecutive_restarts, COOLDOWN_ON_RESTART_LOOP_SECS
                );
                tokio::select! {
                    _ = shutdown.cancelled() => {
                        log::info!("component={} event=shutdown_requested", component.name);
                        break;
                    }
                    _ = sleep(Duration::from_secs(COOLDOWN_ON_RESTART_LOOP_SECS)) => {}
                }
                // Partial reset for a clean retry
                consecutive_restarts = 0;
                backoff_secs = component.restart.backoff_min_secs.max(1);
                continue;
            }

            // Attempt the restart
            match self.restart_component(&component).await {
                Ok(()) => {
                    consecutive_restarts = consecutive_restarts.saturating_add(1);
                    warn!(
                        "component={} event=restarted restarts={}",
                        component.name, consecutive_restarts
                    );
                }
                Err(e) => {
                    error!(
                        "component={} event=restart_failed error=\"{}\"",
                        component.name, e
                    );
                }
            }

            // Exponential backoff before the next ping
            let max_backoff = component
                .restart
                .backoff_max_secs
                .max(component.restart.backoff_min_secs)
                .max(1);
            tokio::select! {
                _ = shutdown.cancelled() => {
                    log::info!("component={} event=shutdown_requested", component.name);
                    break;
                }
                _ = sleep(Duration::from_secs(backoff_secs)) => {}
            }
            backoff_secs = (backoff_secs.saturating_mul(2)).min(max_backoff);
        }
    }

    async fn ping_component(&self, component: &ComponentConfig) -> bool {
        match &component.ping {
            PingConfig::Disabled => {
                // "always alive" to never trigger a restart
                true
            }

            PingConfig::Systemd { unit } => systemd_is_active(unit).await,

            PingConfig::Process { process_name } => process_is_running(process_name).await,

            PingConfig::Http { url } => self.http_is_ok(url).await,
        }
    }

    async fn http_is_ok(&self, url: &str) -> bool {
        match self.client.get(url).send().await {
            Ok(resp) => resp.status().is_success(),
            Err(e) => {
                warn!("[http] GET {} failed: {}", url, e);
                false
            }
        }
    }

    async fn restart_component(&self, component: &ComponentConfig) -> Result<(), String> {
        let unit = if let Some(u) = component.restart.systemd_unit.as_deref() {
            u.to_string()
        } else if let PingConfig::Systemd { unit } = &component.ping {
            unit.clone()
        } else {
            format!("{}.service", component.name)
        };

        systemd_restart(&unit).await
    }
}

impl Default for Supervisor {
    fn default() -> Self {
        Self::new()
    }
}

/* ---------------- systemd helpers ---------------- */

async fn systemd_is_active(unit: &str) -> bool {
    // systemctl is-active --quiet <unit> => exit code 0 if active
    match Command::new(SYSTEMCTL)
        .arg("is-active")
        .arg("--quiet")
        .arg(unit)
        .output()
        .await
    {
        Ok(out) => out.status.success(),
        Err(e) => {
            warn!("[systemd] is-active failed for {}: {}", unit, e);
            false
        }
    }
}

async fn systemd_restart(unit: &str) -> Result<(), String> {
    let out = Command::new(SYSTEMCTL)
        .arg("restart")
        .arg(unit)
        .output()
        .await
        .map_err(|e| format!("systemctl restart {} failed: {}", unit, e))?;

    if out.status.success() {
        Ok(())
    } else {
        Err(format!(
            "systemctl restart {} failed: {}",
            unit,
            String::from_utf8_lossy(&out.stderr)
        ))
    }
}

/* ---------------- process helpers ---------------- */

async fn process_is_running(process_name: &str) -> bool {
    // pgrep -x <process_name> => non-empty stdout if found
    match Command::new(PGREP)
        .arg("-x")
        .arg(process_name)
        .output()
        .await
    {
        Ok(out) => out.status.success() && !out.stdout.is_empty(),
        Err(e) => {
            warn!("[process] pgrep failed for {}: {}", process_name, e);
            false
        }
    }
}
