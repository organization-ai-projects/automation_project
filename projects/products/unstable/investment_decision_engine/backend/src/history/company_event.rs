use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventCategory {
    Earnings,
    GuidanceChange,
    ManagementChange,
    Acquisition,
    Divestiture,
    RegulatoryAction,
    ProductLaunch,
    LegalEvent,
    MarketEvent,
    Other,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompanyEvent {
    pub date: String,
    pub category: EventCategory,
    pub title: String,
    pub summary: String,
    pub impact_assessment: Option<String>,
}

impl CompanyEvent {
    pub fn new(
        date: impl Into<String>,
        category: EventCategory,
        title: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            date: date.into(),
            category,
            title: title.into(),
            summary: summary.into(),
            impact_assessment: None,
        }
    }
}
