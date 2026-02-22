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
    max_iterations: 20,
    timeout_seconds: Some(600),
)
"#;
    fs::write(config_ron_path, content).expect("failed to write test config");
}

fn run_with_env(goal: &str, extra_env: &[(&str, &str)]) -> (Output, PathBuf, PathBuf) {
    let bin = env!("CARGO_BIN_EXE_autonomous_dev_ai");
    let fixture = fixture_repo_dir();
    let out_dir = unique_temp_dir("pr_flow");

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
            "feat(autonomous_dev_ai): pr flow test",
        )
        .env(
            "AUTONOMOUS_ISSUE_BODY",
            "Context\nPR flow integration test\n\nHierarchy\nParent: none",
        )
        .env("AUTONOMOUS_PR_NUMBER", "653")
        .env("AUTONOMOUS_PR_DESCRIPTION_OUTPUT", &pr_description)
        .env("AUTONOMOUS_REVIEW_REQUIRED", "true");

    for (k, v) in extra_env {
        cmd.env(k, v);
    }

    let output = cmd
        .output()
        .expect("failed to execute autonomous_dev_ai binary");
    (output, run_report, out_dir)
}

#[test]
fn review_loop_times_out_after_multiple_iterations() {
    let unresolved = r#"[{"reviewer":"reviewer-a","body":"please fix","resolved":false}]"#;
    let (_output, run_report, out_dir) = run_with_env(
        "Validate PR review loop timeout behavior for issue #653 with tests",
        &[
            ("AUTONOMOUS_REVIEW_COMMENTS_JSON", unresolved),
            ("AUTONOMOUS_AUTO_RESOLVE_REVIEW", "false"),
        ],
    );

    let raw = fs::read_to_string(&run_report).expect("missing run report");
    let json: serde_json::Value = serde_json::from_str(&raw).expect("run report invalid");
    let final_state = json["final_state"].as_str().unwrap_or_default();
    assert!(
        matches!(final_state, "Blocked" | "Failed"),
        "expected Blocked or Failed final state for timeout path, got '{}' in report: {}",
        final_state,
        raw
    );
    assert_eq!(json["last_review_outcome"], "Timeout");
    assert!(
        json["total_iterations"].as_u64().unwrap_or(0) >= 3,
        "expected >=3 iterations to hit review timeout, report: {}",
        raw
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn merge_readiness_resolves_successfully_on_nominal_review_path() {
    let approved = r#"[{"reviewer":"reviewer-a","body":"looks good","resolved":true}]"#;
    let (output, run_report, out_dir) = run_with_env(
        "Validate PR merge readiness happy path for issue #653 with tests",
        &[("AUTONOMOUS_REVIEW_COMMENTS_JSON", approved)],
    );

    assert!(
        output.status.success(),
        "binary failed. stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let raw = fs::read_to_string(&run_report).expect("missing run report");
    let json: serde_json::Value = serde_json::from_str(&raw).expect("run report invalid");
    assert_eq!(json["final_state"], "Done");
    assert_eq!(json["last_review_outcome"], "Approved");
    assert_eq!(json["pr_readiness"], "ready");
    assert_eq!(json["pr_number_source"], "env_injected");
    assert_eq!(json["issue_compliance"], "Compliant");

    let _ = fs::remove_dir_all(out_dir);
}
