// projects/products/unstable/auto_manager_ai/src/domain/action_target.rs

use serde::{Deserialize, Serialize};

/// Target type for an action
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ActionTarget {
    #[serde(rename = "repo")]
    Repo { 
        #[serde(rename = "ref")]
        reference: String 
    },
    #[serde(rename = "pr")]
    Pr { 
        #[serde(rename = "ref")]
        reference: String 
    },
    #[serde(rename = "issue")]
    Issue { 
        #[serde(rename = "ref")]
        reference: String 
    },
    #[serde(rename = "branch")]
    Branch { 
        #[serde(rename = "ref")]
        reference: String 
    },
    #[serde(rename = "workflow_run")]
    WorkflowRun { 
        #[serde(rename = "ref")]
        reference: String 
    },
}
