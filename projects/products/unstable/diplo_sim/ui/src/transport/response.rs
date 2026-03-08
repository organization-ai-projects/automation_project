#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    Ok,
    MatchRun { run_id: u64 },
    ReplayReady { run_id: u64 },
    Error(String),
}
