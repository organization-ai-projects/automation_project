// projects/products/unstable/vod_forge/backend/src/protocol/u16_visitor.rs
use serde::de;
use std::fmt;

pub struct U32Visitor;

impl<'de> de::Visitor<'de> for U32Visitor {
    type Value = u32;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "u32")
    }

    fn visit_u8<E: de::Error>(self, v: u8) -> Result<u32, E> {
        Ok(v as u32)
    }

    fn visit_u16<E: de::Error>(self, v: u16) -> Result<u32, E> {
        Ok(v as u32)
    }

    fn visit_u32<E: de::Error>(self, v: u32) -> Result<u32, E> {
        Ok(v)
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<u32, E> {
        u32::try_from(v).map_err(|_| E::custom("value out of range for u32"))
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<u32, E> {
        u32::try_from(v).map_err(|_| E::custom("value out of range for u32"))
    }

    fn visit_f64<E: de::Error>(self, v: f64) -> Result<u32, E> {
        if v.fract() == 0.0 && v >= 0.0 && v <= u32::MAX as f64 {
            Ok(v as u32)
        } else {
            Err(E::custom("not a valid u32"))
        }
    }
}
