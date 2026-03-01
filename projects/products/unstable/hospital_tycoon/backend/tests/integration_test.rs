// projects/products/unstable/hospital_tycoon/backend/tests/integration_test.rs
//
// Integration tests for hospital_tycoon_backend.
// These tests exercise the binary via stdin/stdout IPC (JSON Lines protocol).

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

const BINARY: &str = env!("CARGO_BIN_EXE_hospital_tycoon_backend");
const TINY_CLINIC: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/scenarios/tiny_clinic.json"
);

/// Helper: spawn backend, exchange JSON-line messages, collect responses.
fn run_session(scenario: &str, messages: &[serde_json::Value]) -> Vec<serde_json::Value> {
    let mut child = Command::new(BINARY)
        .arg("serve")
        .arg("--scenario")
        .arg(scenario)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to spawn backend binary");

    let stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let reader = BufReader::new(stdout);
    let mut writer = stdin;

    let mut responses = Vec::new();
    let mut lines = reader.lines();

    for msg in messages {
        let line = serde_json::to_string(msg).unwrap() + "\n";
        writer.write_all(line.as_bytes()).unwrap();
        writer.flush().unwrap();
        if let Some(Ok(resp_line)) = lines.next() {
            let v: serde_json::Value =
                serde_json::from_str(&resp_line).unwrap_or(serde_json::Value::Null);
            responses.push(v);
        }
    }

    drop(writer);
    let _ = child.wait();
    responses
}

/// Test 1: Deterministic report — same seed + scenario => identical RunHash
#[test]
fn test_deterministic_report() {
    let messages = vec![
        serde_json::json!({"id": 1, "request": {"type": "NewRun", "seed": 42, "ticks": 50}}),
        serde_json::json!({"id": 2, "request": {"type": "RunToEnd"}}),
        serde_json::json!({"id": 3, "request": {"type": "GetReport"}}),
    ];

    let resps1 = run_session(TINY_CLINIC, &messages);
    let resps2 = run_session(TINY_CLINIC, &messages);

    let hash1 = &resps1[2]["run_hash"];
    let hash2 = &resps2[2]["run_hash"];

    assert!(!hash1.is_null(), "run_hash should not be null");
    assert_eq!(
        hash1, hash2,
        "run_hash must be identical for same seed+scenario"
    );
}

/// Test 2: Triage routing — patients are assigned deterministically
#[test]
fn test_triage_routing_determinism() {
    let messages = vec![
        serde_json::json!({"id": 1, "request": {"type": "NewRun", "seed": 77, "ticks": 20}}),
        serde_json::json!({"id": 2, "request": {"type": "Step", "n": 10}}),
        serde_json::json!({"id": 3, "request": {"type": "GetSnapshot", "at_tick": 10}}),
    ];

    let resps1 = run_session(TINY_CLINIC, &messages);
    let resps2 = run_session(TINY_CLINIC, &messages);

    // Snapshot hash must match between two identical runs
    let snap1 = &resps1[2]["snapshot"]["hash"];
    let snap2 = &resps2[2]["snapshot"]["hash"];
    assert!(!snap1.is_null(), "snapshot hash should not be null");
    assert_eq!(
        snap1, snap2,
        "snapshot hash must be identical for same inputs"
    );
}

/// Test 3: Canonical report encoding — same report produces same JSON
#[test]
fn test_canonical_report_encoding_determinism() {
    let messages = vec![
        serde_json::json!({"id": 1, "request": {"type": "NewRun", "seed": 42, "ticks": 50}}),
        serde_json::json!({"id": 2, "request": {"type": "RunToEnd"}}),
        serde_json::json!({"id": 3, "request": {"type": "GetReport"}}),
        serde_json::json!({"id": 4, "request": {"type": "GetReport"}}),
    ];

    let resps = run_session(TINY_CLINIC, &messages);
    let json1 = &resps[2]["report_json"];
    let json2 = &resps[3]["report_json"];
    assert_eq!(
        json1, json2,
        "repeated GetReport must return identical report_json"
    );
}

/// Test 4: Replay produces identical report as original run
#[test]
fn test_replay_produces_identical_report() {
    let replay_path = std::env::temp_dir().join("hospital_tycoon_test_replay.json");
    let replay_str = replay_path.to_str().unwrap().to_string();

    // Original run
    let messages_run = vec![
        serde_json::json!({"id": 1, "request": {"type": "NewRun", "seed": 42, "ticks": 30}}),
        serde_json::json!({"id": 2, "request": {"type": "RunToEnd"}}),
        serde_json::json!({"id": 3, "request": {"type": "GetReport"}}),
        serde_json::json!({"id": 4, "request": {"type": "SaveReplay", "path": replay_str}}),
    ];
    let resps_run = run_session(TINY_CLINIC, &messages_run);
    let original_hash = resps_run[2]["run_hash"].as_str().unwrap_or("").to_string();
    assert!(
        !original_hash.is_empty(),
        "original run_hash must not be empty"
    );
    assert_eq!(resps_run[3]["type"], "Ok", "SaveReplay should return Ok");

    // Replay run
    let messages_replay = vec![
        serde_json::json!({"id": 1, "request": {"type": "LoadReplay", "path": replay_str}}),
        serde_json::json!({"id": 2, "request": {"type": "ReplayToEnd"}}),
    ];
    let resps_replay = run_session(TINY_CLINIC, &messages_replay);
    let replay_hash = resps_replay[1]["run_hash"]
        .as_str()
        .unwrap_or("")
        .to_string();

    assert_eq!(
        original_hash, replay_hash,
        "replay run_hash must match original run_hash"
    );

    // Clean up
    let _ = std::fs::remove_file(&replay_path);
}

/// Test 5: Invalid scenario path returns exit code 3
#[test]
fn test_invalid_scenario_exit_code() {
    let status = Command::new(BINARY)
        .arg("serve")
        .arg("--scenario")
        .arg("/nonexistent/path/scenario.json")
        .status()
        .expect("failed to run backend");
    assert_eq!(
        status.code(),
        Some(3),
        "invalid scenario path should exit with code 3"
    );
}

/// Test 6: Invalid CLI returns exit code 2
#[test]
fn test_invalid_cli_exit_code() {
    let status = Command::new(BINARY)
        .arg("unknown_command")
        .status()
        .expect("failed to run backend");
    assert_eq!(
        status.code(),
        Some(2),
        "unknown command should exit with code 2"
    );
}
