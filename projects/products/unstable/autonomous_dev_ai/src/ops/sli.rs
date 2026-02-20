//projects/products/unstable/autonomous_dev_ai/src/ops/sli.rs
use serde::{Deserialize, Serialize};

/// A Service-Level Indicator: a measurable signal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sli {
    pub name: String,
    pub description: String,
}
