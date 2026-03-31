/// A single entry in a ranking report.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct RankingEntry {
    pub(crate) doc_id: String,
    pub(crate) score: f64,
}

/// Full ranking report for a query.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct RankingReport {
    pub(crate) query_terms: Vec<String>,
    pub(crate) results: Vec<RankingEntry>,
}
