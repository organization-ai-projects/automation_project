use serde::{Deserialize, Serialize};
use crate::model::candidate_id::CandidateId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    CampaignRally { target_block: String },
    MediaAppearance,
    PolicyAnnouncement { topic: String, position: i32 },
    AttackOpponent { target: CandidateId },
    DamageControl,
    Fundraising,
}
