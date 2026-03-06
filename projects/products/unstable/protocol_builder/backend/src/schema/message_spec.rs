// projects/products/unstable/protocol_builder/backend/src/schema/message_spec.rs
use serde::{Deserialize, Serialize};

use super::field_spec::FieldSpec;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSpec {
    pub name: String,
    pub fields: Vec<FieldSpec>,
}
