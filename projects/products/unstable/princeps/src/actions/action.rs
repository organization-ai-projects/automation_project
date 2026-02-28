use crate::model::candidate_id::CandidateId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    CampaignRally { target_block: String },
    MediaAppearance,
    PolicyAnnouncement { topic: String, position: i32 },
    AttackOpponent { target: CandidateId },
    DamageControl,
    Fundraising,
}
