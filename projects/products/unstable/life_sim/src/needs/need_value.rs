use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NeedValue(pub u8);

impl NeedValue {
    pub fn new(v: u8) -> Self {
        Self(v.min(100))
    }

    pub fn saturating_add_i32(self, delta: i32) -> Self {
        let v = self.0 as i32 + delta;
        Self(v.clamp(0, 100) as u8)
    }
}

impl Add for NeedValue {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self((self.0.saturating_add(rhs.0)).min(100))
    }
}

impl Sub for NeedValue {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
    }
}
