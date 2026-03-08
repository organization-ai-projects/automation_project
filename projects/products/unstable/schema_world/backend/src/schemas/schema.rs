use crate::schemas::field_spec::FieldSpec;
use crate::schemas::version::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Schema {
    pub name: String,
    pub version: Version,
    pub fields: Vec<FieldSpec>,
}
