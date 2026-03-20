//! projects/products/unstable/neurosymbolic_moe/backend/src/retrieval_engine/chunker.rs
use protocol::ProtocolId;
use serde::{Deserialize, Serialize};

use super::chunk::Chunk;
use super::chunking_strategy::ChunkingStrategy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunker {
    pub strategy: ChunkingStrategy,
}

impl Chunker {
    pub fn new(strategy: ChunkingStrategy) -> Self {
        Self { strategy }
    }

    pub fn chunk(&self, text: &str, source: &str) -> Vec<Chunk> {
        match &self.strategy {
            ChunkingStrategy::FixedSize(size) => self.chunk_fixed_size(text, source, *size),
            ChunkingStrategy::Paragraph => self.chunk_by_paragraph(text, source),
            ChunkingStrategy::Semantic => self.chunk_semantic(text, source),
        }
    }

    fn chunk_fixed_size(&self, text: &str, source: &str, size: usize) -> Vec<Chunk> {
        let mut chunks = Vec::new();
        let chars: Vec<char> = text.chars().collect();
        let mut offset = 0;

        for window in chars.chunks(size) {
            let content: String = window.iter().collect();
            let byte_start = offset;
            let byte_end = offset + content.len();
            offset = byte_end;

            chunks.push(Chunk::new(
                ProtocolId::default(),
                content,
                source,
                byte_start,
                byte_end,
            ));
        }

        chunks
    }

    fn chunk_by_paragraph(&self, text: &str, source: &str) -> Vec<Chunk> {
        let mut chunks = Vec::new();
        let mut search_from = 0;
        let paragraphs: Vec<&str> = text.split("\n\n").collect();

        for (i, paragraph) in paragraphs.iter().enumerate() {
            let trimmed = paragraph.trim();
            if trimmed.is_empty() {
                search_from += paragraph.len();
                if i < paragraphs.len() - 1 {
                    search_from += 2;
                }
                continue;
            }

            let start = text[search_from..]
                .find(trimmed)
                .map(|pos| search_from + pos)
                .unwrap_or(search_from);
            let end = start + trimmed.len();

            chunks.push(Chunk::new(
                ProtocolId::default(),
                trimmed,
                source,
                start,
                end,
            ));

            search_from += paragraph.len();
            if i < paragraphs.len() - 1 {
                search_from += 2;
            }
        }

        chunks
    }

    fn chunk_semantic(&self, text: &str, source: &str) -> Vec<Chunk> {
        let mut chunks = Vec::new();
        let mut search_from = 0;

        for sentence in text.split_inclusive(['.', '!', '?']) {
            let trimmed = sentence.trim();
            if trimmed.is_empty() {
                search_from += sentence.len();
                continue;
            }

            let start = text[search_from..]
                .find(trimmed)
                .map(|pos| search_from + pos)
                .unwrap_or(search_from);
            let end = start + trimmed.len();

            chunks.push(Chunk::new(
                ProtocolId::default(),
                trimmed,
                source,
                start,
                end,
            ));

            search_from += sentence.len();
        }

        chunks
    }
}
