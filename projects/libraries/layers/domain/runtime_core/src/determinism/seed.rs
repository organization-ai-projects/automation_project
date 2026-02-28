use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Seed(u64);

impl Seed {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stores_value() {
        let s = Seed::new(99);
        assert_eq!(s.value(), 99);
    }

    #[test]
    fn default_is_zero() {
        let s = Seed::default();
        assert_eq!(s.value(), 0);
    }
}
