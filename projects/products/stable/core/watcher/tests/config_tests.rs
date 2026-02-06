// projects/products/stable/core/watcher/tests/config_tests.rs
use std::fs;
use std::process::{Command, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn unique_temp_dir() -> std::path::PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("watcher_test_{nanos}"));
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn write_config(contents: &str) -> std::path::PathBuf {
    let dir = unique_temp_dir();
    let path = dir.join("watcher.toml");
    fs::write(&path, contents).expect("write config");
    path
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

    let config_path = write_config(config);
    let mut child = spawn_watcher(&config_path);

    std::thread::sleep(Duration::from_millis(300));
    if let Ok(Some(status)) = child.try_wait() {
        panic!("watcher exited early: {status}");
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

    let config_path = write_config(config);
    let output = Command::new(env!("CARGO_BIN_EXE_watcher"))
        .env("WATCHER_CONFIG", config_path)
        .output()
        .expect("run watcher");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Ping interval") || stderr.contains("Failed to load configuration"),
        "unexpected stderr: {stderr}"
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

    let config_path = write_config(config);
    let output = Command::new(env!("CARGO_BIN_EXE_watcher"))
        .env("WATCHER_CONFIG", config_path)
        .output()
        .expect("run watcher");

    assert!(!output.status.success());
}
