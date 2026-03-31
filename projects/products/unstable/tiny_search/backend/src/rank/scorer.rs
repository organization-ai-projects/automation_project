use std::collections::BTreeMap;

use crate::index::inverted_index::InvertedIndex;
use crate::query::query::Query;
use crate::rank::ranking_report::{RankingEntry, RankingReport};

/// Deterministic TF-IDF-like scorer.
pub(crate) struct Scorer;

impl Scorer {
    /// Score documents against the given query terms.
    /// Uses a simplified TF-IDF: score = sum over terms of (tf * idf).
    /// IDF = ln(1 + doc_count / df).
    /// Tie-breaking: by document id (lexicographic, ascending).
    pub(crate) fn score(index: &InvertedIndex, query: &Query) -> RankingReport {
        let mut scores: BTreeMap<String, f64> = BTreeMap::new();

        for term in &query.terms {
            let df = index.document_frequency(term);
            if df == 0 {
                continue;
            }
            let idf = ((1.0 + index.doc_count as f64) / (1.0 + df as f64)).ln() + 1.0;

            if let Some(postings) = index.get_postings(term) {
                for posting in postings {
                    let tf = posting.term_frequency as f64;
                    let doc_len = index
                        .doc_lengths
                        .get(&posting.doc_id.0)
                        .copied()
                        .unwrap_or(1) as f64;
                    let normalized_tf = tf / doc_len;
                    let term_score = normalized_tf * idf;
                    *scores.entry(posting.doc_id.0.clone()).or_insert(0.0) += term_score;
                }
            }
        }

        let mut entries: Vec<RankingEntry> = scores
            .into_iter()
            .map(|(doc_id, score)| RankingEntry { doc_id, score })
            .collect();

        // Sort: descending by score, then ascending by doc_id for tie-breaking.
        entries.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.doc_id.cmp(&b.doc_id))
        });

        RankingReport {
            query_terms: query.terms.clone(),
            results: entries,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::index::doc_id::DocId;
    use crate::index::inverted_index::InvertedIndex;
    use crate::query::query::Query;
    use crate::tokenize::tokenizer::Tokenizer;

    use super::*;

    #[test]
    fn scoring_determinism() {
        let mut index = InvertedIndex::new();
        let tokens_a = Tokenizer::tokenize("rust programming language");
        let tokens_b = Tokenizer::tokenize("python programming language");
        index.add_document(&DocId::from_path("a.txt"), &tokens_a);
        index.add_document(&DocId::from_path("b.txt"), &tokens_b);

        let query = Query {
            terms: vec!["rust".to_string(), "programming".to_string()],
        };

        let r1 = Scorer::score(&index, &query);
        let r2 = Scorer::score(&index, &query);

        assert_eq!(r1.results.len(), r2.results.len());
        for (a, b) in r1.results.iter().zip(r2.results.iter()) {
            assert_eq!(a.doc_id, b.doc_id);
            assert!((a.score - b.score).abs() < f64::EPSILON);
        }
        assert!(!r1.results.is_empty());
        // a.txt should rank higher because it has "rust"
        assert_eq!(r1.results[0].doc_id, "a.txt");
    }

    #[test]
    fn tie_breaking_by_doc_id() {
        let mut index = InvertedIndex::new();
        let tokens = Tokenizer::tokenize("hello");
        index.add_document(&DocId::from_path("b.txt"), &tokens);
        let tokens2 = Tokenizer::tokenize("hello");
        index.add_document(&DocId::from_path("a.txt"), &tokens2);

        let query = Query {
            terms: vec!["hello".to_string()],
        };
        let report = Scorer::score(&index, &query);
        assert_eq!(report.results.len(), 2);
        // Same score, tie-break by doc_id ascending
        assert_eq!(report.results[0].doc_id, "a.txt");
        assert_eq!(report.results[1].doc_id, "b.txt");
    }
}
