use crate::config::physics_config::PhysicsConfig;
use crate::config::sim_config::SimConfig;
use crate::sim::sim_engine::SimEngine;

#[test]
fn default_config_run() {
    let config = SimConfig::default();
    let report = SimEngine::run(&config).unwrap();
    assert_eq!(report.ticks_run, 1000);
    assert_eq!(report.seed, 42);
    assert!(report.total_particles > 0);
    assert!(!report.run_hash.0.is_empty());
}

#[test]
fn zero_ticks() {
    let config = SimConfig {
        max_ticks: 0,
        ..Default::default()
    };
    let report = SimEngine::run(&config).unwrap();
    assert_eq!(report.ticks_run, 0);
    assert_eq!(report.total_particles, 0);
    assert_eq!(report.total_stars, 0);
}

#[test]
fn disabled_physics() {
    let config = SimConfig {
        max_ticks: 100,
        physics: PhysicsConfig {
            gravity_enabled: false,
            electromagnetism_enabled: false,
            strong_nuclear_enabled: false,
            weak_nuclear_enabled: false,
            dark_matter_enabled: false,
            dark_energy_enabled: false,
            thermodynamics_enabled: false,
        },
        ..Default::default()
    };
    let report = SimEngine::run(&config).unwrap();
    assert_eq!(report.ticks_run, 100);
}

#[test]
fn late_era_reached() {
    let config = SimConfig {
        max_ticks: 750,
        ticks_per_era: 50,
        ..Default::default()
    };
    let report = SimEngine::run(&config).unwrap();
    assert_ne!(report.final_era, "Singularity");
}

#[test]
fn determinism() {
    let config = SimConfig {
        max_ticks: 200,
        seed: 123,
        ..Default::default()
    };
    let r1 = SimEngine::run(&config).unwrap();
    let r2 = SimEngine::run(&config).unwrap();
    assert_eq!(r1.run_hash.0, r2.run_hash.0);
    assert_eq!(r1.snapshot_hashes, r2.snapshot_hashes);
}

#[test]
fn snapshot_hashes_present() {
    let config = SimConfig {
        max_ticks: 50,
        ..Default::default()
    };
    let report = SimEngine::run(&config).unwrap();
    assert!(!report.snapshot_hashes.is_empty());
}
