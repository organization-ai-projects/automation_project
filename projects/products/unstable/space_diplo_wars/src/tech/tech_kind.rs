use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TechKind {
    ShipDrive,
    Weapons,
    Shields,
    Economics,
    Diplomacy,
}
