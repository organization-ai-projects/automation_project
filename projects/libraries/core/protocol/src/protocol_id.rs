// projects/libraries/protocol/src/protocol_id.rs
use common::Id128;
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProtocolId(Id128);

impl ProtocolId {
    pub fn new(id: Id128) -> Self {
        Self(id)
    }

    pub fn to_hex(&self) -> String {
        self.0.to_hex()
    }

    pub fn as_inner(&self) -> Id128 {
        self.0
    }
}

impl Default for ProtocolId {
    fn default() -> Self {
        Self(Id128::new(0, Some(0), Some(0)))
    }
}

impl FromStr for ProtocolId {
    type Err = <Id128 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Id128::from_str(s).map(Self)
    }
}

impl fmt::Display for ProtocolId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for ProtocolId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for ProtocolId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProtocolIdVisitor;

        impl<'de> Visitor<'de> for ProtocolIdVisitor {
            type Value = ProtocolId;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a 32-char hex string or a 16-byte array")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                ProtocolId::from_str(v).map_err(serde::de::Error::custom)
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_str(&v)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut bytes = [0u8; 16];
                for (i, byte) in bytes.iter_mut().enumerate() {
                    *byte = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(i, &self))?;
                }
                if seq.next_element::<u8>()?.is_some() {
                    return Err(serde::de::Error::invalid_length(17, &self));
                }
                Ok(ProtocolId::new(Id128::from_bytes_unchecked(bytes)))
            }
        }

        deserializer.deserialize_any(ProtocolIdVisitor)
    }
}
