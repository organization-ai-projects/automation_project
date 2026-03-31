use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssetId {
    pub ticker: String,
    pub exchange: Option<String>,
}

impl AssetId {
    pub fn new(ticker: impl Into<String>) -> Self {
        Self {
            ticker: ticker.into(),
            exchange: None,
        }
    }

    pub fn with_exchange(ticker: impl Into<String>, exchange: impl Into<String>) -> Self {
        Self {
            ticker: ticker.into(),
            exchange: Some(exchange.into()),
        }
    }

    pub fn canonical_key(&self) -> String {
        match &self.exchange {
            Some(ex) => format!("{}:{}", ex, self.ticker),
            None => self.ticker.clone(),
        }
    }
}
