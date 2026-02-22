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

fn run_with_env(goal: &str, extra_env: &[(&str, &str)]) -> (Output, PathBuf, PathBuf, PathBuf) {
    let bin = env!("CARGO_BIN_EXE_autonomous_dev_ai");
    let fixture = fixture_repo_dir();
    let out_dir = unique_temp_dir("run_report");

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
            "feat(autonomous_dev_ai): run-report verification",
        )
        .env(
            "AUTONOMOUS_ISSUE_BODY",
            "Context\nRun report integration test\n\nHierarchy\nParent: none",
        )
        .env("AUTONOMOUS_REVIEW_REQUIRED", "true")
        .env(
            "AUTONOMOUS_REVIEW_COMMENTS_JSON",
            "[{\"reviewer\":\"ci-bot\",\"body\":\"approved\",\"resolved\":true}]",
        )
        .env("AUTONOMOUS_PR_NUMBER", "649")
        .env("AUTONOMOUS_PR_DESCRIPTION_OUTPUT", &pr_description);

    for (k, v) in extra_env {
        cmd.env(k, v);
    }

    let output = cmd
        .output()
        .expect("failed to execute autonomous_dev_ai binary");

    (output, run_report, run_replay, out_dir)
}

#[test]
fn run_report_contains_extended_fields_on_successful_run() {
    let (output, run_report, _run_replay, out_dir) =
        run_with_env("Validate run report fields for issue #649 with tests", &[]);

    assert!(
        output.status.success(),
        "binary failed. stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let raw = fs::read_to_string(&run_report).expect("missing run report");
    let json: serde_json::Value = serde_json::from_str(&raw).expect("run report invalid JSON");

    assert_eq!(json["final_state"], "Done");
    assert_eq!(json["last_review_outcome"], "Approved");
    assert_eq!(json["pr_number_source"], "env_injected");
    assert_eq!(json["issue_context_source"], "env_or_goal");
    assert!(json.get("failure_kind_counts").is_some());
    assert!(json.get("top_failure_kind").is_some());
    assert!(json.get("last_failure_recovery_action").is_some());

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn run_report_captures_failure_telemetry_on_failing_run() {
    let (output, run_report, _run_replay, out_dir) = run_with_env(
        "Validate strict issue compliance failure for issue #649",
        &[
            ("AUTONOMOUS_REQUIRE_ISSUE_COMPLIANCE", "true"),
            (
                "AUTONOMOUS_ISSUE_BODY",
                "Context only without required fields",
            ),
            ("AUTONOMOUS_REQUIRED_ISSUE_FIELDS", "Parent"),
        ],
    );

    assert!(
        !output.status.success(),
        "binary unexpectedly succeeded. stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let raw = fs::read_to_string(&run_report).expect("missing run report for failing run");
    let json: serde_json::Value = serde_json::from_str(&raw).expect("run report invalid JSON");

    assert_ne!(json["final_state"], "Done");
    assert!(
        json["total_failures"].as_u64().unwrap_or(0) > 0,
        "expected at least one failure in run report"
    );
    assert!(json.get("failure_kind_counts").is_some());
    assert!(json.get("top_failure_kind").is_some());
    assert!(
        !json["last_failure_description"]
            .as_str()
            .unwrap_or("")
            .is_empty(),
        "missing last_failure_description: {}",
        raw
    );
    assert!(
        !json["last_failure_error"].as_str().unwrap_or("").is_empty(),
        "missing last_failure_error: {}",
        raw
    );
    assert!(
        !json["last_failure_recovery_action"]
            .as_str()
            .unwrap_or("")
            .is_empty(),
        "missing last_failure_recovery_action: {}",
        raw
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn run_replay_records_neural_eval_file_source_for_active_model() {
    let eval_file = unique_temp_dir("neural_eval").join("neural_eval.json");
    let eval_raw = r#"{
  "models": [
    { "model_name": "default-neural", "offline_score": 0.10, "online_score": 0.10 },
    { "model_name": "alt-neural", "offline_score": 0.95, "online_score": 0.96 }
  ]
}"#;
    fs::write(&eval_file, eval_raw).expect("failed to write eval snapshot");
    let eval_file_value: &'static str =
        Box::leak(eval_file.to_string_lossy().to_string().into_boxed_str());

    let (output, run_report, run_replay, out_dir) = run_with_env(
        "Validate neural eval file source for active model",
        &[
            ("AUTONOMOUS_NEURAL_MODEL_NAME", "alt-neural"),
            ("AUTONOMOUS_NEURAL_EVAL_FILE", eval_file_value),
        ],
    );

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

    let replay_raw = fs::read_to_string(&run_replay).expect("missing run replay");
    let replay_json: serde_json::Value =
        serde_json::from_str(&replay_raw).expect("run replay is not valid JSON");
    let events = replay_json["events"]
        .as_array()
        .expect("events should be an array");

    let has_eval_file_source = events.iter().any(|event| {
        event["kind"] == "neural.rollout_eval_source"
            && event["payload"]
                .as_str()
                .map(|p| p.starts_with("file:"))
                .unwrap_or(false)
    });
    assert!(
        has_eval_file_source,
        "expected neural.rollout_eval_source with file: payload in replay: {}",
        replay_raw
    );

    let has_active_model_gate_trace = events.iter().any(|event| {
        event["kind"] == "neural.rollout_gates"
            && event["payload"]
                .as_str()
                .map(|p| p.contains("model=alt-neural"))
                .unwrap_or(false)
    });
    assert!(
        has_active_model_gate_trace,
        "expected neural.rollout_gates trace for alt-neural in replay: {}",
        replay_raw
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn run_report_exposes_queryable_tool_metrics_and_dashboard_artifacts() {
    let dashboard_dir = unique_temp_dir("ops_dashboard");
    let dashboard_json = dashboard_dir.join("ops_dashboard.json");
    let dashboard_md = dashboard_dir.join("ops_dashboard.md");
    let dashboard_json_value: &'static str = Box::leak(
        dashboard_json
            .to_string_lossy()
            .to_string()
            .into_boxed_str(),
    );
    let dashboard_md_value: &'static str =
        Box::leak(dashboard_md.to_string_lossy().to_string().into_boxed_str());

    let (output, run_report, _run_replay, out_dir) = run_with_env(
        "Validate ops dashboard and tool metrics exposure",
        &[
            ("AUTONOMOUS_OPS_DASHBOARD_JSON_PATH", dashboard_json_value),
            ("AUTONOMOUS_OPS_DASHBOARD_MD_PATH", dashboard_md_value),
        ],
    );

    assert!(
        output.status.success(),
        "binary failed. stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let report_raw = fs::read_to_string(&run_report).expect("missing run report");
    let report_json: serde_json::Value =
        serde_json::from_str(&report_raw).expect("run report is not valid JSON");

    let tool_metrics = report_json["tool_metrics"]
        .as_object()
        .expect("tool_metrics should be an object");
    assert!(
        !tool_metrics.is_empty(),
        "expected non-empty tool metrics in run report: {}",
        report_raw
    );
    assert!(report_json["alerts"].is_array());
    assert!(report_json["dashboard_json_path"].is_string());
    assert!(report_json["dashboard_markdown_path"].is_string());

    assert!(
        dashboard_json.exists(),
        "ops dashboard JSON not generated at {:?}",
        dashboard_json
    );
    assert!(
        dashboard_md.exists(),
        "ops dashboard Markdown not generated at {:?}",
        dashboard_md
    );

    let dashboard_md_raw =
        fs::read_to_string(&dashboard_md).expect("failed to read generated dashboard markdown");
    assert!(
        dashboard_md_raw.contains("Autonomous Ops Dashboard")
            && dashboard_md_raw.contains("Alerts")
            && dashboard_md_raw.contains("Tool Metrics"),
        "unexpected dashboard markdown content:\n{}",
        dashboard_md_raw
    );

    let _ = fs::remove_dir_all(out_dir);
    let _ = fs::remove_dir_all(dashboard_dir);
}
