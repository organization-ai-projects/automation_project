// projects/products/stable/core/launcher/src/supervisor/readiness.rs
use std::{
    thread,
    time::{Duration, Instant},
};

use anyhow::{Result, bail};

use crate::{logging::log_message, service::Service};

pub(crate) fn wait_ready(svc: &Service, timeout: Duration) -> Result<()> {
    let Some(url) = &svc.ready_http else {
        return Ok(());
    };

    let start = Instant::now();
    while start.elapsed() < timeout {
        if is_http_healthy(url) {
            log_message(&format!("✅ ready: {}", svc.name), false, &svc.name);
            return Ok(());
        }
        thread::sleep(Duration::from_millis(200));
    }

    bail!("service `{}` not ready within {:?}", svc.name, timeout);
}

fn is_http_healthy(url: &str) -> bool {
    let config = ureq::Agent::config_builder()
        .timeout_per_call(Some(Duration::from_millis(800)))
        .build();
    let agent = config.new_agent();
    let resp = agent.get(url).call();
    matches!(resp, Ok(r) if r.status().is_success())
}
