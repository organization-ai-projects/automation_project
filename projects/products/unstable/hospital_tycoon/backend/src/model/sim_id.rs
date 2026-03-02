// projects/products/unstable/hospital_tycoon/backend/src/model/sim_id.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SimId(pub u64);

impl SimId {
    pub fn new(v: u64) -> Self {
        Self(v)
    }
}
