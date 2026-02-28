use super::TickReport;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RunHash {
    pub value: String,
}

impl RunHash {
    pub fn compute(scenario_name: &str, seed: u64, total_ticks: u64, tick_reports: &[TickReport]) -> Self {
        let mut parts = Vec::new();
        parts.push(format!("scenario:{scenario_name}"));
        parts.push(format!("seed:{seed}"));
        parts.push(format!("total_ticks:{total_ticks}"));
        for tr in tick_reports {
            parts.push(format!("tick:{}:bc{}:pop{}:bal{}:hash{}", tr.tick, tr.building_count, tr.total_population, tr.budget_balance, tr.snapshot_hash));
        }
        let canonical = parts.join("|");
        let mut hasher = Sha256::new();
        hasher.update(canonical.as_bytes());
        let result = hasher.finalize();
        Self { value: hex::encode(result) }
    }
}
