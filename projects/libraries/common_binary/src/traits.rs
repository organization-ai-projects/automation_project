use crate::BinaryError;

/// Trait for types that can be encoded to binary format.
pub trait BinaryEncode {
    /// Encode this value to binary format, appending to the provided buffer.
    fn encode_binary(&self, out: &mut Vec<u8>) -> Result<(), BinaryError>;
}

/// Trait for types that can be decoded from binary format.
pub trait BinaryDecode: Sized {
    /// Decode a value from binary format.
    fn decode_binary(input: &[u8]) -> Result<Self, BinaryError>;
}
