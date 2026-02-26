// projects/products/unstable/autonomy_orchestrator_ai/src/domain/delivery_options.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeliveryOptions {
    pub enabled: bool,
    pub dry_run: bool,
    pub branch: Option<String>,
    pub commit_message: Option<String>,
    pub pr_enabled: bool,
    pub pr_number: Option<String>,
    pub pr_base: Option<String>,
    pub pr_title: Option<String>,
    pub pr_body: Option<String>,
}

impl DeliveryOptions {
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            dry_run: false,
            branch: None,
            commit_message: None,
            pr_enabled: false,
            pr_number: None,
            pr_base: None,
            pr_title: None,
            pr_body: None,
        }
    }
}
