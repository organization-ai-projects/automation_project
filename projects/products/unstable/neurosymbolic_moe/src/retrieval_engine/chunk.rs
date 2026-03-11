use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub id: String,
    pub content: String,
    pub source: String,
    pub start_offset: usize,
    pub end_offset: usize,
    pub metadata: HashMap<String, String>,
}

impl Chunk {
    pub fn new(
        id: impl Into<String>,
        content: impl Into<String>,
        source: impl Into<String>,
        start_offset: usize,
        end_offset: usize,
    ) -> Self {
        Self {
            id: id.into(),
            content: content.into(),
            source: source.into(),
            start_offset,
            end_offset,
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChunkingStrategy {
    FixedSize(usize),
    Paragraph,
    Semantic,
}

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

        for (i, window) in chars.chunks(size).enumerate() {
            let content: String = window.iter().collect();
            let byte_start = offset;
            let byte_end = offset + content.len();
            offset = byte_end;

            chunks.push(Chunk::new(
                format!("{source}-chunk-{i}"),
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
                // Advance past this segment and its delimiter.
                search_from += paragraph.len();
                if i < paragraphs.len() - 1 {
                    search_from += 2; // "\n\n"
                }
                continue;
            }

            let start = text[search_from..]
                .find(trimmed)
                .map(|pos| search_from + pos)
                .unwrap_or(search_from);
            let end = start + trimmed.len();

            chunks.push(Chunk::new(
                format!("{source}-para-{i}"),
                trimmed,
                source,
                start,
                end,
            ));

            // Advance past the current segment and the delimiter.
            search_from += paragraph.len();
            if i < paragraphs.len() - 1 {
                search_from += 2; // "\n\n"
            }
        }

        chunks
    }

    /// Semantic chunking falls back to sentence-level splitting.
    fn chunk_semantic(&self, text: &str, source: &str) -> Vec<Chunk> {
        let mut chunks = Vec::new();
        let mut search_from = 0;

        for (i, sentence) in text.split_inclusive(['.', '!', '?']).enumerate() {
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
                format!("{source}-sent-{i}"),
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
