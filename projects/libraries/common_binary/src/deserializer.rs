// projects/libraries/common_binary/src/deserializer.rs
//! Binary deserializer implementation using serde

use crate::BinaryError;
use serde::de::{self, DeserializeSeed, Visitor};
use std::io::Read;

/// Binary deserializer that reads data in compact binary format
pub struct BinaryDeserializer<R> {
    reader: R,
}

impl<R: Read> BinaryDeserializer<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    fn read_u8(&mut self) -> Result<u8, BinaryError> {
        let mut buf = [0u8; 1];
        self.reader
            .read_exact(&mut buf)
            .map_err(|e| BinaryError::Decode(Box::leak(e.to_string().into_boxed_str())))?;
        Ok(buf[0])
    }

    fn read_bytes(&mut self, buf: &mut [u8]) -> Result<(), BinaryError> {
        self.reader
            .read_exact(buf)
            .map_err(|e| BinaryError::Decode(Box::leak(e.to_string().into_boxed_str())))
    }

    fn read_varint(&mut self) -> Result<u64, BinaryError> {
        let mut result = 0u64;
        let mut shift = 0;

        loop {
            let byte = self.read_u8()?;
            result |= ((byte & 0x7F) as u64) << shift;

            if byte & 0x80 == 0 {
                break;
            }

            shift += 7;
            if shift >= 64 {
                return Err(BinaryError::Decode("varint overflow"));
            }
        }

        Ok(result)
    }
}

impl<'de, R: Read> de::Deserializer<'de> for &mut BinaryDeserializer<R> {
    type Error = BinaryError;

