use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub id: Option<String>,
    #[serde(flatten)]
    pub payload: RequestPayload,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RequestPayload {
    AnalyzeAsset {
        asset_json: String,
        market_json: String,
    },
    AnalyzePortfolio {
        portfolio_json: String,
        market_json: String,
    },
    ReplayDecision {
        replay_json: String,
    },
    ReadJournal {
        journal_path: String,
    },
    EvaluateScenario {
        scenario_json: String,
        asset_json: String,
    },
    Shutdown,
}
