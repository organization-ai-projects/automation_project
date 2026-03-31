use serde::{Deserialize, Serialize};

use crate::assets::AssetId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetProfile {
    pub id: AssetId,
    pub name: String,
    pub sector: Option<String>,
    pub industry: Option<String>,
    pub description: Option<String>,
    pub market_cap_usd: Option<f64>,
    pub country: Option<String>,
}

impl AssetProfile {
    pub fn new(id: AssetId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            sector: None,
            industry: None,
            description: None,
            market_cap_usd: None,
            country: None,
        }
    }
}
