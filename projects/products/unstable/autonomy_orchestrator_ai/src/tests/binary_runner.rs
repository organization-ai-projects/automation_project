use crate::binary_runner::invoke_binary;
use crate::domain::{BinaryInvocationSpec, CommandLineSpec, Stage, StageExecutionStatus};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn missing_binary_reports_spawn_failed() {
    let spec = BinaryInvocationSpec {
        stage: Stage::Planning,
        command_line: CommandLineSpec {
            command: "__definitely_missing_binary__".to_string(),
            args: Vec::new(),
        },
        env: Vec::new(),
        timeout_ms: 250,
        expected_artifacts: Vec::new(),
    };

    let result = invoke_binary(&spec);
    assert_eq!(result.status, StageExecutionStatus::SpawnFailed);
    assert_eq!(result.idempotency_key, Some("stage:Planning".to_string()));
}

#[test]
fn malformed_json_artifact_fails_closed() {
    let temp_root = std::env::temp_dir().join(format!(
        "autonomy_orchestrator_bad_json_{}_{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    fs::create_dir_all(&temp_root).expect("create temp directory");
    let artifact_path = temp_root.join("report.json");
    fs::write(&artifact_path, "{bad-json").expect("write malformed artifact");

    let spec = BinaryInvocationSpec {
        stage: Stage::Planning,
        command_line: CommandLineSpec {
            command: "true".to_string(),
            args: Vec::new(),
        },
        env: Vec::new(),
        timeout_ms: 250,
        expected_artifacts: vec![artifact_path.display().to_string()],
    };

    let result = invoke_binary(&spec);
    assert_eq!(result.status, StageExecutionStatus::ArtifactMissing);
    assert_eq!(result.malformed_artifacts.len(), 1);

    fs::remove_dir_all(&temp_root).ok();
}
