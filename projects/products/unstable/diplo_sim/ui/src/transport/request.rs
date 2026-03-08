#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request {
    Health,
    RunMatch {
        map_id: String,
        turns: u32,
        seed: u64,
        players: u32,
    },
    ReplayMatch {
        run_id: u64,
    },
}
