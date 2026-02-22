// projects/products/unstable/autonomy_orchestrator_ai/src/binary_runner.rs

use crate::domain::{BinaryInvocationSpec, StageExecutionRecord, StageExecutionStatus};
use common_json::parse_str;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub fn invoke_binary(spec: &BinaryInvocationSpec) -> StageExecutionRecord {
    let started_at_unix_secs = unix_timestamp_secs();
    let started = Instant::now();
    let env_keys = spec
        .env
        .iter()
        .map(|(key, _)| key.clone())
        .collect::<Vec<_>>();

    let mut command = Command::new(&spec.command);
    command.args(&spec.args);
    for (key, value) in &spec.env {
        command.env(key, value);
    }

    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(err) => {
            return StageExecutionRecord {
                stage: spec.stage,
                idempotency_key: Some(format!("stage:{:?}", spec.stage)),
                command: spec.command.clone(),
                args: spec.args.clone(),
                env_keys,
                started_at_unix_secs,
                duration_ms: duration_to_u64_ms(started.elapsed()),
                exit_code: None,
                status: StageExecutionStatus::SpawnFailed,
                error: Some(format!("Failed to spawn command '{}': {err}", spec.command)),
                missing_artifacts: Vec::new(),
                malformed_artifacts: Vec::new(),
            };
        }
    };

    let timeout = Duration::from_millis(spec.timeout_ms);

    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let duration_ms = started.elapsed().as_millis();
                let exit_code = status.code();
                let missing_artifacts = collect_missing_artifacts(&spec.expected_artifacts);
                let malformed_artifacts =
                    collect_malformed_json_artifacts(&spec.expected_artifacts, &missing_artifacts);

                let (status, error) = if !missing_artifacts.is_empty() {
                    (
                        StageExecutionStatus::ArtifactMissing,
                        Some("Expected artifacts were not produced".to_string()),
                    )
                } else if !malformed_artifacts.is_empty() {
                    (
                        StageExecutionStatus::ArtifactMissing,
                        Some("Expected JSON artifacts are malformed".to_string()),
                    )
                } else if status.success() {
                    (StageExecutionStatus::Success, None)
                } else {
                    (
                        StageExecutionStatus::Failed,
                        Some(format!("Command exited with code {:?}", exit_code)),
                    )
                };

                return StageExecutionRecord {
                    stage: spec.stage,
                    idempotency_key: Some(format!("stage:{:?}", spec.stage)),
                    command: spec.command.clone(),
                    args: spec.args.clone(),
                    env_keys,
                    started_at_unix_secs,
                    duration_ms: u128_to_u64_ms(duration_ms),
                    exit_code,
                    status,
                    error,
                    missing_artifacts,
                    malformed_artifacts,
                };
            }
            Ok(None) => {
                if started.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    return StageExecutionRecord {
                        stage: spec.stage,
                        idempotency_key: Some(format!("stage:{:?}", spec.stage)),
                        command: spec.command.clone(),
                        args: spec.args.clone(),
                        env_keys,
                        started_at_unix_secs,
                        duration_ms: duration_to_u64_ms(started.elapsed()),
                        exit_code: None,
                        status: StageExecutionStatus::Timeout,
                        error: Some(format!("Command timed out after {} ms", spec.timeout_ms)),
                        missing_artifacts: Vec::new(),
                        malformed_artifacts: Vec::new(),
                    };
                }
                thread::sleep(Duration::from_millis(10));
            }
            Err(err) => {
                return StageExecutionRecord {
                    stage: spec.stage,
                    idempotency_key: Some(format!("stage:{:?}", spec.stage)),
                    command: spec.command.clone(),
                    args: spec.args.clone(),
                    env_keys,
                    started_at_unix_secs,
                    duration_ms: duration_to_u64_ms(started.elapsed()),
                    exit_code: None,
                    status: StageExecutionStatus::Failed,
                    error: Some(format!("Failed to wait for command: {err}")),
                    missing_artifacts: Vec::new(),
                    malformed_artifacts: Vec::new(),
                };
            }
        }
    }
}

fn collect_missing_artifacts(artifacts: &[String]) -> Vec<String> {
    artifacts
        .iter()
        .filter(|artifact| !Path::new(artifact.as_str()).exists())
        .cloned()
        .collect()
}

fn collect_malformed_json_artifacts(
    artifacts: &[String],
    missing_artifacts: &[String],
) -> Vec<String> {
    artifacts
        .iter()
        .filter(|artifact| artifact.ends_with(".json"))
        .filter(|artifact| !missing_artifacts.contains(artifact))
        .filter_map(|artifact| {
            let content = fs::read_to_string(artifact.as_str()).ok()?;
            parse_str(&content).err().map(|_| artifact.clone())
        })
        .collect()
}

fn unix_timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn duration_to_u64_ms(duration: Duration) -> u64 {
    u128_to_u64_ms(duration.as_millis())
}

fn u128_to_u64_ms(value: u128) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}

#[cfg(test)]
mod tests {
    use super::invoke_binary;
    use crate::domain::{BinaryInvocationSpec, Stage, StageExecutionStatus};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn missing_binary_reports_spawn_failed() {
        let spec = BinaryInvocationSpec {
            stage: Stage::Planning,
            command: "__definitely_missing_binary__".to_string(),
            args: Vec::new(),
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
            command: "/bin/sh".to_string(),
            args: vec!["-c".to_string(), "exit 0".to_string()],
            env: Vec::new(),
            timeout_ms: 250,
            expected_artifacts: vec![artifact_path.display().to_string()],
        };

        let result = invoke_binary(&spec);
        assert_eq!(result.status, StageExecutionStatus::ArtifactMissing);
        assert_eq!(result.malformed_artifacts.len(), 1);

        fs::remove_dir_all(&temp_root).ok();
    }
}
