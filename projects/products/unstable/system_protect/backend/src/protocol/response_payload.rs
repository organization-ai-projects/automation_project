use serde::{Deserialize, Serialize};

use crate::moe_protect::engine_status::EngineStatus;
use crate::moe_protect::expert_info::ExpertInfo;
use crate::moe_protect::protection_result::ProtectionResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ResponsePayload {
    Ok,
    Error { message: String },
    ProtectionResult { result: ProtectionResult },
    ExpertList { experts: Vec<ExpertInfo> },
    Status { status: EngineStatus },
}
