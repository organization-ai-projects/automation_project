use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtectionAction {
    Allow,
    Block,
    Quarantine,
    Alert,
    Log,
    RateLimit,
    Redirect,
}
