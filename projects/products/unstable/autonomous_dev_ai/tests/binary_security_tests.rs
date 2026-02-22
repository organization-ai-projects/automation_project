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
    ],
    max_iterations: 12,
    timeout_seconds: Some(600),
)
"#;
    fs::write(config_ron_path, content).expect("failed to write test config");
}

fn run_with_env(goal: &str, extra_env: &[(&str, &str)]) -> (Output, PathBuf, PathBuf) {
    let bin = env!("CARGO_BIN_EXE_autonomous_dev_ai");
    let fixture = fixture_repo_dir();
    let out_dir = unique_temp_dir("security");

    let config_base = out_dir.join("agent_config");
    let config_ron = config_base.with_extension("ron");
    write_test_config(&config_ron);

    let audit_log = out_dir.join("agent_audit.log");
    let run_report = out_dir.join("agent_run_report.json");
    let run_replay = out_dir.join("agent_run_replay.json");
    let run_replay_txt = out_dir.join("agent_run_replay.txt");
    let checkpoint = out_dir.join("agent_checkpoint.json");
    let pr_description = out_dir.join("pr_description.md");

    let mut cmd = Command::new(bin);
    cmd.current_dir(&fixture)
        .arg(goal)
        .arg(&config_base)
        .arg(&audit_log)
        .env("AUTONOMOUS_RUN_REPORT_PATH", &run_report)
        .env("AUTONOMOUS_RUN_REPLAY_PATH", &run_replay)
        .env("AUTONOMOUS_RUN_REPLAY_TEXT_PATH", &run_replay_txt)
        .env("AUTONOMOUS_CHECKPOINT_PATH", &checkpoint)
        .env("AUTONOMOUS_REPO_ROOT", &fixture)
        .env("AUTONOMOUS_REQUIRE_EXPLORED_FILES", "true")
        .env(
            "AUTONOMOUS_ISSUE_TITLE",
            "feat(autonomous_dev_ai): security test",
        )
        .env(
            "AUTONOMOUS_ISSUE_BODY",
            "Context\nSecurity integration test\n\nHierarchy\nParent: none",
        )
        .env("AUTONOMOUS_REVIEW_REQUIRED", "true")
        .env(
            "AUTONOMOUS_REVIEW_COMMENTS_JSON",
            "[{\"reviewer\":\"security-bot\",\"body\":\"approved\",\"resolved\":true}]",
        )
        .env("AUTONOMOUS_PR_NUMBER", "654")
        .env("AUTONOMOUS_PR_DESCRIPTION_OUTPUT", &pr_description);

    for (k, v) in extra_env {
        cmd.env(k, v);
    }

    let output = cmd
        .output()
        .expect("failed to execute autonomous_dev_ai binary");
    (output, run_report, out_dir)
}

#[test]
fn read_only_actor_cannot_execute_developer_tools() {
    let (output, run_report, out_dir) = run_with_env(
        "Validate authz denial for read-only actor with tests",
        &[("AUTONOMOUS_ACTOR_ROLES", "read_only")],
    );

    assert!(
        !output.status.success(),
        "binary unexpectedly succeeded. stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let raw = fs::read_to_string(&run_report).expect("missing run report");
    let json: serde_json::Value = serde_json::from_str(&raw).expect("run report invalid");
    assert!(
        json["authz_denials_total"].as_u64().unwrap_or(0) > 0,
        "expected authz denials in report: {}",
        raw
    );
    assert_eq!(json["actor_roles"][0], "ReadOnly");

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn external_action_is_blocked_without_explicit_opt_in() {
    let (output, run_report, out_dir) = run_with_env(
        "Validate external action guard for PR creation",
        &[
            ("AUTONOMOUS_PR_NUMBER", ""),
            ("AUTONOMOUS_CREATE_PR", "true"),
            ("AUTONOMOUS_CREATE_PR_REQUIRED", "true"),
            ("AUTONOMOUS_ALLOW_EXTERNAL_ACTIONS", "false"),
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
        stderr.contains("external actions require AUTONOMOUS_ALLOW_EXTERNAL_ACTIONS=true"),
        "unexpected stderr:\n{}",
        stderr
    );

    let raw = fs::read_to_string(&run_report).expect("missing run report");
    let json: serde_json::Value = serde_json::from_str(&raw).expect("run report invalid");
    assert!(
        json["authz_denials_total"].as_u64().unwrap_or(0) > 0
            || json["policy_violations_total"].as_u64().unwrap_or(0) > 0,
        "expected security denial signal in report: {}",
        raw
    );

    let _ = fs::remove_dir_all(out_dir);
}
