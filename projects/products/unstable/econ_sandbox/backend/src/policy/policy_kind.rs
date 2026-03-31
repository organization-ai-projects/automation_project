use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyKind {
    FlatTax { rate_pct: u64 },
    Subsidy { per_agent: i64 },
}
