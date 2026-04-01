use crate::cosmology::era::Era;

pub struct CosmicTime;

impl CosmicTime {
    pub fn years_for_era(era: &Era) -> f64 {
        match era {
            Era::Singularity => 0.0,
            Era::Inflation => 1e-32,
            Era::QuarkEpoch => 1e-6,
            Era::HadronEpoch => 1.0,
            Era::LeptonEpoch => 10.0,
            Era::Nucleosynthesis => 180.0,
            Era::PhotonEpoch => 380_000.0,
            Era::DarkAges => 150_000_000.0,
            Era::Reionization => 400_000_000.0,
            Era::StarFormation => 1_000_000_000.0,
            Era::GalaxyFormation => 2_000_000_000.0,
            Era::StellarEvolution => 5_000_000_000.0,
            Era::PlanetaryFormation => 9_000_000_000.0,
            Era::Present => 13_800_000_000.0,
            Era::HeatDeath => 1e100,
        }
    }

    pub fn cosmic_time_years(era: &Era, progress: f64) -> f64 {
        let base = Self::years_for_era(era);
        let next_years = era
            .next()
            .map(|e| Self::years_for_era(&e))
            .unwrap_or(base * 10.0);
        base + (next_years - base) * progress.clamp(0.0, 1.0)
    }
}
