use crate::output::candidate_manifest::CandidateManifest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Response {
    Ok,
    Error {
        message: String,
    },
    Candidates {
        manifest: CandidateManifest,
    },
    Report {
        generation: u32,
        best_fitness: f64,
        population_size: usize,
        done: bool,
    },
}
