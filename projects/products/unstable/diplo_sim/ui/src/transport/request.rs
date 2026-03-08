#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request {
    Health,
    RunMatch,
    ReplayMatch,
}
