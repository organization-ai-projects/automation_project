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
    std::env::temp_dir().join(format!("city_builder_{name}_{pid}.json"))
}

#[test]
fn determinism_same_seed_same_report() {
    let bin = bin_path();
    let scenario = scenario_path("small_town_growth.json");
    let out1 = temp_out("det1");
    let out2 = temp_out("det2");

    let r1 = Command::new(bin)
        .args(["run", "--ticks", "10", "--seed", "42",
               "--scenario", scenario.to_str().unwrap(),
               "--out", out1.to_str().unwrap()])
        .status().unwrap();
    assert_eq!(r1.code(), Some(0));

    let r2 = Command::new(bin)
        .args(["run", "--ticks", "10", "--seed", "42",
               "--scenario", scenario.to_str().unwrap(),
               "--out", out2.to_str().unwrap()])
        .status().unwrap();
    assert_eq!(r2.code(), Some(0));

    let b1 = std::fs::read_to_string(&out1).unwrap();
    let b2 = std::fs::read_to_string(&out2).unwrap();
    assert_eq!(b1, b2, "Determinism violated: outputs differ");

    let _ = std::fs::remove_file(&out1);
    let _ = std::fs::remove_file(&out2);
}

#[test]
fn report_has_run_hash() {
    let bin = bin_path();
    let scenario = scenario_path("small_town_growth.json");
    let out = temp_out("hash");

    let r = Command::new(bin)
        .args(["run", "--ticks", "5", "--seed", "1",
               "--scenario", scenario.to_str().unwrap(),
               "--out", out.to_str().unwrap()])
        .status().unwrap();
    assert_eq!(r.code(), Some(0));

    let content = std::fs::read_to_string(&out).unwrap();
    assert!(content.contains("\"run_hash\""), "Report missing run_hash");

    let _ = std::fs::remove_file(&out);
}
