// projects/products/unstable/vod_forge/backend/src/protocol/serde_helpers.rs
use serde::de;

use crate::protocol::{U16Visitor, U32Visitor};

pub fn deser_u16<'de, D: de::Deserializer<'de>>(d: D) -> Result<u16, D::Error> {
    d.deserialize_any(U16Visitor)
}

pub fn deser_u32<'de, D: de::Deserializer<'de>>(d: D) -> Result<u32, D::Error> {
    d.deserialize_any(U32Visitor)
}
