// projects/products/unstable/autonomy_orchestrator_ai/src/cli_command/cli_risk_tier.rs
use crate::domain::RiskTier;
use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum CliRiskTier {
    Low,
    Medium,
    High,
}

impl From<CliRiskTier> for RiskTier {
    fn from(value: CliRiskTier) -> Self {
        match value {
            CliRiskTier::Low => Self::Low,
            CliRiskTier::Medium => Self::Medium,
            CliRiskTier::High => Self::High,
        }
    }
}
