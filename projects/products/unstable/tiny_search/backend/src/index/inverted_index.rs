use std::collections::BTreeMap;

use crate::index::doc_id::DocId;

/// A posting: document id + positions where the term appears.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct Posting {
    pub(crate) doc_id: DocId,
    pub(crate) positions: Vec<usize>,
    pub(crate) term_frequency: usize,
}

/// Inverted index mapping terms to postings lists.
/// Uses BTreeMap for deterministic ordering.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct InvertedIndex {
    pub(crate) postings: BTreeMap<String, Vec<Posting>>,
    pub(crate) doc_count: usize,
    pub(crate) doc_lengths: BTreeMap<String, usize>,
}

impl InvertedIndex {
    pub(crate) fn new() -> Self {
        Self {
            postings: BTreeMap::new(),
            doc_count: 0,
            doc_lengths: BTreeMap::new(),
        }
    }

    pub(crate) fn add_document(
        &mut self,
        doc_id: &DocId,
        tokens: &[crate::tokenize::token::Token],
    ) {
        self.doc_count += 1;
        self.doc_lengths.insert(doc_id.0.clone(), tokens.len());

        let mut term_positions: BTreeMap<String, Vec<usize>> = BTreeMap::new();
        for token in tokens {
            term_positions
                .entry(token.term.clone())
                .or_default()
                .push(token.position);
        }

        for (term, positions) in term_positions {
            let tf = positions.len();
            let posting = Posting {
                doc_id: doc_id.clone(),
                positions,
                term_frequency: tf,
            };
            self.postings.entry(term).or_default().push(posting);
        }
    }

    pub(crate) fn get_postings(&self, term: &str) -> Option<&Vec<Posting>> {
        self.postings.get(term)
    }

    pub(crate) fn document_frequency(&self, term: &str) -> usize {
        self.postings.get(term).map_or(0, |p| p.len())
    }
}
