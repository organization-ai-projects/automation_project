use crate::cosmology::era::Era;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CosmicParameters {
    pub temperature: f64,
    pub density: f64,
    pub scale_factor: f64,
    pub expansion_rate: f64,
}

impl CosmicParameters {
    pub fn at_era(era: &Era, progress: f64) -> Self {
        let t = progress.clamp(0.0, 1.0);
        match era {
            Era::Singularity => Self {
                temperature: 1e32 * (1.0 - t * 0.1),
                density: 1e96 * (1.0 - t * 0.1),
                scale_factor: 1e-30,
                expansion_rate: 1e43,
            },
            Era::Inflation => Self {
                temperature: 1e28 * (1.0 - t * 0.5),
                density: 1e80 * (1.0 - t * 0.9),
                scale_factor: 1e-30 + t * 1e26,
                expansion_rate: 1e43 * (1.0 - t * 0.99),
            },
            Era::QuarkEpoch => Self {
                temperature: 1e12 * (1.0 - t * 0.3),
                density: 1e18 * (1.0 - t * 0.2),
                scale_factor: 1e-6 + t * 1e-5,
                expansion_rate: 1e10,
            },
            Era::HadronEpoch => Self {
                temperature: 1e10 * (1.0 - t * 0.2),
                density: 1e14 * (1.0 - t * 0.3),
                scale_factor: 1e-4 + t * 1e-3,
                expansion_rate: 1e8,
            },
            Era::LeptonEpoch => Self {
                temperature: 1e9 * (1.0 - t * 0.3),
                density: 1e10 * (1.0 - t * 0.4),
                scale_factor: 1e-3 + t * 1e-2,
                expansion_rate: 1e6,
            },
            Era::Nucleosynthesis => Self {
                temperature: 1e9 * (1.0 - t * 0.8),
                density: 1e5 * (1.0 - t * 0.5),
                scale_factor: 0.01 + t * 0.09,
                expansion_rate: 1e4,
            },
            Era::PhotonEpoch => Self {
                temperature: 3000.0 * (1.0 - t * 0.5),
                density: 1e3 * (1.0 - t * 0.3),
                scale_factor: 0.1 + t * 0.2,
                expansion_rate: 1e3,
            },
            Era::DarkAges => Self {
                temperature: 1500.0 * (1.0 - t * 0.9),
                density: 100.0 * (1.0 - t * 0.5),
                scale_factor: 0.3 + t * 0.1,
                expansion_rate: 100.0,
            },
            Era::Reionization => Self {
                temperature: 1e4 + t * 1e4,
                density: 10.0 * (1.0 - t * 0.3),
                scale_factor: 0.4 + t * 0.1,
                expansion_rate: 80.0,
            },
            Era::StarFormation => Self {
                temperature: 1e4 * (1.0 - t * 0.5),
                density: 5.0 * (1.0 - t * 0.3),
                scale_factor: 0.5 + t * 0.15,
                expansion_rate: 70.0,
            },
            Era::GalaxyFormation => Self {
                temperature: 5000.0 * (1.0 - t * 0.5),
                density: 3.0 * (1.0 - t * 0.3),
                scale_factor: 0.65 + t * 0.15,
                expansion_rate: 68.0,
            },
            Era::StellarEvolution => Self {
                temperature: 100.0 * (1.0 - t * 0.5),
                density: 1.0 * (1.0 - t * 0.3),
                scale_factor: 0.8 + t * 0.1,
                expansion_rate: 67.5,
            },
            Era::PlanetaryFormation => Self {
                temperature: 10.0 * (1.0 - t * 0.5),
                density: 0.5 * (1.0 - t * 0.2),
                scale_factor: 0.9 + t * 0.05,
                expansion_rate: 67.4,
            },
            Era::Present => Self {
                temperature: 2.725,
                density: 0.3,
                scale_factor: 1.0,
                expansion_rate: 67.4,
            },
            Era::HeatDeath => Self {
                temperature: 1e-30 * (1.0 - t * 0.9),
                density: 1e-40,
                scale_factor: 1e30,
                expansion_rate: 1e20,
            },
        }
    }
}
