use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let pid = std::process::id();
    let dir = std::env::temp_dir().join(format!("autonomy_orchestrator_cfg_{name}_{pid}_{nanos}"));
    fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

#[test]
fn saves_and_loads_ron_config() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir_1 = unique_temp_dir("save_ron_run_1");
    let out_dir_2 = unique_temp_dir("save_ron_run_2");
    let config_path = out_dir_1.join("orchestrator_config.ron");

    let output_1 = Command::new(bin)
        .arg(&out_dir_1)
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--config-save-ron")
        .arg(&config_path)
        .output()
        .expect("execute first run");
    assert!(output_1.status.success());
    assert!(config_path.exists(), "RON config should be written");

    let output_2 = Command::new(bin)
        .arg(&out_dir_2)
        .arg("--config-load-ron")
        .arg(&config_path)
        .output()
        .expect("execute second run");
    assert!(output_2.status.success());

    let _ = fs::remove_dir_all(out_dir_1);
    let _ = fs::remove_dir_all(out_dir_2);
}

#[test]
fn saves_and_loads_bin_config() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir_1 = unique_temp_dir("save_bin_run_1");
    let out_dir_2 = unique_temp_dir("save_bin_run_2");
    let config_path = out_dir_1.join("orchestrator_config.bin");

    let output_1 = Command::new(bin)
        .arg(&out_dir_1)
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--config-save-bin")
        .arg(&config_path)
        .output()
        .expect("execute first run");
    assert!(output_1.status.success());
    assert!(config_path.exists(), "BIN config should be written");

    let output_2 = Command::new(bin)
        .arg(&out_dir_2)
        .arg("--config-load-bin")
        .arg(&config_path)
        .output()
        .expect("execute second run");
    assert!(output_2.status.success());

    let _ = fs::remove_dir_all(out_dir_1);
    let _ = fs::remove_dir_all(out_dir_2);
}

#[test]
fn saves_and_loads_json_config() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir_1 = unique_temp_dir("save_json_run_1");
    let out_dir_2 = unique_temp_dir("save_json_run_2");
    let config_path = out_dir_1.join("orchestrator_config.json");

    let output_1 = Command::new(bin)
        .arg(&out_dir_1)
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--config-save-json")
        .arg(&config_path)
        .output()
        .expect("execute first run");
    assert!(output_1.status.success());
    assert!(config_path.exists(), "JSON config should be written");

    let output_2 = Command::new(bin)
        .arg(&out_dir_2)
        .arg("--config-load-json")
        .arg(&config_path)
        .output()
        .expect("execute second run");
    assert!(output_2.status.success());

    let _ = fs::remove_dir_all(out_dir_1);
    let _ = fs::remove_dir_all(out_dir_2);
}

#[test]
fn fails_when_multiple_config_load_modes_are_set() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("conflicting_load_modes");
    let config_ron = out_dir.join("orchestrator_config.ron");
    let config_bin = out_dir.join("orchestrator_config.bin");

    let setup = Command::new(bin)
        .arg(&out_dir)
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--config-save-ron")
        .arg(&config_ron)
        .arg("--config-save-bin")
        .arg(&config_bin)
        .output()
        .expect("execute setup run");
    assert!(setup.status.success());

    let output = Command::new(bin)
        .arg(&out_dir)
        .arg("--config-load-ron")
        .arg(&config_ron)
        .arg("--config-load-bin")
        .arg(&config_bin)
        .output()
        .expect("execute conflict run");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Only one config load mode is allowed"),
        "unexpected stderr: {stderr}"
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn saves_and_loads_config_with_auto_extension_mode() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir_1 = unique_temp_dir("auto_mode_save");
    let out_dir_2 = unique_temp_dir("auto_mode_load");
    let config_path = out_dir_1.join("orchestrator_config.json");

    let save_output = Command::new(bin)
        .arg(&out_dir_1)
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--config-save")
        .arg(&config_path)
        .output()
        .expect("execute save run");
    assert!(save_output.status.success());
    assert!(
        config_path.exists(),
        "config should be written by auto mode"
    );

    let load_output = Command::new(bin)
        .arg(&out_dir_2)
        .arg("--config-load")
        .arg(&config_path)
        .output()
        .expect("execute load run");
    assert!(load_output.status.success());

    let _ = fs::remove_dir_all(out_dir_1);
    let _ = fs::remove_dir_all(out_dir_2);
}

#[test]
fn fails_when_auto_save_is_mixed_with_explicit_mode() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("mixed_save_modes");
    let auto_path = out_dir.join("orchestrator_config.json");
    let explicit_path = out_dir.join("orchestrator_config.ron");

    let output = Command::new(bin)
        .arg(&out_dir)
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--config-save")
        .arg(&auto_path)
        .arg("--config-save-ron")
        .arg(&explicit_path)
        .output()
        .expect("execute mixed save mode run");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("When --config-save is used"),
        "unexpected stderr: {stderr}"
    );

    let _ = fs::remove_dir_all(out_dir);
}
