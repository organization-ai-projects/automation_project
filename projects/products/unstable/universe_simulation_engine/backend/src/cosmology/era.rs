use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Era {
    Singularity,
    Inflation,
    QuarkEpoch,
    HadronEpoch,
    LeptonEpoch,
    Nucleosynthesis,
    PhotonEpoch,
    DarkAges,
    Reionization,
    StarFormation,
    GalaxyFormation,
    StellarEvolution,
    PlanetaryFormation,
    Present,
    HeatDeath,
}

static ALL_ERAS: [Era; 15] = [
    Era::Singularity,
    Era::Inflation,
    Era::QuarkEpoch,
    Era::HadronEpoch,
    Era::LeptonEpoch,
    Era::Nucleosynthesis,
    Era::PhotonEpoch,
    Era::DarkAges,
    Era::Reionization,
    Era::StarFormation,
    Era::GalaxyFormation,
    Era::StellarEvolution,
    Era::PlanetaryFormation,
    Era::Present,
    Era::HeatDeath,
];

impl Era {
    pub fn all() -> &'static [Era] {
        &ALL_ERAS
    }

    pub fn index(&self) -> usize {
        ALL_ERAS.iter().position(|e| e == self).unwrap_or(0)
    }

    pub fn next(&self) -> Option<Era> {
        let idx = self.index();
        if idx + 1 < ALL_ERAS.len() {
            Some(ALL_ERAS[idx + 1])
        } else {
            None
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Era::Singularity => "Singularity",
            Era::Inflation => "Inflation",
            Era::QuarkEpoch => "Quark Epoch",
            Era::HadronEpoch => "Hadron Epoch",
            Era::LeptonEpoch => "Lepton Epoch",
            Era::Nucleosynthesis => "Nucleosynthesis",
            Era::PhotonEpoch => "Photon Epoch",
            Era::DarkAges => "Dark Ages",
            Era::Reionization => "Reionization",
            Era::StarFormation => "Star Formation",
            Era::GalaxyFormation => "Galaxy Formation",
            Era::StellarEvolution => "Stellar Evolution",
            Era::PlanetaryFormation => "Planetary Formation",
            Era::Present => "Present",
            Era::HeatDeath => "Heat Death",
        }
    }
}
