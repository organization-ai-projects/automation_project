use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub(crate) struct Tick(pub(crate) u64);

impl Tick {
    pub(crate) fn advance(&mut self) {
        self.0 += 1;
    }

    pub(crate) fn value(self) -> u64 {
        self.0
    }
}
