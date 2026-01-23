// projects/libraries/common_json/src/deserialization/json_enum_access.rs
use crate::{Json, json_error::JsonError};
use serde::de::{self, DeserializeSeed, EnumAccess, IntoDeserializer};

use super::json_variant_access::JsonVariantAccess;

pub(crate) struct JsonEnumAccess<'de> {
    variant: &'de str,
    value: Option<&'de Json>,
}

impl<'de> JsonEnumAccess<'de> {
    pub(crate) fn new(variant: &'de str, value: Option<&'de Json>) -> Self {
        Self { variant, value }
    }
}

impl<'de> EnumAccess<'de> for JsonEnumAccess<'de> {
    type Error = JsonError;
    type Variant = JsonVariantAccess<'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), JsonError>
    where
        V: DeserializeSeed<'de>,
    {
        let deserializer: de::value::StrDeserializer<'_, JsonError> =
            self.variant.into_deserializer();
        let value = seed.deserialize(deserializer)?;
        Ok((value, JsonVariantAccess { value: self.value }))
    }
}
