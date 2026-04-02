//! projects/products/unstable/neurosymbolic_moe/backend/src/retrieval_engine/tests/chunk.rs
use crate::retrieval_engine::{Chunker, ChunkingStrategy};

#[test]
fn fixed_size_chunking() {
    let chunker = Chunker::new(ChunkingStrategy::FixedSize(5));
    let chunks = chunker.chunk("HelloWorld!", "src");
    assert_eq!(chunks.len(), 3);
    assert_eq!(chunks[0].content, "Hello");
    assert_eq!(chunks[1].content, "World");
    assert_eq!(chunks[2].content, "!");
    assert_eq!(chunks[0].source, "src");
}

#[test]
fn paragraph_chunking() {
    let chunker = Chunker::new(ChunkingStrategy::Paragraph);
    let text = "First paragraph.\n\nSecond paragraph.\n\nThird paragraph.";
    let chunks = chunker.chunk(text, "doc");
    assert_eq!(chunks.len(), 3);
    assert_eq!(chunks[0].content, "First paragraph.");
    assert_eq!(chunks[1].content, "Second paragraph.");
    assert_eq!(chunks[2].content, "Third paragraph.");
}
