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
