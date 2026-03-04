use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunReportDto {
    pub seed: u64,
    pub final_species: String,
    pub evolution_stage: u32,
    pub total_ticks: u64,
    pub care_mistakes: usize,
    pub final_happiness: u32,
    pub final_discipline: u32,
    pub final_hp: u32,
    pub event_count: usize,
    pub run_hash: String,
}
