// projects/libraries/protocol/src/protocol_id.rs
use common::Id128;
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
        let s = String::deserialize(deserializer)?;
        ProtocolId::from_str(&s).map_err(serde::de::Error::custom)
    }
}
