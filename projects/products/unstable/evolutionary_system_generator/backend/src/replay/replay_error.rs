// projects/products/unstable/evolutionary_system_generator/backend/src/replay/replay_error.rs
#[derive(Debug, thiserror::Error)]
pub enum ReplayError {
    #[error("SearchStarted event not found in log")]
    NoSearchStartedEvent,
    #[error("Event count mismatch: original {0}, replayed {1}")]
    EventCountMismatch(usize, usize),
}
