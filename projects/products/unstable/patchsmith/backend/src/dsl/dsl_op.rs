use serde::{Deserialize, Deserializer, Serialize};

fn deserialize_u64_from_number<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
    struct U64Visitor;
    impl<'de> serde::de::Visitor<'de> for U64Visitor {
        type Value = u64;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("a non-negative integer")
        }
        fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<u64, E> { Ok(v) }
        fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<u64, E> {
            u64::try_from(v).map_err(|_| E::custom("negative integer"))
        }
        fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<u64, E> {
            if v.fract() == 0.0 && v >= 0.0 && v <= u64::MAX as f64 {
                Ok(v as u64)
            } else {
                Err(E::custom(format!("cannot convert {v} to u64")))
            }
        }
    }
    deserializer.deserialize_any(U64Visitor)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op")]
pub enum DslOp {
    ReplaceRange {
        file: String,
        #[serde(deserialize_with = "deserialize_u64_from_number")]
        start: u64,
        #[serde(deserialize_with = "deserialize_u64_from_number")]
        end: u64,
        text: String,
    },
    ReplaceFirst {
        file: String,
        pattern: String,
        text: String,
    },
    InsertAfter {
        file: String,
        pattern: String,
        text: String,
    },
    DeleteRange {
        file: String,
        #[serde(deserialize_with = "deserialize_u64_from_number")]
        start: u64,
        #[serde(deserialize_with = "deserialize_u64_from_number")]
        end: u64,
    },
}
