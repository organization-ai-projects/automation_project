use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Request {
    PlanDsl {
        dsl: String,
    },
    ApplyPlan {
        plan_json: String,
    },
    VerifyPlan {
        plan_json: String,
        file_contents: std::collections::BTreeMap<String, String>,
    },
    Shutdown,
}
