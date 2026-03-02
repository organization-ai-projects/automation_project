#[derive(Debug, Clone)]
pub enum Action {
    StartSearch {
        seed: u64,
        population_size: usize,
        max_generations: u32,
        rule_pool: Vec<String>,
    },
    StepGen,
    RunToEnd,
    ShowCandidates {
        top_n: usize,
    },
    Quit,
}
