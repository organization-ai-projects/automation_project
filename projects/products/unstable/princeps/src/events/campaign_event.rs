use crate::model::candidate_id::CandidateId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CampaignEvent {
    Scandal {
        target: CandidateId,
        description: String,
        severity: u8,
        approval_delta: f64,
    },
    Endorsement {
        target: CandidateId,
        source: String,
        approval_delta: f64,
    },
    Gaffe {
        target: CandidateId,
        description: String,
        approval_delta: f64,
    },
    PolicyWin {
        target: CandidateId,
        topic: String,
        approval_delta: f64,
    },
    PolicyFail {
        target: CandidateId,
        topic: String,
        approval_delta: f64,
    },
    DebateMoment {
        day: u32,
    },
}

impl CampaignEvent {
    pub fn target_candidate(&self) -> Option<&CandidateId> {
        match self {
            Self::Scandal { target, .. } => Some(target),
            Self::Endorsement { target, .. } => Some(target),
            Self::Gaffe { target, .. } => Some(target),
            Self::PolicyWin { target, .. } => Some(target),
            Self::PolicyFail { target, .. } => Some(target),
            Self::DebateMoment { .. } => None,
        }
    }

    pub fn approval_delta(&self) -> f64 {
        match self {
            Self::Scandal { approval_delta, .. } => *approval_delta,
            Self::Endorsement { approval_delta, .. } => *approval_delta,
            Self::Gaffe { approval_delta, .. } => *approval_delta,
            Self::PolicyWin { approval_delta, .. } => *approval_delta,
            Self::PolicyFail { approval_delta, .. } => *approval_delta,
            Self::DebateMoment { .. } => 0.0,
        }
    }
}
