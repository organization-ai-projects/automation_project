// projects/products/unstable/digital_pet/backend/src/care/care_action.rs
use crate::time::tick::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CareAction {
    pub kind: String,
    pub tick: Tick,
}
