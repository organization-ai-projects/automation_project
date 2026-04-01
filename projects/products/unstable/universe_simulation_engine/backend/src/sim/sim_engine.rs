use crate::config::sim_config::SimConfig;
use crate::cosmology::cosmic_parameters::CosmicParameters;
use crate::cosmology::era::Era;
use crate::cosmology::era_transition::EraTransition;
use crate::diagnostics::engine_error::EngineError;
use crate::math::vec3::Vec3;
use crate::particles::particle::Particle;
use crate::particles::particle_id::ParticleId;
use crate::particles::particle_kind::ParticleKind;
use crate::physics::dark_energy::DarkEnergyEngine;
use crate::physics::dark_matter::DarkMatterEngine;
use crate::physics::electromagnetism::ElectromagnetismEngine;
use crate::physics::gravity::GravityEngine;
use crate::physics::nuclear_strong::StrongNuclearEngine;
use crate::physics::nuclear_weak::WeakNuclearEngine;
use crate::physics::thermodynamics::ThermodynamicsEngine;
use crate::report::run_hash::RunHash;
use crate::report::sim_report::SimReport;
use crate::rng::seeded_rng::SeededRng;
use crate::sim::sim_event::{EventLog, SimEvent};
use crate::sim::sim_state::SimState;
use crate::snapshot::snapshot_hash::SnapshotHash;
use crate::snapshot::state_snapshot::StateSnapshot;
use crate::structures::cosmic_web::{Filament, Void};
use crate::structures::galaxy::{Galaxy, GalaxyType};
use crate::structures::star::Star;
use crate::structures::structure_id::StructureId;
use crate::time::cosmic_time::CosmicTime;
use std::collections::BTreeMap;

pub struct SimEngine;

impl SimEngine {
    pub fn run(config: &SimConfig) -> Result<SimReport, EngineError> {
        let mut rng = SeededRng::from_seed(config.seed);
        let mut state = SimState::new(config.physics.clone());
        let mut event_log = EventLog::default();
        let mut snapshot_hashes: BTreeMap<String, String> = BTreeMap::new();
        let mut prev_era = Era::Singularity;

        let dt = 1.0;

        for tick_idx in 0..config.max_ticks {
            state.clock.tick();
            let current_tick = state.clock.current();

            let new_era = EraTransition::era_for_tick(tick_idx, config.ticks_per_era);
            let progress = EraTransition::era_progress(tick_idx, config.ticks_per_era);
            state.era = new_era;
            state.era_progress = progress;
            state.cosmic_params = CosmicParameters::at_era(&new_era, progress);

            if new_era != prev_era {
                event_log.record(
                    current_tick,
                    SimEvent::EraTransition {
                        era_name: new_era.display_name().to_string(),
                    },
                );
                Self::on_era_enter(&new_era, &mut state, &mut rng, &mut event_log);
                prev_era = new_era;
            }

            Self::apply_physics(&mut state, &mut rng, &mut event_log, dt);
            Self::process_era_logic(
                &new_era,
                tick_idx,
                config.ticks_per_era,
                &mut state,
                &mut rng,
                &mut event_log,
            );

            for p in &mut state.particles {
                p.step(dt);
            }

            state.spatial_grid.clear();
            for p in &state.particles {
                if p.alive {
                    state
                        .spatial_grid
                        .insert(p.id.0, p.position.x, p.position.y, p.position.z);
                }
            }

            for star in &mut state.stars {
                star.evolve();
            }
            for galaxy in &mut state.galaxies {
                galaxy.evolve();
            }

            if (tick_idx + 1) % 10 == 0 || tick_idx + 1 == config.max_ticks {
                let snap = StateSnapshot::take(current_tick, &state);
                let hash = SnapshotHash::compute(&snap);
                snapshot_hashes.insert(format!("tick_{}", current_tick.value()), hash.0);
            }
        }

        let final_era = state.era;
        let final_years = CosmicTime::cosmic_time_years(&final_era, state.era_progress);
        let alive_particles = state.particles.iter().filter(|p| p.alive).count();
        let alive_stars = state.stars.iter().filter(|s| s.alive).count();

        let mut report = SimReport {
            ticks_run: config.max_ticks,
            seed: config.seed,
            final_era: final_era.display_name().to_string(),
            final_cosmic_time_years: final_years,
            total_particles: alive_particles,
            total_stars: alive_stars,
            total_galaxies: state.galaxies.len(),
            filament_count: state.cosmic_web.filaments.len(),
            void_count: state.cosmic_web.voids.len(),
            event_count: event_log.len(),
            snapshot_hashes,
            run_hash: RunHash(String::new()),
            physics_config: config.physics.clone(),
        };
        report.compute_hash();
        Ok(report)
    }

