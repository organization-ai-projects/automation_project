use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Turn {
    pub number: u32,
}

impl Turn {
    pub fn new(number: u32) -> Self {
        Self { number }
    }

    pub fn next(self) -> Self {
        Self { number: self.number + 1 }
    }
}

impl std::fmt::Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Turn({})", self.number)
    }
}
