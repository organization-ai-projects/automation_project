use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, serde::Deserialize)]
struct CliOutcome {
    task_id: String,
    success: bool,
    output: std::collections::BTreeMap<String, String>,
    logs: Vec<String>,
}

fn backend_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_agent_engine_backend"))
}

fn fixture_task() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("task_minimal.json")
}

#[test]
fn run_cli_produces_structured_outcome() {
    let output = Command::new(backend_bin())
        .args(["run", fixture_task().to_str().expect("fixture path")])
        .output()
        .expect("run backend cli");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let parsed: CliOutcome = common_json::from_slice(&output.stdout).expect("parse outcome json");

    assert_eq!(parsed.task_id, "task-v0-001");
    assert!(parsed.success);
    assert_eq!(parsed.output.get("status"), Some(&"ok".to_string()));
    assert_eq!(parsed.output.get("echo"), Some(&"hello agent".to_string()));
    assert_eq!(
        parsed.logs.first().map(String::as_str),
        Some("starting task")
    );
}

#[test]
fn run_cli_is_deterministic_for_same_input() {
    let run_once = || {
        Command::new(backend_bin())
            .args(["run", fixture_task().to_str().expect("fixture path")])
            .output()
            .expect("run backend cli")
    };

    let out1 = run_once();
    let out2 = run_once();

    assert!(out1.status.success());
    assert!(out2.status.success());

    let parsed1: common_json::Json = common_json::from_slice(&out1.stdout).expect("parse output 1");
    let parsed2: common_json::Json = common_json::from_slice(&out2.stdout).expect("parse output 2");
    assert_eq!(parsed1, parsed2);
}
