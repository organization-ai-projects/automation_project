// projects/libraries/common/src/common_id.rs
//control file no use it for ID128
use crate::custom_uuid::Id128;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommonID(Id128);

impl CommonID {
    pub fn new(id: Id128) -> Self {
        Self(id)
    }

    pub fn is_valid(id: Id128) -> bool {
        id.timestamp_ms() > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::custom_uuid::Id128;

    #[test]
    fn test_is_valid_with_zero_bytes() {
        let zero_id = Id128::from_bytes_unchecked([0u8; 16]);
        assert!(!CommonID::is_valid(zero_id));
    }
}
