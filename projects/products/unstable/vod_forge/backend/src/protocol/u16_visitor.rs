// projects/products/unstable/vod_forge/backend/src/protocol/u16_visitor.rs
use serde::de;
use std::fmt;

pub struct U16Visitor;

impl<'de> de::Visitor<'de> for U16Visitor {
    type Value = u16;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "u16")
    }

    fn visit_u8<E: de::Error>(self, v: u8) -> Result<u16, E> {
        Ok(v as u16)
    }

    fn visit_u16<E: de::Error>(self, v: u16) -> Result<u16, E> {
        Ok(v)
    }

    fn visit_u32<E: de::Error>(self, v: u32) -> Result<u16, E> {
        u16::try_from(v).map_err(|_| E::custom("value out of range for u16"))
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<u16, E> {
        u16::try_from(v).map_err(|_| E::custom("value out of range for u16"))
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<u16, E> {
        u16::try_from(v).map_err(|_| E::custom("value out of range for u16"))
    }

    fn visit_f64<E: de::Error>(self, v: f64) -> Result<u16, E> {
        if v.fract() == 0.0 && v >= 0.0 && v <= u16::MAX as f64 {
            Ok(v as u16)
        } else {
            Err(E::custom("not a valid u16"))
        }
    }
}
