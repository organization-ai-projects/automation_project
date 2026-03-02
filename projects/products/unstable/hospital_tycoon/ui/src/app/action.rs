// projects/products/unstable/hospital_tycoon/ui/src/app/action.rs

#[derive(Debug, Clone)]
pub enum Action {
    Step(u64),
    RunToEnd,
    GetReport,
    GetSnapshot,
    SaveReplay(String),
    LoadReplay(String),
    ReplayToEnd,
    Quit,
}
