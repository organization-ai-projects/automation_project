// projects/libraries/common_json/src/json_number.rs
use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use crate::json_number_visitor::JsonNumberVisitor;

/// JSON number.
#[derive(Debug, Clone, PartialEq)]
pub struct JsonNumber {
    pub(crate) value: f64,
}

impl Serialize for JsonNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as integer if it has no fractional part
        if self.value.fract() == 0.0 && self.value.abs() < (i64::MAX as f64) {
            serializer.serialize_i64(self.value as i64)
        } else {
            serializer.serialize_f64(self.value)
        }
    }
}

impl<'de> Deserialize<'de> for JsonNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(JsonNumberVisitor)
    }
}

impl JsonNumber {
    pub fn from_f64(n: f64) -> Option<Self> {
        if n.is_finite() {
            Some(JsonNumber { value: n })
        } else {
            None
        }
    }

    pub fn as_f64(&self) -> f64 {
        self.value
    }

    pub fn is_non_zero(&self) -> bool {
        self.value != 0.0
    }
}

impl From<i64> for JsonNumber {
    fn from(value: i64) -> Self {
        JsonNumber {
            value: value as f64,
        }
    }
}

impl From<u64> for JsonNumber {
    fn from(value: u64) -> Self {
        JsonNumber {
            value: value as f64,
        }
    }
}
