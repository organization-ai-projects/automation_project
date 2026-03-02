#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandKind {
    Spawn,
    Despawn,
    SetComponent,
    RemoveComponent,
    Custom(String),
}
