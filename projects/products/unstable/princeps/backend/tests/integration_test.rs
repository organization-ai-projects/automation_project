use common_json::JsonAccess;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static REPLAY_FILE_COUNTER: AtomicU64 = AtomicU64::new(0);

fn bin_path() -> &'static str {
    env!("CARGO_BIN_EXE_princeps_backend")
}

#[test]
fn run_produces_output() {
    let output = Command::new(bin_path())
        .args(["run", "--days", "10", "--seed", "42", "--json"])
        .output()
        .expect("failed to run princeps");
    assert!(output.status.success(), "princeps run failed: {:?}", output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("run_hash"),
        "output missing run_hash: {}",
        stdout
    );
    assert!(
        stdout.contains("winner"),
        "output missing winner: {}",
        stdout
    );
}

#[test]
fn determinism_same_seed_same_hash() {
    let run1 = Command::new(bin_path())
        .args(["run", "--days", "20", "--seed", "1234", "--json"])
        .output()
        .expect("failed to run princeps");
    let run2 = Command::new(bin_path())
        .args(["run", "--days", "20", "--seed", "1234", "--json"])
        .output()
        .expect("failed to run princeps");
    assert!(run1.status.success());
    assert!(run2.status.success());
    let out1 = String::from_utf8_lossy(&run1.stdout);
    let out2 = String::from_utf8_lossy(&run2.stdout);
    let json1: common_json::Json =
        common_json::from_json_str(&out1).expect("run #1 output must be valid JSON");
    let json2: common_json::Json =
        common_json::from_json_str(&out2).expect("run #2 output must be valid JSON");
    assert_eq!(
        json1, json2,
        "runs with same seed must be semantically identical"
    );
}

#[test]
fn different_seeds_may_differ() {
    let run1 = Command::new(bin_path())
        .args(["run", "--days", "10", "--seed", "1", "--json"])
        .output()
        .expect("failed to run princeps");
    let run2 = Command::new(bin_path())
        .args(["run", "--days", "10", "--seed", "999", "--json"])
        .output()
        .expect("failed to run princeps");
    assert!(run1.status.success());
    assert!(run2.status.success());
    let out1 = String::from_utf8_lossy(&run1.stdout);
    let out2 = String::from_utf8_lossy(&run2.stdout);
    assert!(out1.contains("run_hash"));
    assert!(out2.contains("run_hash"));
}

#[test]
fn replay_produces_identical_report() {
    let sequence = REPLAY_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
    let replay_path = std::env::temp_dir()
        .join(format!(
            "princeps_test_replay_{}_{}.json",
            std::process::id(),
            sequence
        ))
        .to_string_lossy()
        .to_string();

    // Run and save replay
    let run_out = Command::new(bin_path())
        .args([
            "run",
            "--days",
            "15",
            "--seed",
            "777",
            "--json",
            "--replay-out",
            &replay_path,
        ])
        .output()
        .expect("failed to run princeps for replay test");
    assert!(
        run_out.status.success(),
        "run failed: {:?}",
        String::from_utf8_lossy(&run_out.stderr)
    );

    // Replay and compare
    let replay_out = Command::new(bin_path())
        .args(["replay", &replay_path, "--json"])
        .output()
        .expect("failed to replay");
    assert!(
        replay_out.status.success(),
        "replay failed: {:?}",
        String::from_utf8_lossy(&replay_out.stderr)
    );

    let run_stdout = String::from_utf8_lossy(&run_out.stdout);
    let replay_stdout = String::from_utf8_lossy(&replay_out.stdout);
    let run_json: common_json::Json =
        common_json::from_json_str(&run_stdout).expect("run output must be valid JSON");
    let replay_json: common_json::Json =
        common_json::from_json_str(&replay_stdout).expect("replay output must be valid JSON");
    assert_eq!(
        run_json, replay_json,
        "replay must produce semantically identical report"
    );

    let _ = std::fs::remove_file(&replay_path);
}

#[test]
fn export_markdown_matches_golden_fixture() {
    let output = Command::new(bin_path())
        .args([
            "export", "--format", "markdown", "--seed", "42", "--days", "30",
        ])
        .output()
        .expect("failed to run princeps export");
    assert!(output.status.success());
    let actual = String::from_utf8_lossy(&output.stdout);
    let golden_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/golden_export.md");
    let golden = std::fs::read_to_string(&golden_path)
        .expect("golden fixture must exist at tests/fixtures/golden_export.md");
    assert_eq!(
        actual.as_ref(),
        golden.as_str(),
        "export output must match golden fixture"
    );
}

#[test]
fn export_markdown_contains_expected_sections() {
    let output = Command::new(bin_path())
        .args([
            "export", "--format", "markdown", "--seed", "42", "--days", "30",
        ])
        .output()
        .expect("failed to run princeps export");
    assert!(output.status.success());
    let md = String::from_utf8_lossy(&output.stdout);
    assert!(md.contains("# Princeps"), "missing title");
    assert!(md.contains("## Final Poll Results"), "missing poll section");
    assert!(md.contains("**Winner:**"), "missing winner");
    assert!(md.contains("Run Hash"), "missing run hash");
}

#[test]
fn export_json_is_valid() {
    let output = Command::new(bin_path())
        .args(["export", "--format", "json", "--seed", "42", "--days", "30"])
        .output()
        .expect("failed to run princeps export");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: common_json::Json =
        common_json::from_json_str(&stdout).expect("export --format json must produce valid JSON");
    let run_hash = parsed
        .get_field("run_hash")
        .expect("run_hash field must exist");
    assert!(
        run_hash.as_str_strict().is_ok(),
        "run_hash must be a string field"
    );

    let winner = parsed.get_field("winner").expect("winner field must exist");
    assert!(
        winner.as_str_strict().is_ok() || winner.as_object_strict().is_ok(),
        "winner must be a string or object"
    );
}
