use crate::needs::NeedKind;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionEffect {
    pub need_deltas: BTreeMap<NeedKind, i32>,
}
