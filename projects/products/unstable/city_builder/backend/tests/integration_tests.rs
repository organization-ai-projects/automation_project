use common_json::JsonAccess;
use std::path::PathBuf;
use std::process::Command;

fn bin_path() -> &'static str {
    env!("CARGO_BIN_EXE_city_builder_backend")
}

fn scenario_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/scenarios")
        .join(name)
}

fn temp_out(name: &str) -> PathBuf {
    let pid = std::process::id();
    std::env::temp_dir().join(format!("city_builder_int_{name}_{pid}.json"))
}

fn temp_replay(name: &str) -> PathBuf {
    let pid = std::process::id();
    std::env::temp_dir().join(format!("city_builder_int_{name}_{pid}.replay.json"))
}

#[test]
fn run_produces_report_with_run_hash() {
    let bin = bin_path();
    let scenario = scenario_path("small_town_growth.json");
    let out = temp_out("run");

    let status = Command::new(bin)
        .args([
            "run",
            "--ticks",
            "10",
            "--seed",
            "42",
            "--scenario",
            scenario.to_str().unwrap(),
            "--out",
            out.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    assert_eq!(status.code(), Some(0), "Expected exit 0");

    let content = std::fs::read_to_string(&out).unwrap();
    let v: common_json::Json = common_json::from_str(&content).unwrap();
    assert!(v.get_field("run_hash").is_ok());
    assert_eq!(
        v.get_field("total_ticks").unwrap().as_u64_strict().unwrap(),
        10
    );

    let _ = std::fs::remove_file(&out);
}

#[test]
fn invalid_scenario_returns_exit_3() {
    let bin = bin_path();
    let out = temp_out("invalid");
    let status = Command::new(bin)
        .args([
            "run",
            "--ticks",
            "5",
            "--seed",
            "1",
            "--scenario",
            "/nonexistent/path/scenario.json",
            "--out",
            out.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    assert_eq!(
        status.code(),
        Some(3),
        "Expected exit 3 for invalid scenario"
    );
    let _ = std::fs::remove_file(&out);
}

#[test]
fn coverage_scenario_runs_successfully() {
    let bin = bin_path();
    let scenario = scenario_path("power_water_coverage.json");
    let out = temp_out("coverage");

    let status = Command::new(bin)
        .args([
            "run",
            "--ticks",
            "5",
            "--seed",
            "10",
            "--scenario",
            scenario.to_str().unwrap(),
            "--out",
            out.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    assert_eq!(status.code(), Some(0));
    let _ = std::fs::remove_file(&out);
}

#[test]
fn traffic_scenario_runs_successfully() {
    let bin = bin_path();
    let scenario = scenario_path("traffic_jam_case.json");
    let out = temp_out("traffic");

    let status = Command::new(bin)
        .args([
            "run",
            "--ticks",
            "5",
            "--seed",
            "7",
            "--scenario",
            scenario.to_str().unwrap(),
            "--out",
            out.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    assert_eq!(status.code(), Some(0));
    let _ = std::fs::remove_file(&out);
}

#[test]
fn replay_same_seed_produces_same_report() {
    let bin = bin_path();
    let scenario = scenario_path("small_town_growth.json");
    let out1 = temp_out("replay1");
    let out2 = temp_out("replay2");

    let s1 = Command::new(bin)
        .args([
            "run",
            "--ticks",
            "8",
            "--seed",
            "99",
            "--scenario",
            scenario.to_str().unwrap(),
            "--out",
            out1.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    assert_eq!(s1.code(), Some(0));

    let s2 = Command::new(bin)
        .args([
            "run",
            "--ticks",
            "8",
            "--seed",
            "99",
            "--scenario",
            scenario.to_str().unwrap(),
            "--out",
            out2.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    assert_eq!(s2.code(), Some(0));

    let c1 = std::fs::read_to_string(&out1).unwrap();
    let c2 = std::fs::read_to_string(&out2).unwrap();
    let j1: common_json::Json = common_json::from_str(&c1).unwrap();
    let j2: common_json::Json = common_json::from_str(&c2).unwrap();
    assert_eq!(
        j1, j2,
        "Replay mismatch: identical seeds should produce identical reports"
    );

    let _ = std::fs::remove_file(&out1);
    let _ = std::fs::remove_file(&out2);
}

#[test]
fn validate_scenario_command_returns_zero() {
    let bin = bin_path();
    let scenario = scenario_path("small_town_growth.json");
    let status = Command::new(bin)
        .args(["validate", "--scenario", scenario.to_str().unwrap()])
        .status()
        .unwrap();
    assert_eq!(status.code(), Some(0));
}

#[test]
fn replay_command_matches_run_report() {
    let bin = bin_path();
    let scenario = scenario_path("small_town_growth.json");
    let run_out = temp_out("run_with_replay");
    let replay_out = temp_replay("record");
    let replay_report = temp_out("replay_report");

    let run_status = Command::new(bin)
        .args([
            "run",
            "--ticks",
            "12",
            "--seed",
            "123",
            "--scenario",
            scenario.to_str().unwrap(),
            "--out",
            run_out.to_str().unwrap(),
            "--replay-out",
            replay_out.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    assert_eq!(run_status.code(), Some(0));

    let replay_status = Command::new(bin)
        .args([
            "replay",
            "--replay",
            replay_out.to_str().unwrap(),
            "--out",
            replay_report.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    assert_eq!(replay_status.code(), Some(0));

    let run_json: common_json::Json =
        common_json::from_str(&std::fs::read_to_string(&run_out).unwrap()).unwrap();
    let replay_json: common_json::Json =
        common_json::from_str(&std::fs::read_to_string(&replay_report).unwrap()).unwrap();
    assert_eq!(run_json, replay_json);

    let _ = std::fs::remove_file(&run_out);
    let _ = std::fs::remove_file(&replay_out);
    let _ = std::fs::remove_file(&replay_report);
}

#[test]
fn snapshot_command_writes_requested_tick() {
    let bin = bin_path();
    let scenario = scenario_path("small_town_growth.json");
    let run_out = temp_out("snapshot_run");
    let replay_out = temp_replay("snapshot_record");
    let snapshot_out = temp_out("snapshot_output");

    let run_status = Command::new(bin)
        .args([
            "run",
            "--ticks",
            "9",
            "--seed",
            "999",
            "--scenario",
            scenario.to_str().unwrap(),
            "--out",
            run_out.to_str().unwrap(),
            "--replay-out",
            replay_out.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    assert_eq!(run_status.code(), Some(0));

    let snap_status = Command::new(bin)
        .args([
            "snapshot",
            "--replay",
            replay_out.to_str().unwrap(),
            "--at-tick",
            "5",
            "--out",
            snapshot_out.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    assert_eq!(snap_status.code(), Some(0));

    let snapshot_json: common_json::Json =
        common_json::from_str(&std::fs::read_to_string(&snapshot_out).unwrap()).unwrap();
    assert_eq!(
        snapshot_json
            .get_field("tick")
            .unwrap()
            .as_u64_strict()
            .unwrap(),
        5
    );
    assert!(snapshot_json.get_field("snapshot_hash").is_ok());
    assert!(snapshot_json.get_field("state").is_ok());

    let _ = std::fs::remove_file(&run_out);
    let _ = std::fs::remove_file(&replay_out);
    let _ = std::fs::remove_file(&snapshot_out);
}
