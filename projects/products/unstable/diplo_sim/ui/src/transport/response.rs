#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    Ok,
    Maps {
        map_ids: Vec<String>,
    },
    MapInfo {
        map_id: String,
        territory_count: u32,
        adjacency_count: u32,
        starting_unit_count: u32,
    },
    MatchRun {
        run_id: u64,
    },
    RunStatus {
        run_id: u64,
        replayed: bool,
    },
    ReplayReady {
        run_id: u64,
    },
    Error(String),
}
