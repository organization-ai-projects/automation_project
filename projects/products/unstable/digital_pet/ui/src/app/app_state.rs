// projects/products/unstable/digital_pet/ui/src/app/app_state.rs
use crate::transport::run_report_dto::RunReportDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub seed: u64,
    pub ticks: u64,
    pub current_tick: u64,
    pub running: bool,
    pub pet_name: String,
    pub species: String,
    pub evolution_stage: u32,
    pub hp: u32,
    pub max_hp: u32,
    pub hunger: u32,
    pub fatigue: u32,
    pub happiness: u32,
    pub discipline: u32,
    pub sick: bool,
    pub last_event: Option<String>,
}

impl AppState {
    pub fn new(seed: u64, ticks: u64) -> Self {
        Self {
            seed,
            ticks,
            current_tick: 0,
            running: false,
            pet_name: String::new(),
            species: String::new(),
            evolution_stage: 0,
            hp: 0,
            max_hp: 0,
            hunger: 0,
            fatigue: 0,
            happiness: 0,
            discipline: 0,
            sick: false,
            last_event: None,
        }
    }

    pub fn from_report(report: &RunReportDto) -> Self {
        Self {
            seed: report.seed,
            ticks: report.total_ticks,
            current_tick: report.total_ticks,
            running: false,
            pet_name: String::new(),
            species: report.final_species.clone(),
            evolution_stage: report.evolution_stage,
            hp: report.final_hp,
            max_hp: report.final_hp,
            hunger: 0,
            fatigue: 0,
            happiness: report.final_happiness,
            discipline: report.final_discipline,
            sick: false,
            last_event: Some(format!("report_hash: {}", report.run_hash)),
        }
    }
}
