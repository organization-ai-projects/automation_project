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

#[test]
fn auto_mode_without_extension_uses_binary_format() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir_1 = unique_temp_dir("auto_no_ext_save");
    let out_dir_2 = unique_temp_dir("auto_no_ext_load");
    let config_path = out_dir_1.join("orchestrator_config");

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
        "extensionless config should be written"
    );

    let raw = fs::read(&config_path).expect("read extensionless config");
    assert!(
        raw.starts_with(b"AOCF"),
        "expected binary config header AOCF, got bytes: {:?}",
        &raw.get(0..4).unwrap_or_default()
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
fn config_validate_accepts_valid_binary_config() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("config_validate_bin_ok");
    let config_path = out_dir.join("orchestrator_config.bin");

    let save_output = Command::new(bin)
        .arg(&out_dir)
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--config-save-bin")
        .arg(&config_path)
        .output()
        .expect("execute save run");
    assert!(save_output.status.success());

    let validate_output = Command::new(bin)
        .arg("config-validate")
        .arg(&config_path)
        .arg("--ai-config-only-binary")
        .output()
        .expect("execute config-validate");
    assert!(validate_output.status.success());

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn config_validate_rejects_json_in_ai_binary_mode() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("config_validate_json_ai_reject");
    let config_path = out_dir.join("orchestrator_config.json");

    let save_output = Command::new(bin)
        .arg(&out_dir)
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--config-save-json")
        .arg(&config_path)
        .output()
        .expect("execute save run");
    assert!(save_output.status.success());

    let validate_output = Command::new(bin)
        .arg("config-validate")
        .arg(&config_path)
        .arg("--ai-config-only-binary")
        .output()
        .expect("execute config-validate");
    assert!(!validate_output.status.success());
    let stderr = String::from_utf8_lossy(&validate_output.stderr);
    assert!(
        stderr.contains("AI binary-only mode forbids non-binary config path"),
        "unexpected stderr: {stderr}"
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn config_validate_reports_actionable_diagnostics() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("config_validate_diagnostics");
    let config_path = out_dir.join("orchestrator_config.json");

    let save_output = Command::new(bin)
        .arg(&out_dir)
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--validation-from-planning-context")
        .arg("--config-save-json")
        .arg(&config_path)
        .output()
        .expect("execute save run");
    assert!(save_output.status.success());

    let validate_output = Command::new(bin)
        .arg("config-validate")
        .arg(&config_path)
        .output()
        .expect("execute config-validate");
    assert!(!validate_output.status.success());
    let stderr = String::from_utf8_lossy(&validate_output.stderr);
    assert!(
        stderr.contains("validation_from_planning_context=true requires planning_context_artifact"),
        "unexpected stderr: {stderr}"
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn config_canonicalize_converts_json_to_binary() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("config_canonicalize_json_to_bin");
    let json_path = out_dir.join("orchestrator_config.json");
    let bin_path = out_dir.join("orchestrator_config_latest.bin");

    let save_output = Command::new(bin)
        .arg(&out_dir)
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--config-save-json")
        .arg(&json_path)
        .output()
        .expect("execute save run");
    assert!(save_output.status.success());

    let canonicalize_output = Command::new(bin)
        .arg("config-canonicalize")
        .arg(&json_path)
        .arg(&bin_path)
        .output()
        .expect("execute config-canonicalize");
    assert!(canonicalize_output.status.success());
    assert!(
        bin_path.exists(),
        "canonical binary config should be written"
    );
    let raw = fs::read(&bin_path).expect("read canonical binary config");
    assert!(
        raw.starts_with(b"AOCF"),
        "expected binary config header AOCF, got bytes: {:?}",
        &raw.get(0..4).unwrap_or_default()
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn config_canonicalize_rejects_non_binary_paths_in_ai_mode() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("config_canonicalize_ai_binary_only");
    let json_path = out_dir.join("orchestrator_config.json");
    let bin_path = out_dir.join("orchestrator_config_latest.bin");

    let save_output = Command::new(bin)
        .arg(&out_dir)
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--config-save-json")
        .arg(&json_path)
        .output()
        .expect("execute save run");
    assert!(save_output.status.success());

    let canonicalize_output = Command::new(bin)
        .arg("config-canonicalize")
        .arg(&json_path)
        .arg(&bin_path)
        .arg("--ai-config-only-binary")
        .output()
        .expect("execute config-canonicalize");
    assert!(!canonicalize_output.status.success());
    let stderr = String::from_utf8_lossy(&canonicalize_output.stderr);
    assert!(
        stderr.contains("AI binary-only mode forbids non-binary config path"),
        "unexpected stderr: {stderr}"
    );

    let _ = fs::remove_dir_all(out_dir);
}
