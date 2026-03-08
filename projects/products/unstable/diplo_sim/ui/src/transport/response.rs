#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    Ok,
    Maps { map_ids: Vec<String> },
    MatchRun { run_id: u64 },
    RunStatus { run_id: u64, replayed: bool },
    ReplayReady { run_id: u64 },
    Error(String),
}
