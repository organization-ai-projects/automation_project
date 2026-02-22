use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let pid = std::process::id();
    let dir = std::env::temp_dir().join(format!("autonomous_dev_ai_{name}_{pid}_{nanos}"));
    fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

fn fixture_repo_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join("ci_like_repo")
}

fn write_test_config(config_ron_path: &Path) {
    let content = r#"(
    agent_name: "autonomous_dev_ai",
    execution_mode: "ci_bound",
    neural: (
        enabled: true,
        prefer_gpu: false,
        cpu_fallback: true,
        models: {
            "intent": "intent_v1.bin",
            "codegen": "codegen_v2.bin",
            "review": "review_v1.bin",
        },
    ),
    symbolic: (
        strict_validation: true,
        deterministic: true,
    ),
    objectives: [
        (
            name: "task_completion",
            weight: 1.0,
            hard: true,
            threshold: Some(0.5),
        ),
        (
            name: "policy_safety",
            weight: 1.0,
            hard: true,
            threshold: Some(0.0),
        ),
        (
            name: "tests_pass",
            weight: 0.9,
            hard: true,
            threshold: Some(0.7),
        ),
        (
            name: "minimal_diff",
            weight: 0.6,
            hard: false,
            threshold: None,
        ),
        (
            name: "time_budget",
            weight: 0.4,
            hard: false,
            threshold: None,
        ),
    ],
    max_iterations: 8,
    timeout_seconds: Some(600),
)
"#;
    fs::write(config_ron_path, content).expect("failed to write test config");
}

fn run_binary(goal: &str, extra_env: &[(&str, &str)]) -> (Output, PathBuf) {
    let bin = env!("CARGO_BIN_EXE_autonomous_dev_ai");
    let fixture = fixture_repo_dir();
    let out_dir = unique_temp_dir("strict_runtime");

    let config_base = out_dir.join("agent_config");
    let config_ron = config_base.with_extension("ron");
    write_test_config(&config_ron);

    let audit_log = out_dir.join("agent_audit.log");
    let run_report = out_dir.join("agent_run_report.json");
    let run_replay = out_dir.join("agent_run_replay.json");
    let run_replay_txt = out_dir.join("agent_run_replay.txt");
    let checkpoint = out_dir.join("agent_checkpoint.json");

    let mut cmd = Command::new(bin);
    cmd.current_dir(&fixture)
        .arg(goal)
        .arg(&config_base)
        .arg(&audit_log)
        .env("AUTONOMOUS_RUN_REPORT_PATH", &run_report)
        .env("AUTONOMOUS_RUN_REPLAY_PATH", &run_replay)
        .env("AUTONOMOUS_RUN_REPLAY_TEXT_PATH", &run_replay_txt)
        .env("AUTONOMOUS_CHECKPOINT_PATH", &checkpoint);

    for (k, v) in extra_env {
        cmd.env(k, v);
    }

    let output = cmd
        .output()
        .expect("failed to execute autonomous_dev_ai binary");

    (output, out_dir)
}

#[test]
fn binary_fails_fast_when_gh_review_source_required_without_fetch_flag() {
    let (output, out_dir) = run_binary(
        "Validate strict review flags for issue #649",
        &[("AUTONOMOUS_REQUIRE_GH_REVIEW_SOURCE", "true")],
    );

    assert!(
        !output.status.success(),
        "binary unexpectedly succeeded. stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "AUTONOMOUS_REQUIRE_GH_REVIEW_SOURCE=true requires AUTONOMOUS_FETCH_REVIEW_FROM_GH=true"
        ),
        "unexpected stderr:\n{}",
        stderr
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn binary_fails_fast_when_pr_ci_status_required_without_fetch_flag() {
    let (output, out_dir) = run_binary(
        "Validate strict PR CI flags for issue #649",
        &[("AUTONOMOUS_FETCH_PR_CI_STATUS_REQUIRED", "true")],
    );

    assert!(
        !output.status.success(),
        "binary unexpectedly succeeded. stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("AUTONOMOUS_FETCH_PR_CI_STATUS_REQUIRED=true requires AUTONOMOUS_FETCH_PR_CI_STATUS_FROM_GH=true"),
        "unexpected stderr:\n{}",
        stderr
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn binary_fails_fast_when_issue_context_required_without_fetch_flag() {
    let (output, out_dir) = run_binary(
        "Validate strict issue context flags for issue #649",
        &[("AUTONOMOUS_FETCH_ISSUE_CONTEXT_REQUIRED", "true")],
    );

    assert!(
        !output.status.success(),
        "binary unexpectedly succeeded. stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("AUTONOMOUS_FETCH_ISSUE_CONTEXT_REQUIRED=true requires AUTONOMOUS_FETCH_ISSUE_CONTEXT_FROM_GH=true"),
        "unexpected stderr:\n{}",
        stderr
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn binary_blocks_pr_stage_when_issue_compliance_is_required_and_invalid() {
    let issue_body_without_parent = "Context\nNo required parent field present";
    let (output, out_dir) = run_binary(
        "Validate issue compliance gate for issue #649",
        &[
            ("AUTONOMOUS_REQUIRE_ISSUE_COMPLIANCE", "true"),
            ("AUTONOMOUS_ISSUE_TITLE", "invalid title format"),
            ("AUTONOMOUS_ISSUE_BODY", issue_body_without_parent),
            ("AUTONOMOUS_REQUIRED_ISSUE_FIELDS", "Parent"),
        ],
    );

    assert!(
        !output.status.success(),
        "binary unexpectedly succeeded. stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("AUTONOMOUS_REQUIRE_ISSUE_COMPLIANCE=true"),
        "unexpected stderr:\n{}",
        stderr
    );

    let _ = fs::remove_dir_all(out_dir);
}
