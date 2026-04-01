use crate::config::sim_config::SimConfig;
use crate::sim::sim_engine::SimEngine;
use crate::sim::sim_state::SimState;
use crate::snapshot::snapshot_hash::SnapshotHash;
use crate::snapshot::state_snapshot::StateSnapshot;
use crate::time::tick::Tick;

#[test]
fn snapshot_take() {
    let state = SimState::new(Default::default());
    let snap = StateSnapshot::take(Tick(1), &state);
    assert_eq!(snap.tick.value(), 1);
}

#[test]
fn hash_deterministic() {
    let state = SimState::new(Default::default());
    let snap = StateSnapshot::take(Tick(1), &state);
    let h1 = SnapshotHash::compute(&snap);
    let h2 = SnapshotHash::compute(&snap);
    assert_eq!(h1.0, h2.0);
}

#[test]
fn hash_changes_with_different_states() {
    let config1 = SimConfig {
        max_ticks: 10,
        seed: 1,
        ..Default::default()
    };
    let config2 = SimConfig {
        max_ticks: 10,
        seed: 2,
        ..Default::default()
    };
    let r1 = SimEngine::run(&config1).unwrap();
    let r2 = SimEngine::run(&config2).unwrap();
    assert_ne!(r1.run_hash.0, r2.run_hash.0);
}
