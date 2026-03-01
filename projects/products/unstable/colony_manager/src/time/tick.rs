use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Tick(pub u64);

impl Tick {
    pub fn advance(&mut self) { self.0 += 1; }
    pub fn value(self) -> u64 { self.0 }
}
