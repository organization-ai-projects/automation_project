use crate::index::inverted_index::InvertedIndex;
use crate::query::query::Query;
use crate::rank::ranking_report::RankingReport;
use crate::rank::scorer::Scorer;

/// Executes a query against an inverted index and returns ranked results.
pub(crate) struct QueryEngine;

impl QueryEngine {
    pub(crate) fn execute(index: &InvertedIndex, query: &Query) -> RankingReport {
        Scorer::score(index, query)
    }
}
