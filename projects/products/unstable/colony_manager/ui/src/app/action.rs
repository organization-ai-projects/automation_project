// projects/products/unstable/colony_manager/ui/src/app/action.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    RunRequested,
    RunCompleted,
    ReplayRequested,
    ReplayCompleted,
}
