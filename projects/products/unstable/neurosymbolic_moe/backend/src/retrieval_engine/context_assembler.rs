//! projects/products/unstable/neurosymbolic_moe/backend/src/retrieval_engine/context_assembler.rs
use serde::{Deserialize, Serialize};
use std::cmp;

use crate::{moe_core::Task, retrieval_engine::retrieval_result::RetrievalResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextAssembler {
    pub max_context_length: usize,
}

impl ContextAssembler {
    pub fn new(max_context_length: usize) -> Self {
        Self { max_context_length }
    }

    /// Ranks results by relevance, truncates to `max_context_length`, and returns
    /// assembled context strings.
    pub fn assemble(&self, results: &[RetrievalResult]) -> Vec<String> {
        let mut ranked: Vec<&RetrievalResult> = results.iter().collect();
        ranked.sort_by(|a, b| {
            b.relevance_score
                .partial_cmp(&a.relevance_score)
                .unwrap_or(cmp::Ordering::Equal)
        });

        let mut assembled = Vec::new();
        let mut total_length = 0;

        for result in ranked {
            if total_length + result.content.len() > self.max_context_length {
                let remaining = self.max_context_length.saturating_sub(total_length);
                if remaining > 0 {
                    let truncated: String = result.content.chars().take(remaining).collect();
                    assembled.push(truncated);
                }
                break;
            }
            total_length += result.content.len();
            assembled.push(result.content.clone());
        }

        assembled
    }

    /// Task-aware assembly that prepends task context and input information.
    pub fn assemble_for_task(&self, results: &[RetrievalResult], task: &Task) -> Vec<String> {
        let header = format!("[task:{}] {}", task.id(), task.input());
        let header_len = header.len();

        if header_len >= self.max_context_length {
            return vec![header.chars().take(self.max_context_length).collect()];
        }

        let remaining_budget = self.max_context_length - header_len;
        let sub_assembler = ContextAssembler::new(remaining_budget);
        let mut context = vec![header];
        context.extend(sub_assembler.assemble(results));
        context
    }
}
