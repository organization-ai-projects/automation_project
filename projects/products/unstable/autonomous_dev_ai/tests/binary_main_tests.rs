use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
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

#[test]
fn binary_non_interactive_ci_like_run_reaches_done() {
    let bin = env!("CARGO_BIN_EXE_autonomous_dev_ai");
    let fixture = fixture_repo_dir();
    let out_dir = unique_temp_dir("ci_like_success");

    let config_base = out_dir.join("agent_config");
    let config_ron = config_base.with_extension("ron");
    write_test_config(&config_ron);

    let audit_log = out_dir.join("agent_audit.log");
    let run_report = out_dir.join("agent_run_report.json");
    let run_replay = out_dir.join("agent_run_replay.json");
    let run_replay_txt = out_dir.join("agent_run_replay.txt");
    let checkpoint = out_dir.join("agent_checkpoint.json");
    let pr_description = out_dir.join("pr_description.md");

    let issue_body = "Context\nFixture test run\n\nHierarchy\nParent: none";
    let review_comments =
        "[{\"reviewer\":\"ci-bot\",\"body\":\"fixture approved\",\"resolved\":true}]";

    let output = Command::new(bin)
        .current_dir(&fixture)
        .arg("Validate CI-like autonomous flow for issue #649 with tests")
        .arg(&config_base)
        .arg(&audit_log)
        .env("AUTONOMOUS_RUN_REPLAY_PATH", &run_replay)
        .env("AUTONOMOUS_RUN_REPLAY_TEXT_PATH", &run_replay_txt)
        .env("AUTONOMOUS_RUN_REPORT_PATH", &run_report)
        .env("AUTONOMOUS_CHECKPOINT_PATH", &checkpoint)
        .env("AUTONOMOUS_REPO_ROOT", &fixture)
        .env("AUTONOMOUS_REQUIRE_EXPLORED_FILES", "true")
        .env(
            "AUTONOMOUS_ISSUE_TITLE",
            "feat(autonomous_dev_ai): ci fixture",
        )
        .env("AUTONOMOUS_ISSUE_BODY", issue_body)
        .env("AUTONOMOUS_REVIEW_REQUIRED", "true")
        .env("AUTONOMOUS_REVIEW_COMMENTS_JSON", review_comments)
        .env("AUTONOMOUS_PR_NUMBER", "649")
        .env("AUTONOMOUS_PR_DESCRIPTION_OUTPUT", &pr_description)
        .output()
        .expect("failed to execute autonomous_dev_ai binary");

    assert!(
        output.status.success(),
        "binary failed. stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let report_raw = fs::read_to_string(&run_report).expect("missing run report");
    let report_json: serde_json::Value =
        serde_json::from_str(&report_raw).expect("run report is not valid JSON");

    assert_eq!(report_json["final_state"], "Done");
    assert_eq!(report_json["last_review_outcome"], "Approved");
    assert_eq!(report_json["last_objective_passed"], true);

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn binary_fails_fast_on_inconsistent_strict_runtime_flags() {
    let bin = env!("CARGO_BIN_EXE_autonomous_dev_ai");
    let fixture = fixture_repo_dir();
    let out_dir = unique_temp_dir("strict_flags");

    let config_base = out_dir.join("agent_config");
    let config_ron = config_base.with_extension("ron");
    write_test_config(&config_ron);

    let audit_log = out_dir.join("agent_audit.log");
    let run_report = out_dir.join("agent_run_report.json");

    let output = Command::new(bin)
        .current_dir(&fixture)
        .arg("Validate runtime requirements for issue #649")
        .arg(&config_base)
        .arg(&audit_log)
        .env("AUTONOMOUS_RUN_REPORT_PATH", &run_report)
        .env("AUTONOMOUS_REQUIRE_REAL_PR_CREATION", "true")
        .output()
        .expect("failed to execute autonomous_dev_ai binary");

    assert!(
        !output.status.success(),
        "binary unexpectedly succeeded. stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "AUTONOMOUS_REQUIRE_REAL_PR_CREATION=true requires AUTONOMOUS_CREATE_PR=true"
        ),
        "unexpected stderr:\n{}",
        stderr
    );

    let _ = fs::remove_dir_all(out_dir);
}
