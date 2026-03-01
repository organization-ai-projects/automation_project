use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum JobKind {
    Gather,
    Build,
    Rest,
    Haul,
    Guard,
}
