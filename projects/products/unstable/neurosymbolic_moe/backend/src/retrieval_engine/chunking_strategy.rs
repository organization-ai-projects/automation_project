use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChunkingStrategy {
    FixedSize(usize),
    Paragraph,
    Semantic,
}
