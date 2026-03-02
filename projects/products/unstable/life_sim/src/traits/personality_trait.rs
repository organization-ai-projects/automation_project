use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersonalityTrait {
    Outgoing,
    Shy,
    Ambitious,
    Lazy,
    Neat,
    Messy,
    Playful,
    Serious,
}
