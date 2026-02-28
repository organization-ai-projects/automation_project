use crate::needs::need_kind::NeedKind;
use crate::needs::need_value::NeedValue;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeedsState {
    pub values: BTreeMap<NeedKind, NeedValue>,
}

impl NeedsState {
    pub fn new_full() -> Self {
        let mut values = BTreeMap::new();
        for kind in [
            NeedKind::Hunger,
            NeedKind::Energy,
            NeedKind::Social,
            NeedKind::Fun,
            NeedKind::Hygiene,
            NeedKind::Bladder,
            NeedKind::Comfort,
        ] {
            values.insert(kind, NeedValue::new(80));
        }
        Self { values }
    }

    pub fn get(&self, kind: NeedKind) -> NeedValue {
        self.values.get(&kind).copied().unwrap_or(NeedValue(0))
    }

    pub fn set(&mut self, kind: NeedKind, value: NeedValue) {
        self.values.insert(kind, NeedValue::new(value.0));
    }

    /// Each need decays by 1 per tick (capped at 0).
    pub fn decay_tick(&mut self) {
        for v in self.values.values_mut() {
            *v = NeedValue(v.0.saturating_sub(1));
        }
    }
}
