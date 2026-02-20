// projects/libraries/common_json/src/serialization/json_struct_variant_serializer.rs
use crate::Json;
use crate::json_error::JsonError;
use crate::value::JsonMap;
use serde::ser::{Serialize, SerializeTupleVariant};

use super::json_serializer::JsonSerializer;

pub(crate) struct JsonTupleVariantSerializer {
    pub(crate) name: String,
    pub(crate) elements: Vec<Json>,
}

impl SerializeTupleVariant for JsonTupleVariantSerializer {
    type Ok = Json;
    type Error = JsonError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), JsonError> {
        self.elements.push(value.serialize(JsonSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Json, JsonError> {
        let mut map = JsonMap::new();
        map.insert(self.name, Json::Array(self.elements));
        Ok(Json::Object(map))
    }
}
