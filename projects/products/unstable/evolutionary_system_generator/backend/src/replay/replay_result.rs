// projects/products/unstable/evolutionary_system_generator/backend/src/replay/replay_result.rs
use crate::output::candidate_manifest::CandidateManifest;

#[derive(Debug)]
pub struct ReplayResult {
    pub matches: bool,
    pub original_event_count: usize,
    pub replayed_event_count: usize,
    pub manifest: CandidateManifest,
}
