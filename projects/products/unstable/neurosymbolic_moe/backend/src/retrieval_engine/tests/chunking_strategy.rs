use crate::retrieval_engine::ChunkingStrategy;

#[test]
fn chunking_strategy_variants_are_constructible() {
    let fixed = ChunkingStrategy::FixedSize(32);
    let paragraph = ChunkingStrategy::Paragraph;
    let semantic = ChunkingStrategy::Semantic;
    assert!(matches!(fixed, ChunkingStrategy::FixedSize(32)));
    assert!(matches!(paragraph, ChunkingStrategy::Paragraph));
    assert!(matches!(semantic, ChunkingStrategy::Semantic));
}