    fn deserialize_any<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, BinaryError> {
        Err(BinaryError::Decode(
            "deserialize_any not supported in binary format",
        ))
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, BinaryError> {
        let v = self.read_u8()?;
        visitor.visit_bool(v != 0)
    }

    fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, BinaryError> {
        let v = self.read_u8()?;
        visitor.visit_i8(v as i8)
    }

    fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, BinaryError> {
        let mut buf = [0u8; 2];
        self.read_bytes(&mut buf)?;
        visitor.visit_i16(i16::from_le_bytes(buf))
    }

    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, BinaryError> {
        let mut buf = [0u8; 4];
        self.read_bytes(&mut buf)?;
        visitor.visit_i32(i32::from_le_bytes(buf))
    }

    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, BinaryError> {
        let mut buf = [0u8; 8];
        self.read_bytes(&mut buf)?;
        visitor.visit_i64(i64::from_le_bytes(buf))
    }

    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, BinaryError> {
        visitor.visit_u8(self.read_u8()?)
    }

    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, BinaryError> {
        let mut buf = [0u8; 2];
        self.read_bytes(&mut buf)?;
        visitor.visit_u16(u16::from_le_bytes(buf))
    }

    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, BinaryError> {
        let mut buf = [0u8; 4];
        self.read_bytes(&mut buf)?;
        visitor.visit_u32(u32::from_le_bytes(buf))
    }

    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, BinaryError> {
        let mut buf = [0u8; 8];
        self.read_bytes(&mut buf)?;
        visitor.visit_u64(u64::from_le_bytes(buf))
    }

    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, BinaryError> {
        let mut buf = [0u8; 4];
        self.read_bytes(&mut buf)?;
        visitor.visit_f32(f32::from_le_bytes(buf))
    }

    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, BinaryError> {
        let mut buf = [0u8; 8];
        self.read_bytes(&mut buf)?;
        visitor.visit_f64(f64::from_le_bytes(buf))
    }

    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, BinaryError> {
        let mut buf = [0u8; 4];
        self.read_bytes(&mut buf)?;
        let code = u32::from_le_bytes(buf);
        let c = char::from_u32(code).ok_or(BinaryError::Decode("invalid char code point"))?;
        visitor.visit_char(c)
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, BinaryError> {
        let len = self.read_varint()? as usize;
        let mut buf = vec![0u8; len];
        self.read_bytes(&mut buf)?;
        let s =
            std::str::from_utf8(&buf).map_err(|_| BinaryError::Decode("invalid UTF-8 string"))?;
        visitor.visit_str(s)
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, BinaryError> {
        let len = self.read_varint()? as usize;
        let mut buf = vec![0u8; len];
        self.read_bytes(&mut buf)?;
        let s = String::from_utf8(buf).map_err(|_| BinaryError::Decode("invalid UTF-8 string"))?;
        visitor.visit_string(s)
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, BinaryError> {
        let len = self.read_varint()? as usize;
        let mut buf = vec![0u8; len];
        self.read_bytes(&mut buf)?;
        visitor.visit_bytes(&buf)
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, BinaryError> {
        let len = self.read_varint()? as usize;
        let mut buf = vec![0u8; len];
        self.read_bytes(&mut buf)?;
        visitor.visit_byte_buf(buf)
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, BinaryError> {
        let tag = self.read_u8()?;
        if tag == 0 {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, BinaryError> {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, BinaryError> {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, BinaryError> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, BinaryError> {
        let len = self.read_varint()? as usize;
        visitor.visit_seq(SeqAccess::new(self, len))
    }

    fn deserialize_tuple<V: Visitor<'de>>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, BinaryError> {
        visitor.visit_seq(SeqAccess::new(self, len))
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, BinaryError> {
        visitor.visit_seq(SeqAccess::new(self, len))
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, BinaryError> {
        let len = self.read_varint()? as usize;
        visitor.visit_map(MapAccess::new(self, len))
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, BinaryError> {
        visitor.visit_seq(SeqAccess::new(self, fields.len()))
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, BinaryError> {
        visitor.visit_enum(EnumAccess::new(self))
    }

    fn deserialize_identifier<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, BinaryError> {
        Err(BinaryError::Decode(
            "deserialize_identifier not supported in binary format",
        ))
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, BinaryError> {
        self.deserialize_any(visitor)
    }
}

struct SeqAccess<'a, R> {
    de: &'a mut BinaryDeserializer<R>,
    remaining: usize,
}

impl<'a, R> SeqAccess<'a, R> {
    fn new(de: &'a mut BinaryDeserializer<R>, len: usize) -> Self {
        Self { de, remaining: len }
    }
}

impl<'de, 'a, R: Read> de::SeqAccess<'de> for SeqAccess<'a, R> {
    type Error = BinaryError;

    fn next_element_seed<T: DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, BinaryError> {
        if self.remaining == 0 {
            return Ok(None);
        }
        self.remaining -= 1;
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining)
    }
}

struct MapAccess<'a, R> {
    de: &'a mut BinaryDeserializer<R>,
    remaining: usize,
}

impl<'a, R> MapAccess<'a, R> {
    fn new(de: &'a mut BinaryDeserializer<R>, len: usize) -> Self {
        Self { de, remaining: len }
    }
}

impl<'de, 'a, R: Read> de::MapAccess<'de> for MapAccess<'a, R> {
    type Error = BinaryError;

    fn next_key_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, BinaryError> {
        if self.remaining == 0 {
            return Ok(None);
        }
        self.remaining -= 1;
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(
        &mut self,
        seed: V,
    ) -> Result<V::Value, BinaryError> {
        seed.deserialize(&mut *self.de)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining)
    }
}

struct EnumAccess<'a, R> {
    de: &'a mut BinaryDeserializer<R>,
}

impl<'a, R> EnumAccess<'a, R> {
    fn new(de: &'a mut BinaryDeserializer<R>) -> Self {
        Self { de }
    }
}

impl<'de, 'a, R: Read> de::EnumAccess<'de> for EnumAccess<'a, R> {
    type Error = BinaryError;
    type Variant = Self;

    fn variant_seed<V: DeserializeSeed<'de>>(
        self,
        seed: V,
    ) -> Result<(V::Value, Self::Variant), BinaryError> {
        let variant_index = self.de.read_varint()? as u32;
        use serde::de::value::U32Deserializer;
        let value = seed.deserialize(U32Deserializer::<BinaryError>::new(variant_index))?;
        Ok((value, self))
    }
}

impl<'de, 'a, R: Read> de::VariantAccess<'de> for EnumAccess<'a, R> {
    type Error = BinaryError;

    fn unit_variant(self) -> Result<(), BinaryError> {
        Ok(())
    }

    fn newtype_variant_seed<T: DeserializeSeed<'de>>(
        self,
        seed: T,
    ) -> Result<T::Value, BinaryError> {
        seed.deserialize(&mut *self.de)
    }

    fn tuple_variant<V: Visitor<'de>>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, BinaryError> {
        de::Deserializer::deserialize_tuple(&mut *self.de, len, visitor)
    }

    fn struct_variant<V: Visitor<'de>>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, BinaryError> {
        de::Deserializer::deserialize_struct(&mut *self.de, "", fields, visitor)
    }
}

/// Deserialize a value from binary format
pub fn deserialize_from_slice<'de, T: serde::Deserialize<'de>>(
    input: &'de [u8],
) -> Result<T, BinaryError> {
    let mut deserializer = BinaryDeserializer::new(input);
    T::deserialize(&mut deserializer)
}
