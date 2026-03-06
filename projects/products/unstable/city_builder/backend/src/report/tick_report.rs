#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TickReport {
    pub tick: u64,
    pub building_count: u32,
    pub total_population: u64,
    pub budget_balance: i64,
    pub snapshot_hash: String,
}