    fn on_era_enter(
        era: &Era,
        state: &mut SimState,
        rng: &mut SeededRng,
        event_log: &mut EventLog,
    ) {
        let tick = state.clock.current();
        match era {
            Era::QuarkEpoch => {
                let count = 20;
                for _ in 0..count {
                    let pos = Vec3::new(
                        rng.next_range(-1e-5, 1e-5),
                        rng.next_range(-1e-5, 1e-5),
                        rng.next_range(-1e-5, 1e-5),
                    );
                    let kinds = [
                        ParticleKind::UpQuark,
                        ParticleKind::DownQuark,
                        ParticleKind::Gluon,
                    ];
                    let kind = kinds[(rng.next_u64() % 3) as usize];
                    let p = Particle::new(ParticleId(state.next_particle_id), kind, pos);
                    state.next_particle_id += 1;
                    state.particles.push(p);
                }
                event_log.record(tick, SimEvent::ParticlesCreated { count });
            }
            Era::HadronEpoch => {
                let mut combined = 0;
                let count = 10;
                for _ in 0..count {
                    let pos = Vec3::new(
                        rng.next_range(-1e-3, 1e-3),
                        rng.next_range(-1e-3, 1e-3),
                        rng.next_range(-1e-3, 1e-3),
                    );
                    let kind = if rng.next_f64() < 0.5 {
                        ParticleKind::Proton
                    } else {
                        ParticleKind::Neutron
                    };
                    let p = Particle::new(ParticleId(state.next_particle_id), kind, pos);
                    state.next_particle_id += 1;
                    state.particles.push(p);
                    combined += 1;
                }
                event_log.record(tick, SimEvent::ParticlesCombined { count: combined });
            }
            Era::LeptonEpoch => {
                let count = 15;
                for _ in 0..count {
                    let pos = Vec3::new(
                        rng.next_range(-1e-2, 1e-2),
                        rng.next_range(-1e-2, 1e-2),
                        rng.next_range(-1e-2, 1e-2),
                    );
                    let kind = if rng.next_f64() < 0.6 {
                        ParticleKind::Electron
                    } else {
                        ParticleKind::Photon
                    };
                    let p = Particle::new(ParticleId(state.next_particle_id), kind, pos);
                    state.next_particle_id += 1;
                    state.particles.push(p);
                }
                event_log.record(tick, SimEvent::ParticlesCreated { count });
            }
            Era::Nucleosynthesis => {
                let count = 10;
                for _ in 0..count {
                    let pos = Vec3::new(
                        rng.next_range(-1.0, 1.0),
                        rng.next_range(-1.0, 1.0),
                        rng.next_range(-1.0, 1.0),
                    );
                    let kind = if rng.next_f64() < 0.75 {
                        ParticleKind::Hydrogen
                    } else {
                        ParticleKind::Helium
                    };
                    let p = Particle::new(ParticleId(state.next_particle_id), kind, pos);
                    state.next_particle_id += 1;
                    state.particles.push(p);
                }
                event_log.record(tick, SimEvent::ParticlesCreated { count });
            }
            Era::StarFormation => {
                let count = 5;
                for _ in 0..count {
                    let pos = Vec3::new(
                        rng.next_range(-1e15, 1e15),
                        rng.next_range(-1e15, 1e15),
                        rng.next_range(-1e15, 1e15),
                    );
                    let mass = rng.next_range(0.3, 50.0) * crate::math::constants::SOLAR_MASS;
                    let star = Star::new(StructureId(state.next_structure_id), pos, mass);
                    event_log.record(tick, SimEvent::StarFormed { mass });
                    state.next_structure_id += 1;
                    state.stars.push(star);
                }
            }
            Era::GalaxyFormation => {
                let galaxy_types = [
                    GalaxyType::Spiral,
                    GalaxyType::Elliptical,
                    GalaxyType::Irregular,
                    GalaxyType::Lenticular,
                ];
                for i in 0..3 {
                    let pos = Vec3::new(
                        rng.next_range(-1e22, 1e22),
                        rng.next_range(-1e22, 1e22),
                        rng.next_range(-1e22, 1e22),
                    );
                    let mass = rng.next_range(1e9, 1e12) * crate::math::constants::SOLAR_MASS;
                    let gtype = galaxy_types[i % galaxy_types.len()];
                    let galaxy =
                        Galaxy::new(StructureId(state.next_structure_id), pos, mass, gtype);
                    event_log.record(
                        tick,
                        SimEvent::GalaxyFormed {
                            galaxy_type: format!("{:?}", gtype),
                        },
                    );
                    state.next_structure_id += 1;
                    state.galaxies.push(galaxy);
                }

                // Create filaments and voids
                let filament = Filament {
                    id: StructureId(state.next_structure_id),
                    start: Vec3::new(
                        rng.next_range(-1e23, 1e23),
                        rng.next_range(-1e23, 1e23),
                        rng.next_range(-1e23, 1e23),
                    ),
                    end: Vec3::new(
                        rng.next_range(-1e23, 1e23),
                        rng.next_range(-1e23, 1e23),
                        rng.next_range(-1e23, 1e23),
                    ),
                    mass: rng.next_range(1e14, 1e16) * crate::math::constants::SOLAR_MASS,
                    galaxy_count: 50,
                };
                state.next_structure_id += 1;
                state.cosmic_web.add_filament(filament);
                event_log.record(tick, SimEvent::FilamentFormed);

                let void_region = Void {
                    id: StructureId(state.next_structure_id),
                    center: Vec3::new(
                        rng.next_range(-1e24, 1e24),
                        rng.next_range(-1e24, 1e24),
                        rng.next_range(-1e24, 1e24),
                    ),
                    radius: rng.next_range(1e23, 1e24),
                };
                state.next_structure_id += 1;
                state.cosmic_web.add_void(void_region);
            }
            _ => {}
        }
    }

