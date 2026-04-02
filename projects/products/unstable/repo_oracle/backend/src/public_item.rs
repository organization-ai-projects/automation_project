use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PublicItem {
    pub name: String,
    pub kind: ItemKind,
    pub crate_name: String,
    pub module_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ItemKind {
    Constant,
    Enum,
    Function,
    Module,
    Static,
    Struct,
    Trait,
    TypeAlias,
}
