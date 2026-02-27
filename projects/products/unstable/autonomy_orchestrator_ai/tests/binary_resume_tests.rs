use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use common_json::from_str;

#[derive(Debug, serde::Deserialize)]
struct RunReportView {
    terminal_state: Option<String>,
    stage_executions: Vec<StageExecutionView>,
}

#[derive(Debug, serde::Deserialize)]
struct StageExecutionView {
    status: String,
}

#[derive(Debug, serde::Serialize)]
struct CheckpointFixture {
    run_id: String,
    completed_stages: Vec<String>,
    terminal_state: Option<String>,
    updated_at_unix_secs: u64,
}

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let pid = std::process::id();
    let dir = std::env::temp_dir().join(format!("autonomy_orchestrator_{name}_{pid}_{nanos}"));
    fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

#[test]
fn resume_after_completed_checkpoint_skips_side_effect_stage() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("resume");

    let first = Command::new(bin)
        .arg(&out_dir)
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--decision-contribution")
        .arg(
            "contributor_id=resume_test,capability=validation,vote=proceed,confidence=100,weight=100",
        )
        .output()
        .expect("failed first run");
    assert!(
        first.status.success(),
        "first run failed: stdout={} stderr={}",
        String::from_utf8_lossy(&first.stdout),
        String::from_utf8_lossy(&first.stderr)
    );

    let second = Command::new(bin)
        .arg(&out_dir)
        .arg("--resume")
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--decision-contribution")
        .arg(
            "contributor_id=resume_test,capability=validation,vote=proceed,confidence=100,weight=100",
        )
        .arg("--manager-bin")
        .arg("__missing_binary__")
        .output()
        .expect("failed second run");

    assert!(
        second.status.success(),
        "second run failed: stdout={} stderr={}",
        String::from_utf8_lossy(&second.stdout),
        String::from_utf8_lossy(&second.stderr)
    );

    let report_path = out_dir.join("orchestrator_run_report.json");
    let report_raw = fs::read_to_string(&report_path).expect("failed to read run report");
    let report: RunReportView = from_str(&report_raw).expect("failed to deserialize run report");

    assert_eq!(report.terminal_state.as_deref(), Some("done"));
    assert!(
        report.stage_executions.is_empty(),
        "resume-on-done should not execute stages, got statuses: {:?}",
        report
            .stage_executions
            .iter()
            .map(|e| e.status.clone())
            .collect::<Vec<_>>()
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn resume_from_partial_checkpoint_skips_completed_stage_and_continues() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("partial_resume");
    let checkpoint_path = out_dir.join("orchestrator_checkpoint.json");

    let checkpoint = CheckpointFixture {
        run_id: "run_partial_resume".to_string(),
        completed_stages: vec!["planning".to_string()],
        terminal_state: None,
        updated_at_unix_secs: 1,
    };
    let checkpoint_json = common_json::to_string_pretty(&checkpoint).expect("serialize checkpoint");
    fs::write(&checkpoint_path, checkpoint_json).expect("write checkpoint fixture");

    let run = Command::new(bin)
        .arg(&out_dir)
        .arg("--resume")
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--decision-contribution")
        .arg(
            "contributor_id=resume_test,capability=validation,vote=proceed,confidence=100,weight=100",
        )
        .arg("--manager-bin")
        .arg("__missing_binary__")
        .output()
        .expect("failed resume run");

    assert!(
        run.status.success(),
        "resume run failed: stdout={} stderr={}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    let report_path = out_dir.join("orchestrator_run_report.json");
    let report_raw = fs::read_to_string(&report_path).expect("failed to read run report");
    let report: RunReportView = from_str(&report_raw).expect("failed to deserialize run report");

    assert_eq!(report.terminal_state.as_deref(), Some("done"));
    assert_eq!(report.stage_executions.len(), 1);
    assert_eq!(report.stage_executions[0].status, "skipped");

    let _ = fs::remove_dir_all(out_dir);
}
