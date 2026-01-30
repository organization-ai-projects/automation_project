// projects/libraries/common_json/src/json_number_visitor.rs
use serde::de::{self, Visitor};

use crate::json_number::JsonNumber;

pub(crate) struct JsonNumberVisitor;

impl<'de> Visitor<'de> for JsonNumberVisitor {
    type Value = JsonNumber;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a JSON number")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JsonNumber::from(v))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JsonNumber::from(v))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        JsonNumber::from_f64(v)
            .ok_or_else(|| de::Error::custom("invalid float value: must be finite"))
    }
}
