use serde::{Deserialize, Serialize};

/// Ordered float wrapper - NaN treated as 0.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct UtilityScore(pub f64);

impl UtilityScore {
    fn key(self) -> i64 {
        let v = if self.0.is_nan() { 0.0 } else { self.0 };
        v.to_bits() as i64
    }
}

impl PartialEq for UtilityScore {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl Eq for UtilityScore {}

impl PartialOrd for UtilityScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UtilityScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = if self.0.is_nan() { 0.0 } else { self.0 };
        let b = if other.0.is_nan() { 0.0 } else { other.0 };
        a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)
    }
}
