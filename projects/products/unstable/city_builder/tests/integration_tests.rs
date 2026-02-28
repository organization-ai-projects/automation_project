use std::path::PathBuf;
use std::process::Command;

fn bin_path() -> &'static str {
    env!("CARGO_BIN_EXE_city_builder")
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

#[test]
fn run_produces_report_with_run_hash() {
    let bin = bin_path();
    let scenario = scenario_path("small_town_growth.json");
    let out = temp_out("run");

    let status = Command::new(bin)
        .args(["run", "--ticks", "10", "--seed", "42",
               "--scenario", scenario.to_str().unwrap(),
               "--out", out.to_str().unwrap()])
        .status().unwrap();
    assert_eq!(status.code(), Some(0), "Expected exit 0");

    let content = std::fs::read_to_string(&out).unwrap();
    let v: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(v.get("run_hash").is_some());
    assert_eq!(v["total_ticks"], 10);

    let _ = std::fs::remove_file(&out);
}

#[test]
fn invalid_scenario_returns_exit_3() {
    let bin = bin_path();
    let out = temp_out("invalid");
    let status = Command::new(bin)
        .args(["run", "--ticks", "5", "--seed", "1",
               "--scenario", "/nonexistent/path/scenario.json",
               "--out", out.to_str().unwrap()])
        .status().unwrap();
    assert_eq!(status.code(), Some(3), "Expected exit 3 for invalid scenario");
    let _ = std::fs::remove_file(&out);
}

#[test]
fn coverage_scenario_runs_successfully() {
    let bin = bin_path();
    let scenario = scenario_path("power_water_coverage.json");
    let out = temp_out("coverage");

    let status = Command::new(bin)
        .args(["run", "--ticks", "5", "--seed", "10",
               "--scenario", scenario.to_str().unwrap(),
               "--out", out.to_str().unwrap()])
        .status().unwrap();
    assert_eq!(status.code(), Some(0));
    let _ = std::fs::remove_file(&out);
}

#[test]
fn traffic_scenario_runs_successfully() {
    let bin = bin_path();
    let scenario = scenario_path("traffic_jam_case.json");
    let out = temp_out("traffic");

    let status = Command::new(bin)
        .args(["run", "--ticks", "5", "--seed", "7",
               "--scenario", scenario.to_str().unwrap(),
               "--out", out.to_str().unwrap()])
        .status().unwrap();
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
        .args(["run", "--ticks", "8", "--seed", "99",
               "--scenario", scenario.to_str().unwrap(),
               "--out", out1.to_str().unwrap()])
        .status().unwrap();
    assert_eq!(s1.code(), Some(0));

    let s2 = Command::new(bin)
        .args(["run", "--ticks", "8", "--seed", "99",
               "--scenario", scenario.to_str().unwrap(),
               "--out", out2.to_str().unwrap()])
        .status().unwrap();
    assert_eq!(s2.code(), Some(0));

    let c1 = std::fs::read_to_string(&out1).unwrap();
    let c2 = std::fs::read_to_string(&out2).unwrap();
    assert_eq!(c1, c2, "Replay mismatch: identical seeds should produce identical reports");

    let _ = std::fs::remove_file(&out1);
    let _ = std::fs::remove_file(&out2);
}
