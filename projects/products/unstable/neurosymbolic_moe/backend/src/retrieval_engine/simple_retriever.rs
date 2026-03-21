//! projects/products/unstable/neurosymbolic_moe/backend/src/retrieval_engine/simple_retriever.rs
use std::{cmp, collections};

use serde::{Deserialize, Serialize};

use crate::{
    moe_core::MoeError,
    retrieval_engine::{
        retrieval_query::RetrievalQuery, retrieval_result::RetrievalResult, retriever::Retriever,
    },
};

use super::chunk::Chunk;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleRetriever {
    documents: Vec<Chunk>,
}

impl SimpleRetriever {
    pub fn new() -> Self {
        Self {
            documents: Vec::new(),
        }
    }

    pub fn add_document(&mut self, chunk: Chunk) {
        self.documents.push(chunk);
    }

    /// Computes relevance as the fraction of the content covered by non-overlapping
    /// occurrences of the query substring (match density).
    fn compute_relevance(content: &str, query: &str) -> f64 {
        if query.is_empty() || content.is_empty() {
            return 0.0;
        }

        let content_lower = content.to_lowercase();
        let query_lower = query.to_lowercase();

        let mut matches = 0usize;
        let mut start = 0;
        while let Some(pos) = content_lower[start..].find(&query_lower) {
            matches += 1;
            start += pos + query_lower.len();
        }

        if matches == 0 {
            return 0.0;
        }

        let covered = matches * query_lower.len();
        (covered as f64 / content_lower.len() as f64).min(1.0)
    }

    fn matches_filters(chunk: &Chunk, filters: &collections::HashMap<String, String>) -> bool {
        filters
            .iter()
            .all(|(k, v)| chunk.metadata.get(k).is_some_and(|val| val == v))
    }
}

impl Default for SimpleRetriever {
    fn default() -> Self {
        Self::new()
    }
}

impl Retriever for SimpleRetriever {
    fn retrieve(&self, query: &RetrievalQuery) -> Result<Vec<RetrievalResult>, MoeError> {
        let mut results: Vec<RetrievalResult> = self
            .documents
            .iter()
            .filter(|chunk| Self::matches_filters(chunk, &query.filters))
            .filter_map(|chunk| {
                let relevance = Self::compute_relevance(&chunk.content, &query.query);
                if relevance >= query.min_relevance && relevance > 0.0 {
                    Some(RetrievalResult {
                        chunk_id: chunk.id,
                        content: chunk.content.clone(),
                        relevance_score: relevance,
                        source: chunk.source.clone(),
                        metadata: chunk.metadata.clone(),
                    })
                } else {
                    None
                }
            })
            .collect();

        results.sort_by(|a, b| {
            b.relevance_score
                .partial_cmp(&a.relevance_score)
                .unwrap_or(cmp::Ordering::Equal)
        });

        results.truncate(query.max_results);

        Ok(results)
    }
}
