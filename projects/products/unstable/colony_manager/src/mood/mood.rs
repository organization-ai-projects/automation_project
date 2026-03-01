use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mood {
    pub value: f32,
}

impl Default for Mood {
    fn default() -> Self {
        Self { value: 0.5 }
    }
}

impl Mood {
    pub fn update_from_needs(&mut self, needs_avg: f32) {
        let target = needs_avg;
        self.value = (self.value * 0.8 + target * 0.2).clamp(0.0, 1.0);
    }
    pub fn apply_modifier(&mut self, delta: f32) {
        self.value = (self.value + delta).clamp(0.0, 1.0);
    }
}
