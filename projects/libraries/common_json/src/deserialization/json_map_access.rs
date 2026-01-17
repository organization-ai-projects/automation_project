// projects/libraries/common_json/src/deserialization/json_map_access.rs
use crate::{Json, json_error::JsonError, json_error::JsonErrorCode};
use serde::de::{DeserializeSeed, IntoDeserializer, MapAccess};

use super::json_deserializer::JsonDeserializer;

pub(crate) struct JsonMapAccess<'de, I> {
    iter: I,
    value: Option<&'de Json>,
}

impl<'de, I> JsonMapAccess<'de, I>
where
    I: Iterator<Item = (&'de String, &'de Json)>,
{
    pub(crate) fn new(iter: I) -> Self {
        Self { iter, value: None }
    }
}

impl<'de, I> MapAccess<'de> for JsonMapAccess<'de, I>
where
    I: Iterator<Item = (&'de String, &'de Json)>,
{
    type Error = JsonError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, JsonError>
    where
        K: DeserializeSeed<'de>,
    {
        if let Some((key, value)) = self.iter.next() {
            self.value = Some(value);
            let key = key.as_str().into_deserializer();
            seed.deserialize(key).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, JsonError>
    where
        V: DeserializeSeed<'de>,
    {
        let value = self
            .value
            .take()
            .ok_or_else(|| JsonError::new(JsonErrorCode::ValueIsMissing))?;
        seed.deserialize(JsonDeserializer::new(value))
    }
}
