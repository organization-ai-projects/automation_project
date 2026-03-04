#[derive(Debug, Clone)]
pub enum Action {
    LoadModel(String),
    ValidateModel,
    NewRun { seed: u64 },
    Step,
    RunToEnd,
    GetSnapshot,
    GetReport,
    SaveReplay,
    LoadReplay(String),
    ReplayToEnd,
    Quit,
}
