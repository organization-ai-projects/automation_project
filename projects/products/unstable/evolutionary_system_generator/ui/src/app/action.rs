use evolutionary_system_generator_backend::public_api::Constraint;

#[derive(Debug, Clone)]
pub enum Action {
    StartSearch {
        seed: u64,
        population_size: usize,
        max_generations: u32,
        rule_pool: Vec<String>,
        constraints: Vec<Constraint>,
    },
    StepGen,
    RunToEnd,
    ShowCandidates { top_n: usize },
    Quit,
}
