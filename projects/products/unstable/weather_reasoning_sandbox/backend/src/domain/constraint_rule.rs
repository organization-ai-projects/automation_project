use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ConstraintRule {
    PressureDropClearSky,
    LowHumidityPrecipitation,
    LowCloudStorm,
    HighWindCalm,
    InstabilityCoherence,
    MutualIncoherence,
}

impl ConstraintRule {
    pub fn all() -> &'static [ConstraintRule] {
        &[
            ConstraintRule::PressureDropClearSky,
            ConstraintRule::LowHumidityPrecipitation,
            ConstraintRule::LowCloudStorm,
            ConstraintRule::HighWindCalm,
            ConstraintRule::InstabilityCoherence,
            ConstraintRule::MutualIncoherence,
        ]
    }

    pub fn description(&self) -> &'static str {
        match self {
            ConstraintRule::PressureDropClearSky => {
                "Sharp pressure drop with high humidity penalizes clear-sky confidence"
            }
            ConstraintRule::LowHumidityPrecipitation => {
                "Extremely low humidity penalizes precipitation confidence"
            }
            ConstraintRule::LowCloudStorm => {
                "Low cloudiness without instability reduces storm probability"
            }
            ConstraintRule::HighWindCalm => {
                "High wind intensity penalizes calm-condition confidence"
            }
            ConstraintRule::InstabilityCoherence => {
                "Instability indicators must be coherent with storm likelihood"
            }
            ConstraintRule::MutualIncoherence => {
                "Mutually incoherent atmospheric state outputs are normalized"
            }
        }
    }

    pub fn id(&self) -> &'static str {
        match self {
            ConstraintRule::PressureDropClearSky => "PRESSURE_DROP_CLEAR_SKY",
            ConstraintRule::LowHumidityPrecipitation => "LOW_HUMIDITY_PRECIPITATION",
            ConstraintRule::LowCloudStorm => "LOW_CLOUD_STORM",
            ConstraintRule::HighWindCalm => "HIGH_WIND_CALM",
            ConstraintRule::InstabilityCoherence => "INSTABILITY_COHERENCE",
            ConstraintRule::MutualIncoherence => "MUTUAL_INCOHERENCE",
        }
    }
}

impl std::fmt::Display for ConstraintRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id())
    }
}
