use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RuntimeId(u64);

impl RuntimeId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for RuntimeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RuntimeId({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stores_value() {
        let id = RuntimeId::new(42);
        assert_eq!(id.value(), 42);
    }

    #[test]
    fn equality_holds() {
        let a = RuntimeId::new(1);
        let b = RuntimeId::new(1);
        assert_eq!(a, b);
    }

    #[test]
    fn ordering_holds() {
        let a = RuntimeId::new(1);
        let b = RuntimeId::new(2);
        assert!(a < b);
    }

    #[test]
    fn display_formats_correctly() {
        let id = RuntimeId::new(7);
        assert_eq!(id.to_string(), "RuntimeId(7)");
    }
}
