use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StatusEffect {
    pub name: String,
    pub remaining_turns: u32,
}

impl StatusEffect {
    pub fn new(name: &str, duration: u32) -> Self {
        Self {
            name: name.to_string(),
            remaining_turns: duration,
        }
    }

    pub fn tick(&mut self) -> bool {
        if self.remaining_turns > 0 {
            self.remaining_turns -= 1;
        }
        self.remaining_turns == 0
    }
}
