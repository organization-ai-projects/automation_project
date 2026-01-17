// projects/libraries/common_json/src/serialization/json_seq_serializer.rs
use crate::{Json, json_error::JsonError};
use serde::ser::{Serialize, SerializeSeq, SerializeTuple, SerializeTupleStruct};

use super::json_serializer::JsonSerializer;

pub(crate) struct JsonSeqSerializer {
    pub(crate) elements: Vec<Json>,
}

// Common type aliases for consistency and to avoid repetition.
pub(crate) type SerializeResult = Result<Json, JsonError>;

impl SerializeSeq for JsonSeqSerializer {
    type Ok = Json;
    type Error = JsonError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), JsonError> {
        self.elements.push(value.serialize(JsonSerializer)?);
        Ok(())
    }

    fn end(self) -> SerializeResult {
        Ok(Json::Array(self.elements))
    }
}

impl SerializeTuple for JsonSeqSerializer {
    type Ok = Json;
    type Error = JsonError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), JsonError> {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> SerializeResult {
        SerializeSeq::end(self)
    }
}

impl SerializeTupleStruct for JsonSeqSerializer {
    type Ok = Json;
    type Error = JsonError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), JsonError> {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> SerializeResult {
        SerializeSeq::end(self)
    }
}
