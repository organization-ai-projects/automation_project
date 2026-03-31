use crate::io::JsonCodec;
use crate::model::{FuzzResult, FuzzTarget};
use crate::replay::ReplayEngine;
use crate::replay::ReplayFile;
use crate::runner::FuzzRunner;
use crate::shrinker::InputShrinker;
use crate::targets::DummyTarget;

#[test]
fn dummy_target_finds_failure() {
    let report = FuzzRunner::run(&DummyTarget, 42, 1000).unwrap();
    assert!(
        !report.failures.is_empty(),
        "dummy target should find at least one failure in 1000 iterations"
    );
}

#[test]
fn run_is_deterministic() {
    let report_a = FuzzRunner::run(&DummyTarget, 42, 500).unwrap();
    let report_b = FuzzRunner::run(&DummyTarget, 42, 500).unwrap();

    assert_eq!(report_a.failures.len(), report_b.failures.len());
    assert_eq!(report_a.run_hash.0, report_b.run_hash.0);
    for (a, b) in report_a.failures.iter().zip(report_b.failures.iter()) {
        assert_eq!(a.input.data, b.input.data);
        assert_eq!(a.input.index, b.input.index);
        assert_eq!(a.message, b.message);
    }
}

#[test]
fn replay_reproduces_failure() {
    let report = FuzzRunner::run(&DummyTarget, 42, 1000).unwrap();
    let first_failure = &report.failures[0];

    let replay = ReplayFile {
        target_name: "dummy".to_string(),
        seed: 42,
        input: first_failure.input.clone(),
        failure_message: first_failure.message.clone(),
    };

    let result = ReplayEngine::replay(&DummyTarget, &replay).unwrap();
    match result {
        FuzzResult::Fail(_) => {}
        FuzzResult::Pass => panic!("replay should reproduce the failure"),
    }
}

#[test]
fn shrink_reduces_failing_input() {
    let report = FuzzRunner::run(&DummyTarget, 42, 1000).unwrap();
    let first_failure = &report.failures[0];

    let replay = ReplayFile {
        target_name: "dummy".to_string(),
        seed: 42,
        input: first_failure.input.clone(),
        failure_message: first_failure.message.clone(),
    };

    let shrink_report = InputShrinker::shrink(&DummyTarget, &replay).unwrap();
    assert!(
        shrink_report.shrunk_input.data.len() <= first_failure.input.data.len(),
        "shrunk input should be <= original"
    );

    let result = DummyTarget.execute(&shrink_report.shrunk_input);
    match result {
        FuzzResult::Fail(_) => {}
        FuzzResult::Pass => panic!("shrunk input should still trigger failure"),
    }
}

#[test]
fn full_pipeline_deterministic() {
    let report = FuzzRunner::run(&DummyTarget, 12345, 500).unwrap();
    if report.failures.is_empty() {
        return;
    }

    let first = &report.failures[0];
    let replay = ReplayFile {
        target_name: "dummy".to_string(),
        seed: 12345,
        input: first.input.clone(),
        failure_message: first.message.clone(),
    };

    let shrink_a = InputShrinker::shrink(&DummyTarget, &replay).unwrap();
    let shrink_b = InputShrinker::shrink(&DummyTarget, &replay).unwrap();

    assert_eq!(shrink_a.shrunk_input.data, shrink_b.shrunk_input.data);
    assert_eq!(shrink_a.shrink_steps, shrink_b.shrink_steps);
}

#[test]
fn replay_file_round_trip_via_json() {
    let report = FuzzRunner::run(&DummyTarget, 42, 1000).unwrap();
    let first_failure = &report.failures[0];

    let replay = ReplayFile {
        target_name: "dummy".to_string(),
        seed: 42,
        input: first_failure.input.clone(),
        failure_message: first_failure.message.clone(),
    };

    let dir = std::env::temp_dir().join("fuzz_harness_test");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("test_replay.json");

    JsonCodec::save_replay_file(&replay, &path).unwrap();
    let loaded = JsonCodec::load_replay_file(&path).unwrap();

    assert_eq!(loaded.target_name, replay.target_name);
    assert_eq!(loaded.seed, replay.seed);
    assert_eq!(loaded.input.data, replay.input.data);
    assert_eq!(loaded.input.index, replay.input.index);
    assert_eq!(loaded.failure_message, replay.failure_message);

    let result = ReplayEngine::replay(&DummyTarget, &loaded).unwrap();
    match result {
        FuzzResult::Fail(_) => {}
        FuzzResult::Pass => panic!("loaded replay should reproduce the failure"),
    }

    std::fs::remove_dir_all(&dir).ok();
}
