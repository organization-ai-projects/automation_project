#![allow(dead_code)]
use crate::commands::command_id::CommandId;
use crate::commands::command_kind::CommandKind;
use crate::time::tick::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub id: CommandId,
    pub kind: CommandKind,
    pub at_tick: Tick,
    pub payload: serde_json::Value,
}
