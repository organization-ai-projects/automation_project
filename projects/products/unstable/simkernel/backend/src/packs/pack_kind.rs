#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PackKind {
    Hospital,
    ThemePark,
    MonsterCatcher,
    DigitalPet,
    Custom(String),
}

impl PackKind {
    pub fn as_str(&self) -> &str {
        match self {
            PackKind::Hospital => "hospital",
            PackKind::ThemePark => "theme_park",
            PackKind::MonsterCatcher => "monster_catcher",
            PackKind::DigitalPet => "digital_pet",
            PackKind::Custom(s) => s.as_str(),
        }
    }
}
