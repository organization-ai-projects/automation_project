use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NeuralInput {
    pub ticker: String,
    pub company_history_text: Option<String>,
    pub earnings_summary_text: Option<String>,
    pub news_articles: Vec<String>,
    pub analyst_notes: Vec<String>,
}

impl NeuralInput {
    pub fn new(ticker: impl Into<String>) -> Self {
        Self {
            ticker: ticker.into(),
            company_history_text: None,
            earnings_summary_text: None,
            news_articles: Vec::new(),
            analyst_notes: Vec::new(),
        }
    }

    pub fn has_content(&self) -> bool {
        self.company_history_text.is_some()
            || self.earnings_summary_text.is_some()
            || !self.news_articles.is_empty()
            || !self.analyst_notes.is_empty()
    }
}
