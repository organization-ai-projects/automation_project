use serde::{Deserialize, Serialize};
use crate::debate::debate::Debate;
use crate::events::campaign_event::CampaignEvent;
use crate::poll::poll_report::PollReport;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Day {
    pub number: u32,
    pub events: Vec<CampaignEvent>,
    pub poll: Option<PollReport>,
    pub debate: Option<Debate>,
}

impl Day {
    pub fn new(number: u32) -> Self {
        Self {
            number,
            events: Vec::new(),
            poll: None,
            debate: None,
        }
    }
}
