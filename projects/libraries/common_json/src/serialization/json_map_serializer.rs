use crate::Json;
use crate::error::JsonError;
use crate::value::JsonMap;
use serde::ser::{Serialize, SerializeMap, SerializeStruct};

use super::json_serializer::JsonSerializer;
use super::key_serializer::KeySerializer;

pub(crate) struct JsonMapSerializer {
    map: JsonMap,
    next_key: Option<String>,
}

impl JsonMapSerializer {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            map: JsonMap::with_capacity(capacity),
            next_key: None,
        }
    }
}

impl SerializeMap for JsonMapSerializer {
    type Ok = Json;
    type Error = JsonError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), JsonError> {
        let key = key.serialize(KeySerializer)?;
        self.next_key = Some(key);
        Ok(())
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), JsonError> {
        let key = self
            .next_key
            .take()
            .ok_or_else(|| JsonError::custom("value serialized before key"))?;
        let value = value.serialize(JsonSerializer)?;
        self.map.insert(key, value);
        Ok(())
    }

    fn end(self) -> Result<Json, JsonError> {
        Ok(Json::Object(self.map))
    }
}

impl SerializeStruct for JsonMapSerializer {
    type Ok = Json;
    type Error = JsonError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), JsonError> {
        self.map
            .insert(key.to_string(), value.serialize(JsonSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Json, JsonError> {
        Ok(Json::Object(self.map))
    }
}
