use protocol::ProtocolId;
use std::str::FromStr;

use crate::moe_core::{ExpertId, TaskId};

pub(crate) fn protocol_id(tag: u8) -> ProtocolId {
    ProtocolId::from_str(&format!("{:032x}", tag.max(1)))
        .expect("test protocol id should be valid fixed hex")
}

pub(crate) fn task_id(tag: u8) -> TaskId {
    TaskId::from_protocol_id(protocol_id(tag))
}

pub(crate) fn expert_id(tag: u8) -> ExpertId {
    ExpertId::from_protocol_id(protocol_id(tag))
}
