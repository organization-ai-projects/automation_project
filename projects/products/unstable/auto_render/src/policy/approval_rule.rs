use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApprovalRule {
    AutoApprove,
    RequireApproval { reason: String },
}
