use serde::de;
use std::fmt;

struct U16Visitor;
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

pub fn deser_u16<'de, D: de::Deserializer<'de>>(d: D) -> Result<u16, D::Error> {
    d.deserialize_any(U16Visitor)
}

struct U32Visitor;
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

pub fn deser_u32<'de, D: de::Deserializer<'de>>(d: D) -> Result<u32, D::Error> {
    d.deserialize_any(U32Visitor)
}
