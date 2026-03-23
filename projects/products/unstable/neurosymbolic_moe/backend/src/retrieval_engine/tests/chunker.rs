use crate::retrieval_engine::{Chunker, ChunkingStrategy};

#[test]
fn semantic_chunking_creates_non_empty_chunks() {
    let chunker = Chunker::new(ChunkingStrategy::Semantic);
    let chunks = chunker.chunk("One. Two! Three?", "src");
    assert!(!chunks.is_empty());
    assert!(chunks.iter().all(|chunk| !chunk.content.is_empty()));
}
