// projects/products/unstable/digital_pet/ui/src/app/action.rs

#[derive(Debug, Clone)]
pub enum Action {
    Step(u64),
    Feed,
    Rest,
    Play,
    Discipline,
    Medicine,
    Train(String),
    StartBattle,
    BattleStep,
    GetSnapshot,
    GetReport,
    SaveReplay(String),
    LoadReplay(String),
    ReplayToEnd,
    Quit,
}
