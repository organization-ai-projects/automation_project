use std::fs;
use std::time::Duration;

/// Snapshot of current process usage for runtime budget checks.
#[derive(Debug, Clone, Copy)]
pub struct ProcessUsage {
    pub cpu_time: Duration,
    pub rss_bytes: usize,
}

impl ProcessUsage {
    pub fn sample() -> Option<Self> {
        Some(Self {
            cpu_time: read_cpu_time_from_proc_stat()?,
            rss_bytes: read_rss_bytes_from_proc_status()?,
        })
    }
}

fn read_cpu_time_from_proc_stat() -> Option<Duration> {
    let stat = fs::read_to_string("/proc/self/stat").ok()?;
    let fields: Vec<&str> = stat.split_whitespace().collect();
    if fields.len() < 17 {
        return None;
    }

    let user_ticks = fields.get(13)?.parse::<u64>().ok()?;
    let system_ticks = fields.get(14)?.parse::<u64>().ok()?;
    let total_ticks = user_ticks.saturating_add(system_ticks);

    let ticks_per_second = std::env::var("AUTONOMOUS_CPU_TICKS_PER_SECOND")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(100)
        .max(1);

    Some(Duration::from_secs_f64(
        (total_ticks as f64) / (ticks_per_second as f64),
    ))
}

fn read_rss_bytes_from_proc_status() -> Option<usize> {
    let status = fs::read_to_string("/proc/self/status").ok()?;
    let vm_rss_line = status.lines().find(|line| line.starts_with("VmRSS:"))?;
    let kb = vm_rss_line
        .split_whitespace()
        .nth(1)?
        .parse::<usize>()
        .ok()?;
    kb.checked_mul(1024)
}
