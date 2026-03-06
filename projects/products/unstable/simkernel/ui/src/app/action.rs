// projects/products/unstable/simkernel/ui/src/app/action.rs
#[derive(Debug, Clone)]
pub enum Action {
    SelectPack(String),
    SetSeed(u64),
    SetTicks(u64),
    StartRun,
    Shutdown,
}
