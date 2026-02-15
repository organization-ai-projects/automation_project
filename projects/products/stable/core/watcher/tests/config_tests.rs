// projects/products/stable/core/watcher/tests/config_tests.rs
use std::fs;
use std::hash::{Hash, Hasher};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

static TEMP_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

struct TempDir {
    path: std::path::PathBuf,
}

impl TempDir {
    fn new() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_nanos();
        let pid = std::process::id();
        let counter = TEMP_DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
        let thread_hash = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            std::thread::current().id().hash(&mut hasher);
            hasher.finish()
        };

        for collision_retry in 0..8 {
            let path = std::env::temp_dir().join(format!(
                "watcher_test_{nanos}_{pid}_{thread_hash}_{counter}_{collision_retry}"
            ));
            match fs::create_dir(&path) {
                Ok(()) => return Self { path },
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => panic!("create temp dir failed: {e}"),
            }
        }

        panic!("failed to create unique temp dir after retries");
    }

    fn path(&self) -> &std::path::Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn write_config(contents: &str) -> (TempDir, std::path::PathBuf) {
    let dir = TempDir::new();
    let path = dir.path().join("watcher.toml");
    fs::write(&path, contents).expect("write config");
    (dir, path)
}

fn spawn_watcher(config_path: &std::path::Path) -> std::process::Child {
    Command::new(env!("CARGO_BIN_EXE_watcher"))
        .env("WATCHER_CONFIG", config_path)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn watcher")
}

#[test]
fn test_valid_config_starts() {
    let config = r#"
[[components]]
name = "engine"
ping_interval = 10
ping = "disabled"

[components.restart]
policy = "never"
backoff_min_secs = 1
backoff_max_secs = 60

[logging]
log_file = "watcher.log"
log_level = "info"
"#;

    let (_dir, config_path) = write_config(config);
    let mut child = spawn_watcher(&config_path);

    // Wait up to 300ms and fail early if the process exits.
    let max_wait = Duration::from_millis(300);
    let started_at = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                panic!("watcher exited early: {status}");
            }
            Ok(None) => {
                // still running, continue retry loop
            }
            Err(e) => {
                panic!("failed to wait on watcher process: {e}");
            }
        }
        if started_at.elapsed() >= max_wait {
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }

    let _ = child.kill();
    let _ = child.wait();
}

#[test]
fn test_invalid_ping_interval_fails() {
    let config = r#"
[[components]]
name = "engine"
ping_interval = 0
ping = "disabled"

[components.restart]
policy = "never"
backoff_min_secs = 1
backoff_max_secs = 60
"#;

    let (_dir, config_path) = write_config(config);
    let output = Command::new(env!("CARGO_BIN_EXE_watcher"))
        .env("WATCHER_CONFIG", config_path)
        .output()
        .expect("run watcher");

    assert!(
        !output.status.success(),
        "expected watcher to fail with invalid ping_interval"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.is_empty(),
        "expected error message in stderr, got empty stderr"
    );
    assert!(
        stderr.contains("Ping interval") || stderr.contains("Failed to load configuration"),
        "expected stderr to mention ping interval or configuration failure, got: {stderr}"
    );
}

#[test]
fn test_invalid_log_level_fails() {
    let config = r#"
[[components]]
name = "engine"
ping_interval = 10
ping = "disabled"

[logging]
log_file = "watcher.log"
log_level = "not_a_level"
"#;

    let (_dir, config_path) = write_config(config);
    let output = Command::new(env!("CARGO_BIN_EXE_watcher"))
        .env("WATCHER_CONFIG", config_path)
        .output()
        .expect("run watcher");

    assert!(
        !output.status.success(),
        "expected watcher to fail with invalid log_level"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.is_empty(),
        "expected error message in stderr, got empty stderr"
    );
    assert!(
        stderr.contains("log_level") || stderr.contains("Failed to load configuration"),
        "expected stderr to mention log_level or configuration failure, got: {stderr}"
    );
}
