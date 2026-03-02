use crate::jobs::job_id::JobId;
use crate::jobs::job_kind::JobKind;
use crate::model::colonist_id::ColonistId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: JobId,
    pub kind: JobKind,
    pub priority: u32,
    pub assigned_to: Option<ColonistId>,
    pub ticks_remaining: u32,
}
