use crate::config::sim_config::SimConfig;
use crate::io::json_codec::JsonCodec;
use crate::replay::replay_codec::ReplayCodec;
use crate::replay::replay_engine::ReplayEngine;
use crate::sim::sim_engine::SimEngine;

#[test]
fn run_and_replay_produce_same_hash() {
    let config = SimConfig {
        seed: 42,
        ticks: 50,
    };

    let (report1, replay_data) = SimEngine::run_sim(&config).unwrap();
    let report2 = ReplayEngine::replay(&replay_data).unwrap();

    assert_eq!(report1.run_hash, report2.run_hash);
    assert_eq!(report1.event_count, report2.event_count);
    assert_eq!(report1.snapshot_hash, report2.snapshot_hash);
}

#[test]
fn two_runs_same_seed_same_hash() {
    let config = SimConfig {
        seed: 99,
        ticks: 100,
    };

    let (report1, _) = SimEngine::run_sim(&config).unwrap();
    let (report2, _) = SimEngine::run_sim(&config).unwrap();

    assert_eq!(report1.run_hash, report2.run_hash);
    assert_eq!(report1.event_count, report2.event_count);
    assert_eq!(report1.snapshot_hash, report2.snapshot_hash);
}

#[test]
fn different_seeds_produce_different_hash() {
    let config1 = SimConfig { seed: 1, ticks: 50 };
    let config2 = SimConfig { seed: 2, ticks: 50 };

    let (report1, _) = SimEngine::run_sim(&config1).unwrap();
    let (report2, _) = SimEngine::run_sim(&config2).unwrap();

    assert_ne!(report1.run_hash, report2.run_hash);
}

#[test]
fn report_serializes_to_stable_json() {
    let config = SimConfig {
        seed: 42,
        ticks: 10,
    };

    let (report, _) = SimEngine::run_sim(&config).unwrap();
    let json = JsonCodec::encode(&report).unwrap();

    // Verify roundtrip: serialize then deserialize produces identical report
    let decoded: crate::report::sim_report::SimReport = JsonCodec::decode(&json).unwrap();
    assert_eq!(decoded.run_hash, report.run_hash);
    assert_eq!(decoded.seed, report.seed);
    assert_eq!(decoded.ticks, report.ticks);
    assert_eq!(decoded.event_count, report.event_count);
    assert_eq!(decoded.snapshot_hash, report.snapshot_hash);
}

#[test]
fn replay_codec_roundtrip() {
    let config = SimConfig {
        seed: 42,
        ticks: 20,
    };

    let (_, replay_data) = SimEngine::run_sim(&config).unwrap();
    let encoded = ReplayCodec::encode(&replay_data).unwrap();
    let decoded = ReplayCodec::decode(&encoded).unwrap();

    assert_eq!(decoded.seed, replay_data.seed);
    assert_eq!(decoded.ticks, replay_data.ticks);
    assert_eq!(decoded.events.len(), replay_data.events.len());
}
