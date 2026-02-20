use crate::BinaryError;
use crate::deserializer::deserialize_from_slice;
use crate::serializer::serialize_to_vec;
use serde::{Deserialize, Serialize};

/// Trait for types that can be encoded to binary format.
///
/// Automatically implemented for any type `T: Serialize` through the blanket implementation.
/// You do not need to implement this trait manually.
pub trait BinaryEncode {
    /// Encode this value to binary format, appending to the provided buffer.
    fn encode_binary(&self, out: &mut Vec<u8>) -> Result<(), BinaryError>;
}

/// Trait for types that can be decoded from binary format.
///
/// Automatically implemented for any type `T: Deserialize` through the blanket implementation.
/// You do not need to implement this trait manually.
pub trait BinaryDecode: Sized {
    /// Decode a value from binary format.
    fn decode_binary(input: &[u8]) -> Result<Self, BinaryError>;
}

// Blanket implementation for any type that implements Serialize
impl<T: Serialize> BinaryEncode for T {
    fn encode_binary(&self, out: &mut Vec<u8>) -> Result<(), BinaryError> {
        let encoded = serialize_to_vec(self)?;
        out.extend_from_slice(&encoded);
        Ok(())
    }
}

// Blanket implementation for any type that implements Deserialize
impl<T: for<'de> Deserialize<'de>> BinaryDecode for T {
    fn decode_binary(input: &[u8]) -> Result<Self, BinaryError> {
        deserialize_from_slice(input)
    }
}
