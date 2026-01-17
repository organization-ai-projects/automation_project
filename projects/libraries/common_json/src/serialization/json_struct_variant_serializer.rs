// projects/libraries/common_json/src/serialization/json_struct_variant_serializer.rs
use crate::Json;
use crate::json_error::JsonError;
use crate::value::JsonMap;
use serde::ser::{Serialize, SerializeStructVariant};

use super::json_serializer::JsonSerializer;

pub(crate) struct JsonStructVariantSerializer {
    pub(crate) name: String,
    pub(crate) map: JsonMap,
}

impl SerializeStructVariant for JsonStructVariantSerializer {
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
        let mut wrapper = JsonMap::new();
        wrapper.insert(self.name, Json::Object(self.map));
        Ok(Json::Object(wrapper))
    }
}
