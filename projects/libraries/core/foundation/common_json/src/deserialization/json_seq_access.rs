// projects/libraries/common_json/src/deserialization/json_seq_access.rs
use crate::{Json, json_error::JsonError};
use serde::de::{DeserializeSeed, SeqAccess};

use super::json_deserializer::JsonDeserializer;

pub(crate) struct JsonSeqAccess<'de, I> {
    iter: I,
    marker: std::marker::PhantomData<&'de Json>,
}

impl<'de, I> JsonSeqAccess<'de, I>
where
    I: Iterator<Item = &'de Json>,
{
    pub(crate) fn new(iter: I) -> Self {
        Self {
            iter,
            marker: std::marker::PhantomData,
        }
    }
}

impl<'de, I> SeqAccess<'de> for JsonSeqAccess<'de, I>
where
    I: Iterator<Item = &'de Json>,
{
    type Error = JsonError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, JsonError>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(value) = self.iter.next() {
            seed.deserialize(JsonDeserializer::new(value)).map(Some)
        } else {
            Ok(None)
        }
    }
}
