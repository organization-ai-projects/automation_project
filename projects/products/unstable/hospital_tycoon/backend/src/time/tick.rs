// projects/products/unstable/hospital_tycoon/backend/src/time/tick.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Tick(pub u64);

impl Tick {
    pub fn zero() -> Self {
        Self(0)
    }
    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }
    pub fn value(self) -> u64 {
        self.0
    }
}
