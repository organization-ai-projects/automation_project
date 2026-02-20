// projects/libraries/common_binary/src/serializer.rs
//! Binary serializer implementation using serde

use crate::BinaryError;
use serde::ser::{self, Serialize};
use std::io::Write;

/// Binary serializer that writes data in a compact binary format
pub struct BinarySerializer<W> {
    writer: W,
}

impl<W: Write> BinarySerializer<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    fn write_u8(&mut self, v: u8) -> Result<(), BinaryError> {
        self.writer
            .write_all(&[v])
            .map_err(|e| BinaryError::Encode(Box::leak(e.to_string().into_boxed_str())))
    }

    fn write_bytes(&mut self, v: &[u8]) -> Result<(), BinaryError> {
        self.writer
            .write_all(v)
            .map_err(|e| BinaryError::Encode(Box::leak(e.to_string().into_boxed_str())))
    }

    fn write_varint(&mut self, mut v: u64) -> Result<(), BinaryError> {
        // Variable-length integer encoding (LEB128-style)
        loop {
            let mut byte = (v & 0x7F) as u8;
            v >>= 7;
            if v != 0 {
                byte |= 0x80;
            }
            self.write_u8(byte)?;
            if v == 0 {
                break;
            }
        }
        Ok(())
    }
}

impl<W: Write> ser::Serializer for &mut BinarySerializer<W> {
    type Ok = ();
    type Error = BinaryError;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<(), BinaryError> {
        self.write_u8(if v { 1 } else { 0 })
    }

    fn serialize_i8(self, v: i8) -> Result<(), BinaryError> {
        self.write_u8(v as u8)
    }

    fn serialize_i16(self, v: i16) -> Result<(), BinaryError> {
        self.write_bytes(&v.to_le_bytes())
    }

    fn serialize_i32(self, v: i32) -> Result<(), BinaryError> {
        self.write_bytes(&v.to_le_bytes())
    }

    fn serialize_i64(self, v: i64) -> Result<(), BinaryError> {
        self.write_bytes(&v.to_le_bytes())
    }

    fn serialize_u8(self, v: u8) -> Result<(), BinaryError> {
        self.write_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<(), BinaryError> {
        self.write_bytes(&v.to_le_bytes())
    }

    fn serialize_u32(self, v: u32) -> Result<(), BinaryError> {
        self.write_bytes(&v.to_le_bytes())
    }

    fn serialize_u64(self, v: u64) -> Result<(), BinaryError> {
        self.write_bytes(&v.to_le_bytes())
    }

    fn serialize_f32(self, v: f32) -> Result<(), BinaryError> {
        self.write_bytes(&v.to_le_bytes())
    }

    fn serialize_f64(self, v: f64) -> Result<(), BinaryError> {
        self.write_bytes(&v.to_le_bytes())
    }

    fn serialize_char(self, v: char) -> Result<(), BinaryError> {
        self.serialize_u32(v as u32)
    }

    fn serialize_str(self, v: &str) -> Result<(), BinaryError> {
        self.write_varint(v.len() as u64)?;
        self.write_bytes(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<(), BinaryError> {
        self.write_varint(v.len() as u64)?;
        self.write_bytes(v)
    }

    fn serialize_none(self) -> Result<(), BinaryError> {
        self.write_u8(0)
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<(), BinaryError> {
        self.write_u8(1)?;
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<(), BinaryError> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), BinaryError> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<(), BinaryError> {
        self.write_varint(variant_index as u64)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<(), BinaryError> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<(), BinaryError> {
        self.write_varint(variant_index as u64)?;
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, BinaryError> {
        let len = len.ok_or(BinaryError::Encode("sequence length required"))?;
        self.write_varint(len as u64)?;
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, BinaryError> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, BinaryError> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, BinaryError> {
        self.write_varint(variant_index as u64)?;
        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, BinaryError> {
        let len = len.ok_or(BinaryError::Encode("map length required"))?;
        self.write_varint(len as u64)?;
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, BinaryError> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, BinaryError> {
        self.write_varint(variant_index as u64)?;
        Ok(self)
    }
}

impl<W: Write> ser::SerializeSeq for &mut BinarySerializer<W> {
    type Ok = ();
    type Error = BinaryError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), BinaryError> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), BinaryError> {
        Ok(())
    }
}

impl<W: Write> ser::SerializeTuple for &mut BinarySerializer<W> {
    type Ok = ();
    type Error = BinaryError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), BinaryError> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), BinaryError> {
        Ok(())
    }
}

impl<W: Write> ser::SerializeTupleStruct for &mut BinarySerializer<W> {
    type Ok = ();
    type Error = BinaryError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), BinaryError> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), BinaryError> {
        Ok(())
    }
}

impl<W: Write> ser::SerializeTupleVariant for &mut BinarySerializer<W> {
    type Ok = ();
    type Error = BinaryError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), BinaryError> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), BinaryError> {
        Ok(())
    }
}

impl<W: Write> ser::SerializeMap for &mut BinarySerializer<W> {
    type Ok = ();
    type Error = BinaryError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), BinaryError> {
        key.serialize(&mut **self)
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), BinaryError> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), BinaryError> {
        Ok(())
    }
}

impl<W: Write> ser::SerializeStruct for &mut BinarySerializer<W> {
    type Ok = ();
    type Error = BinaryError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), BinaryError> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), BinaryError> {
        Ok(())
    }
}

impl<W: Write> ser::SerializeStructVariant for &mut BinarySerializer<W> {
    type Ok = ();
    type Error = BinaryError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), BinaryError> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), BinaryError> {
        Ok(())
    }
}

/// Serialize a value to binary format
pub fn serialize_to_vec<T: Serialize>(value: &T) -> Result<Vec<u8>, BinaryError> {
    let mut buf = Vec::new();
    let mut serializer = BinarySerializer::new(&mut buf);
    value.serialize(&mut serializer)?;
    Ok(buf)
}
