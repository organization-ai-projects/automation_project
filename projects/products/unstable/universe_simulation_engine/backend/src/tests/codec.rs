use crate::config::sim_config::SimConfig;
use crate::io::binary_codec;
use crate::io::ron_codec;
use crate::sim::sim_engine::SimEngine;
use std::path::PathBuf;

fn test_output_dir() -> PathBuf {
    let dir = PathBuf::from("test_output_universe");
    std::fs::create_dir_all(&dir).ok();
    dir
}

#[test]
fn binary_round_trip() {
    let config = SimConfig {
        max_ticks: 50,
        ..Default::default()
    };
    let report = SimEngine::run(&config).unwrap();
    let dir = test_output_dir();
    let path = dir.join("test_report.usim");

    binary_codec::save_binary(&report, &path).unwrap();
    let loaded: crate::report::sim_report::SimReport = binary_codec::load_binary(&path).unwrap();

    assert_eq!(loaded.run_hash.0, report.run_hash.0);
    assert_eq!(loaded.ticks_run, report.ticks_run);
    assert_eq!(loaded.seed, report.seed);

    std::fs::remove_file(&path).ok();
    std::fs::remove_dir(&dir).ok();
}

#[test]
fn ron_round_trip() {
    let config = SimConfig {
        max_ticks: 50,
        ..Default::default()
    };
    let report = SimEngine::run(&config).unwrap();
    let dir = test_output_dir();
    let path = dir.join("test_report.ron");

    ron_codec::save_ron(&report, &path).unwrap();
    let loaded: crate::report::sim_report::SimReport = ron_codec::load_ron(&path).unwrap();

    assert_eq!(loaded.run_hash.0, report.run_hash.0);
    assert_eq!(loaded.ticks_run, report.ticks_run);

    std::fs::remove_file(&path).ok();
    std::fs::remove_dir(&dir).ok();
}
