use crate::schemas::type_spec::TypeSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldSpec {
    pub name: String,
    pub required: bool,
    pub ty: TypeSpec,
}
