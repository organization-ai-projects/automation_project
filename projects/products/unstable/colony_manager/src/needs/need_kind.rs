use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum NeedKind {
    Food,
    Rest,
    Social,
    Safety,
}