    fn apply_physics(state: &mut SimState, rng: &mut SeededRng, event_log: &mut EventLog, dt: f64) {
        let tick = state.clock.current();

        if state.physics_config.gravity_enabled {
            GravityEngine::apply_to_particles(&mut state.particles, dt);
            GravityEngine::apply_to_galaxies(&mut state.galaxies, dt);
            event_log.record(
                tick,
                SimEvent::PhysicsApplied {
                    engine: "gravity".to_string(),
                },
            );
        }

        if state.physics_config.electromagnetism_enabled {
            ElectromagnetismEngine::apply_to_particles(&mut state.particles, dt);
            event_log.record(
                tick,
                SimEvent::PhysicsApplied {
                    engine: "electromagnetism".to_string(),
                },
            );
        }

        if state.physics_config.strong_nuclear_enabled {
            StrongNuclearEngine::apply_to_particles(&mut state.particles, dt);
            event_log.record(
                tick,
                SimEvent::PhysicsApplied {
                    engine: "strong_nuclear".to_string(),
                },
            );
        }

        if state.physics_config.weak_nuclear_enabled {
            WeakNuclearEngine::process_decays(
                &mut state.particles,
                rng,
                &mut state.next_particle_id,
            );
            event_log.record(
                tick,
                SimEvent::PhysicsApplied {
                    engine: "weak_nuclear".to_string(),
                },
            );
        }

        if state.physics_config.thermodynamics_enabled {
            ThermodynamicsEngine::apply_thermal_motion(
                &mut state.particles,
                &state.cosmic_params,
                rng,
            );
            event_log.record(
                tick,
                SimEvent::PhysicsApplied {
                    engine: "thermodynamics".to_string(),
                },
            );
        }

        if state.physics_config.dark_matter_enabled {
            DarkMatterEngine::apply_to_galaxies(&mut state.galaxies, dt);
            event_log.record(
                tick,
                SimEvent::PhysicsApplied {
                    engine: "dark_matter".to_string(),
                },
            );
        }

        if state.physics_config.dark_energy_enabled {
            state.cosmic_params = DarkEnergyEngine::apply_expansion(&state.cosmic_params);
            event_log.record(
                tick,
                SimEvent::UniverseExpanded {
                    scale_factor: state.cosmic_params.scale_factor,
                },
            );
        }
    }

    fn process_era_logic(
        era: &Era,
        tick_idx: u64,
        ticks_per_era: u64,
        state: &mut SimState,
        rng: &mut SeededRng,
        event_log: &mut EventLog,
    ) {
        let tick = state.clock.current();
        match era {
            Era::StarFormation | Era::StellarEvolution => {
                if tick_idx % 20 == 0 {
                    let pos = Vec3::new(
                        rng.next_range(-1e16, 1e16),
                        rng.next_range(-1e16, 1e16),
                        rng.next_range(-1e16, 1e16),
                    );
                    let mass = rng.next_range(0.5, 30.0) * crate::math::constants::SOLAR_MASS;
                    let star = Star::new(StructureId(state.next_structure_id), pos, mass);
                    state.next_structure_id += 1;
                    state.stars.push(star);
                    event_log.record(tick, SimEvent::StarFormed { mass });
                }

                for star in &state.stars {
                    if !star.alive && star.age_ticks == star.lifetime_ticks() {
                        event_log.record(
                            tick,
                            SimEvent::StarDied {
                                class: format!("{:?}", star.class),
                            },
                        );
                    }
                }
            }
            Era::PlanetaryFormation | Era::Present => {
                if tick_idx % (ticks_per_era.max(1) / 2).max(1) == 0 {
                    let pos = Vec3::new(
                        rng.next_range(-1e10, 1e10),
                        rng.next_range(-1e10, 1e10),
                        rng.next_range(-1e10, 1e10),
                    );
                    let heavier = [
                        ParticleKind::Carbon,
                        ParticleKind::Nitrogen,
                        ParticleKind::Oxygen,
                        ParticleKind::Iron,
                    ];
                    let kind = heavier[(rng.next_u64() % 4) as usize];
                    let p = Particle::new(ParticleId(state.next_particle_id), kind, pos);
                    state.next_particle_id += 1;
                    state.particles.push(p);
                    event_log.record(tick, SimEvent::ParticlesCreated { count: 1 });
                }
            }
            _ => {}
        }
    }
}
