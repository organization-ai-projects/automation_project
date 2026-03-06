use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/scenarios")
}

fn golden_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/golden")
}

fn tmp_file(suffix: &str) -> PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("space_diplo_wars_test_{n}{suffix}"))
}

fn bin() -> String {
    env!("CARGO_BIN_EXE_space_diplo_wars_backend").to_string()
}

#[test]
fn test_run_and_replay_produce_identical_report_bytes() {
    let scenario = fixtures_dir().join("peaceful_trade_pact.json");
    let run_out = tmp_file("_run_report.json");
    let replay_out = tmp_file("_run.replay.json");
    let replay_report_out = tmp_file("_replay_report.json");

    let status = Command::new(bin())
        .args([
            "run",
            "--turns",
            "5",
            "--ticks-per-turn",
            "4",
            "--seed",
            "42",
            "--scenario",
            scenario.to_str().unwrap(),
            "--out",
            run_out.to_str().unwrap(),
            "--replay-out",
            replay_out.to_str().unwrap(),
        ])
        .status()
        .expect("run command");
    assert!(status.success());

    let status = Command::new(bin())
        .args([
            "replay",
            "--replay",
            replay_out.to_str().unwrap(),
            "--out",
            replay_report_out.to_str().unwrap(),
        ])
        .status()
        .expect("replay command");
    assert!(status.success());

    let run_bytes = fs::read(&run_out).expect("read run report");
    let replay_bytes = fs::read(&replay_report_out).expect("read replay report");
    assert_eq!(run_bytes, replay_bytes);

    let golden = fs::read(golden_dir().join("peaceful_trade_pact_run_report.json"))
        .expect("read golden report");
    assert_eq!(run_bytes, golden);

    let _ = fs::remove_file(run_out);
    let _ = fs::remove_file(replay_out);
    let _ = fs::remove_file(replay_report_out);
}

#[test]
fn test_snapshot_command_outputs_deterministic_hashable_state() {
    let scenario = fixtures_dir().join("border_war_small.json");
    let run_out = tmp_file("_report.json");
    let replay_out = tmp_file("_run.replay.json");
    let snapshot_a = tmp_file("_snapshot_a.json");
    let snapshot_b = tmp_file("_snapshot_b.json");

    let status = Command::new(bin())
        .args([
            "run",
            "--turns",
            "2",
            "--ticks-per-turn",
            "4",
            "--seed",
            "42",
            "--scenario",
            scenario.to_str().unwrap(),
            "--out",
            run_out.to_str().unwrap(),
            "--replay-out",
            replay_out.to_str().unwrap(),
        ])
        .status()
        .expect("run command");
    assert!(status.success());

    for out in [&snapshot_a, &snapshot_b] {
        let status = Command::new(bin())
            .args([
                "snapshot",
                "--replay",
                replay_out.to_str().unwrap(),
                "--at-turn",
                "2",
                "--out",
                out.to_str().unwrap(),
            ])
            .status()
            .expect("snapshot command");
        assert!(status.success());
    }

    assert_eq!(
        fs::read(&snapshot_a).unwrap(),
        fs::read(&snapshot_b).unwrap()
    );

    let _ = fs::remove_file(run_out);
    let _ = fs::remove_file(replay_out);
    let _ = fs::remove_file(snapshot_a);
    let _ = fs::remove_file(snapshot_b);
}

#[test]
fn test_validate_exit_code_for_invalid_scenario() {
    let missing = tmp_file("_missing.json");
    let output = Command::new(bin())
        .args(["validate", "--scenario", missing.to_str().unwrap()])
        .output()
        .expect("validate command");

    assert_eq!(output.status.code(), Some(3));
}

#[test]
fn test_scripted_treaty_decisions_are_applied() {
    let scenario = fixtures_dir().join("scripted_treaty_decision.json");
    let run_out = tmp_file("_scripted_treaty_report.json");
    let replay_out = tmp_file("_scripted_treaty.replay.json");

    let status = Command::new(bin())
        .args([
            "run",
            "--turns",
            "3",
            "--ticks-per-turn",
            "4",
            "--seed",
            "42",
            "--scenario",
            scenario.to_str().unwrap(),
            "--out",
            run_out.to_str().unwrap(),
            "--replay-out",
            replay_out.to_str().unwrap(),
        ])
        .status()
        .expect("run command");
    assert!(status.success());

    let report = fs::read_to_string(&run_out).expect("read run report");
    assert!(report.contains("TreatyAccepted:treaty_empire_a_empire_b_1"));
    assert!(report.contains("\"treaty_reports\":[{"));

    let _ = fs::remove_file(run_out);
    let _ = fs::remove_file(replay_out);
}
