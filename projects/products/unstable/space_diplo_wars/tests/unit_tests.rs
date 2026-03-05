use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/scenarios")
}

fn tmp_file(suffix: &str) -> PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("space_diplo_wars_unit_{n}{suffix}"))
}

fn bin() -> String {
    env!("CARGO_BIN_EXE_space_diplo_wars").to_string()
}

#[test]
fn test_invalid_cli_returns_exit_code_2() {
    let output = Command::new(bin())
        .args(["run", "--turns", "abc"])
        .output()
        .expect("run command");
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn test_replay_mismatch_returns_exit_code_4() {
    let scenario = fixtures_dir().join("border_war_small.json");
    let run_out = tmp_file("_report.json");
    let replay_out = tmp_file("_replay.json");

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

    // Corrupt replay payload to force replay decode mismatch path (exit code 4).
    std::fs::write(&replay_out, "{invalid_json").expect("write invalid replay");
    let replay_report_out = tmp_file("_replay_report.json");

    let output = Command::new(bin())
        .args([
            "replay",
            "--replay",
            replay_out.to_str().unwrap(),
            "--out",
            replay_report_out.to_str().unwrap(),
        ])
        .output()
        .expect("replay command");

    assert_eq!(output.status.code(), Some(4));

    let _ = std::fs::remove_file(run_out);
    let _ = std::fs::remove_file(replay_out);
    let _ = std::fs::remove_file(replay_report_out);
}
