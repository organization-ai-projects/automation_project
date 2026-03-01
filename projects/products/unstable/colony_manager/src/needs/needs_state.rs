use crate::needs::need_kind::NeedKind;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeedsState {
    pub levels: BTreeMap<NeedKind, f32>,
}

impl Default for NeedsState {
    fn default() -> Self {
        let mut levels = BTreeMap::new();
        levels.insert(NeedKind::Food, 1.0);
        levels.insert(NeedKind::Rest, 1.0);
        levels.insert(NeedKind::Social, 0.8);
        levels.insert(NeedKind::Safety, 1.0);
        Self { levels }
    }
}

impl NeedsState {
    pub fn decay(&mut self, rate: f32) {
        for v in self.levels.values_mut() {
            *v = (*v - rate).max(0.0);
        }
    }
    pub fn average(&self) -> f32 {
        if self.levels.is_empty() {
            return 1.0;
        }
        self.levels.values().sum::<f32>() / self.levels.len() as f32
    }
}
