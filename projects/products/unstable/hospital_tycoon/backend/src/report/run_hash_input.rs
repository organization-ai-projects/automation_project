// projects/products/unstable/hospital_tycoon/backend/src/report/run_hash_input.rs
pub struct RunHashInput<'a> {
    pub seed: u64,
    pub scenario_name: &'a str,
    pub total_ticks: u64,
    pub patients_treated: u32,
    pub patients_died: u32,
    pub final_budget: i64,
    pub final_reputation: u32,
    pub event_count: usize,
}
