// projects/products/unstable/hospital_tycoon/backend/tests/integration_test.rs
//
// Integration tests for hospital_tycoon_backend.
// These tests exercise the binary via stdin/stdout IPC (JSON Lines protocol).

use common_json::JsonAccess;
use serde::Serialize;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

const BINARY: &str = env!("CARGO_BIN_EXE_hospital_tycoon_backend");
const TINY_CLINIC: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/scenarios/tiny_clinic.json"
);

#[derive(Debug, Clone, Serialize)]
struct OutboundMessage {
    id: u64,
    request: OutboundRequest,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
enum OutboundRequest {
    NewRun { seed: u64, ticks: u64 },
    Step { n: u64 },
    RunToEnd,
    GetSnapshot { at_tick: u64 },
    GetReport,
    SaveReplay { path: String },
    LoadReplay { path: String },
    ReplayToEnd,
}

/// Helper: spawn backend, exchange JSON-line messages, collect responses.
fn run_session(scenario: &str, messages: &[OutboundMessage]) -> Vec<common_json::Json> {
    let mut child = Command::new(BINARY)
        .arg("serve")
        .arg("--scenario")
        .arg(scenario)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to spawn backend binary");

    let stdin = child.stdin.take().expect("stdin unavailable");
    let stdout = child.stdout.take().expect("stdout unavailable");
    let reader = BufReader::new(stdout);
    let mut writer = stdin;

    let mut responses = Vec::new();
    let mut lines = reader.lines();

    for msg in messages {
        let line = common_json::to_string(msg).expect("message encoding failed") + "\n";
        writer
            .write_all(line.as_bytes())
            .expect("stdin write failed");
        writer.flush().expect("stdin flush failed");
        if let Some(Ok(resp_line)) = lines.next() {
            let v: common_json::Json =
                common_json::from_str(&resp_line).unwrap_or(common_json::Json::Null);
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
        OutboundMessage {
            id: 1,
            request: OutboundRequest::NewRun {
                seed: 42,
                ticks: 50,
            },
        },
        OutboundMessage {
            id: 2,
            request: OutboundRequest::RunToEnd,
        },
        OutboundMessage {
            id: 3,
            request: OutboundRequest::GetReport,
        },
    ];

    let resps1 = run_session(TINY_CLINIC, &messages);
    let resps2 = run_session(TINY_CLINIC, &messages);

    let hash1 = resps1[2]
        .get_field("run_hash")
        .ok()
        .and_then(common_json::Json::as_str)
        .unwrap_or_default()
        .to_string();
    let hash2 = resps2[2]
        .get_field("run_hash")
        .ok()
        .and_then(common_json::Json::as_str)
        .unwrap_or_default()
        .to_string();

    assert!(!hash1.is_empty(), "run_hash should not be empty");
    assert_eq!(
        hash1, hash2,
        "run_hash must be identical for same seed+scenario"
    );
}

/// Test 2: Triage routing — patients are assigned deterministically
#[test]
fn test_triage_routing_determinism() {
    let messages = vec![
        OutboundMessage {
            id: 1,
            request: OutboundRequest::NewRun {
                seed: 77,
                ticks: 20,
            },
        },
        OutboundMessage {
            id: 2,
            request: OutboundRequest::Step { n: 10 },
        },
        OutboundMessage {
            id: 3,
            request: OutboundRequest::GetSnapshot { at_tick: 10 },
        },
    ];

    let resps1 = run_session(TINY_CLINIC, &messages);
    let resps2 = run_session(TINY_CLINIC, &messages);

    let snap1 = resps1[2]
        .get_path("snapshot.hash")
        .ok()
        .and_then(common_json::Json::as_str)
        .unwrap_or_default()
        .to_string();
    let snap2 = resps2[2]
        .get_path("snapshot.hash")
        .ok()
        .and_then(common_json::Json::as_str)
        .unwrap_or_default()
        .to_string();

    assert!(!snap1.is_empty(), "snapshot hash should not be empty");
    assert_eq!(
        snap1, snap2,
        "snapshot hash must be identical for same inputs"
    );
}

/// Test 3: Canonical report encoding — same report produces same JSON
#[test]
fn test_canonical_report_encoding_determinism() {
    let messages = vec![
        OutboundMessage {
            id: 1,
            request: OutboundRequest::NewRun {
                seed: 42,
                ticks: 50,
            },
        },
        OutboundMessage {
            id: 2,
            request: OutboundRequest::RunToEnd,
        },
        OutboundMessage {
            id: 3,
            request: OutboundRequest::GetReport,
        },
        OutboundMessage {
            id: 4,
            request: OutboundRequest::GetReport,
        },
    ];

    let resps = run_session(TINY_CLINIC, &messages);
    let json1 = resps[2]
        .get_field("report_json")
        .ok()
        .and_then(common_json::Json::as_str)
        .unwrap_or_default()
        .to_string();
    let json2 = resps[3]
        .get_field("report_json")
        .ok()
        .and_then(common_json::Json::as_str)
        .unwrap_or_default()
        .to_string();
    assert_eq!(
        json1, json2,
        "repeated GetReport must return identical report_json"
    );
}

/// Test 4: Replay produces identical report as original run
#[test]
fn test_replay_produces_identical_report() {
    let replay_path = std::env::temp_dir().join("hospital_tycoon_test_replay.json");
    let replay_str = replay_path.to_string_lossy().to_string();

    // Original run
    let messages_run = vec![
        OutboundMessage {
            id: 1,
            request: OutboundRequest::NewRun {
                seed: 42,
                ticks: 30,
            },
        },
        OutboundMessage {
            id: 2,
            request: OutboundRequest::RunToEnd,
        },
        OutboundMessage {
            id: 3,
            request: OutboundRequest::GetReport,
        },
        OutboundMessage {
            id: 4,
            request: OutboundRequest::SaveReplay {
                path: replay_str.clone(),
            },
        },
    ];
    let resps_run = run_session(TINY_CLINIC, &messages_run);
    let original_hash = resps_run[2]
        .get_field("run_hash")
        .ok()
        .and_then(common_json::Json::as_str)
        .unwrap_or_default()
        .to_string();
    assert!(
        !original_hash.is_empty(),
        "original run_hash must not be empty"
    );

    let save_type = resps_run[3]
        .get_field("type")
        .ok()
        .and_then(common_json::Json::as_str)
        .unwrap_or_default()
        .to_string();
    assert_eq!(save_type, "Ok", "SaveReplay should return Ok");

    // Replay run
    let messages_replay = vec![
        OutboundMessage {
            id: 1,
            request: OutboundRequest::LoadReplay {
                path: replay_str.clone(),
            },
        },
        OutboundMessage {
            id: 2,
            request: OutboundRequest::ReplayToEnd,
        },
    ];
    let resps_replay = run_session(TINY_CLINIC, &messages_replay);
    let replay_hash = resps_replay[1]
        .get_field("run_hash")
        .ok()
        .and_then(common_json::Json::as_str)
        .unwrap_or_default()
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
