// projects/products/unstable/universal_model_engine/backend/src/constraints/constraint_id.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConstraintId(pub String);
