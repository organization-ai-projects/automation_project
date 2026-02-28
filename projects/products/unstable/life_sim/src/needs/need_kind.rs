use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum NeedKind {
    Hunger,
    Energy,
    Social,
    Fun,
    Hygiene,
    Bladder,
    Comfort,
}
