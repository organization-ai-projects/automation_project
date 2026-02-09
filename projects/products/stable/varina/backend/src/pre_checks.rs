// projects/products/varina/backend/src/pre_checks.rs
use serde::{Deserialize, Serialize};

/// Level of checks before commit/push.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreChecks {
    None,
    FmtCheck,
    FmtCheckAndTests,
}
