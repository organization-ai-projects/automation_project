use crate::particles::particle::Particle;
use crate::particles::particle_id::ParticleId;
use crate::particles::particle_kind::ParticleKind;
use crate::rng::seeded_rng::SeededRng;

const NEUTRON_HALF_LIFE_TICKS: f64 = 610.0;
const MUON_HALF_LIFE_TICKS: f64 = 2.0;

pub struct WeakNuclearEngine;

impl WeakNuclearEngine {
    pub fn decay_probability(particle: &Particle) -> f64 {
        match particle.kind {
            ParticleKind::Neutron => 1.0 - (-0.693 / NEUTRON_HALF_LIFE_TICKS).exp(),
            ParticleKind::Muon => 1.0 - (-0.693 / MUON_HALF_LIFE_TICKS).exp(),
            _ => 0.0,
        }
    }

    pub fn process_decays(particles: &mut Vec<Particle>, rng: &mut SeededRng, next_id: &mut u64) {
        let mut new_particles = Vec::new();
        for p in particles.iter_mut() {
            if !p.alive {
                continue;
            }
            let prob = Self::decay_probability(p);
            if prob <= 0.0 {
                continue;
            }
            if rng.next_f64() < prob {
                match p.kind {
                    ParticleKind::Neutron => {
                        // Beta decay: n -> p + e- + anti-neutrino
                        p.kind = ParticleKind::Proton;
                        p.mass = ParticleKind::Proton.mass_kg();
                        p.charge = ParticleKind::Proton.charge();

                        let electron =
                            Particle::new(ParticleId(*next_id), ParticleKind::Electron, p.position);
                        *next_id += 1;
                        let neutrino = Particle::new(
                            ParticleId(*next_id),
                            ParticleKind::ElectronNeutrino,
                            p.position,
                        );
                        *next_id += 1;
                        new_particles.push(electron);
                        new_particles.push(neutrino);
                    }
                    ParticleKind::Muon => {
                        // Muon decay: mu -> e + neutrino + anti-neutrino
                        p.kind = ParticleKind::Electron;
                        p.mass = ParticleKind::Electron.mass_kg();
                        p.charge = ParticleKind::Electron.charge();

                        let nu_mu = Particle::new(
                            ParticleId(*next_id),
                            ParticleKind::MuonNeutrino,
                            p.position,
                        );
                        *next_id += 1;
                        let nu_e = Particle::new(
                            ParticleId(*next_id),
                            ParticleKind::ElectronNeutrino,
                            p.position,
                        );
                        *next_id += 1;
                        new_particles.push(nu_mu);
                        new_particles.push(nu_e);
                    }
                    _ => {}
                }
            }
        }
        particles.extend(new_particles);
    }
}
