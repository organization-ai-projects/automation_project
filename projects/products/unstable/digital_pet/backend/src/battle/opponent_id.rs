// projects/products/unstable/digital_pet/backend/src/battle/opponent_id.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OpponentId(pub String);
