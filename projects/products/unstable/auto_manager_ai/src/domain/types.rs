use serde::{Deserialize, Serialize};

/// Risk level for an action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

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

/// Evidence source for an action
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evidence {
    pub source: String,
    pub pointer: String,
}

/// Status of an action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActionStatus {
    Proposed,
    #[serde(rename = "needs_input")]
    NeedsInput,
    #[serde(rename = "blocked_by_policy")]
    BlockedByPolicy,
}

/// Dry-run step for an action
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DryRunStep {
    pub tool: String,
    pub command: String,
    pub expected: String,
    pub failure_modes: Vec<String>,
}

/// Dry-run information for an action
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DryRun {
    pub steps: Vec<DryRunStep>,
}
