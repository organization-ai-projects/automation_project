use serde::{Deserialize, Serialize};

use crate::journal::DecisionEntry;
use crate::report::{AssetReport, DecisionReport, PortfolioReport};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub id: Option<String>,
    #[serde(flatten)]
    pub payload: ResponsePayload,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResponsePayload {
    DecisionReport { report: DecisionReport },
    AssetReport { report: AssetReport },
    PortfolioReport { report: PortfolioReport },
    JournalEntries { entries: Vec<DecisionEntry> },
    ReplayReport { report: DecisionReport },
    Error { message: String },
}

impl Response {
    pub fn error(id: Option<String>, message: impl Into<String>) -> Self {
        Self {
            id,
            payload: ResponsePayload::Error {
                message: message.into(),
            },
        }
    }
}
