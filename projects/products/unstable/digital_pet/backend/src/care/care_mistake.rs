// projects/products/unstable/digital_pet/backend/src/care/care_mistake.rs
use crate::time::tick::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CareMistake {
    pub reason: String,
    pub tick: Tick,
}
