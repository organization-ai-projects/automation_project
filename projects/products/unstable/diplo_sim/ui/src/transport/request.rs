#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request {
    Health,
    ListMaps,
    RunMatch {
        map_id: String,
        turns: u32,
        seed: u64,
        players: u32,
    },
    GetRunStatus {
        run_id: u64,
    },
    ReplayMatch {
        run_id: u64,
    },
}
