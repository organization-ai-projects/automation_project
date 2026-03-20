//! projects/products/unstable/neurosymbolic_moe/backend/src/retrieval_engine/chunking_strategy.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChunkingStrategy {
    FixedSize(usize),
    Paragraph,
    Semantic,
}
