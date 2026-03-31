use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Price(i64);

impl Price {
    pub fn new(cents: i64) -> Self {
        Self(cents)
    }

    pub fn cents(self) -> i64 {
        self.0
    }
}

impl std::fmt::Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dollars = self.0 / 100;
        let cents = (self.0 % 100).abs();
        write!(f, "${dollars}.{cents:02}")
    }
}
