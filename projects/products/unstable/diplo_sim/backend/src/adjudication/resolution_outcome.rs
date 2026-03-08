use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionOutcome {
    Stayed,
    Moved,
    Bounced,
    Supported,
}
