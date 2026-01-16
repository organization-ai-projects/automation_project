use crate::{Json, error::JsonError};
use serde::ser::{Serialize, SerializeSeq, SerializeTuple, SerializeTupleStruct};

use super::json_serializer::JsonSerializer;

pub(crate) struct JsonSeqSerializer {
    pub(crate) elements: Vec<Json>,
}

impl SerializeSeq for JsonSeqSerializer {
    type Ok = Json;
    type Error = JsonError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), JsonError> {
        self.elements.push(value.serialize(JsonSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Json, JsonError> {
        Ok(Json::Array(self.elements))
    }
}

impl SerializeTuple for JsonSeqSerializer {
    type Ok = Json;
    type Error = JsonError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), JsonError> {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Json, JsonError> {
        SerializeSeq::end(self)
    }
}

impl SerializeTupleStruct for JsonSeqSerializer {
    type Ok = Json;
    type Error = JsonError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), JsonError> {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Json, JsonError> {
        SerializeSeq::end(self)
    }
}
