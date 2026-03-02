use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ActionKind {
    Eat,
    Sleep,
    SocialChat,
    UseObject,
    Work,
    Relax,
    UseBathroom,
    Move,
}
