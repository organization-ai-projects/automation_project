use crate::memory_engine::{
    LongTermMemory, MemoryEntry, MemoryQuery, MemoryStore, MemoryType, ShortTermMemory,
};
use crate::moe_core::Task;
use crate::retrieval_engine::{
    Chunk, Chunker, ChunkingStrategy, ContextAssembler, RetrievalQuery, Retriever, SimpleRetriever,
};
use std::collections::HashMap;

fn memory_entry(
    id: &str,
    content: &str,
    tags: Vec<&str>,
    created_at: u64,
    expires_at: Option<u64>,
    memory_type: MemoryType,
    relevance: f64,
) -> MemoryEntry {
    MemoryEntry {
        id: id.to_string(),
        content: content.to_string(),
        tags: tags.into_iter().map(|tag| tag.to_string()).collect(),
        created_at,
        expires_at,
        memory_type,
        relevance,
        metadata: HashMap::new(),
    }
}

#[test]
fn v2_retriever_supports_filtering_and_ranking() {
    let mut retriever = SimpleRetriever::new();
    retriever.add_document(
        Chunk::new("c1", "rust rust deterministic systems", "src://a", 0, 30)
            .with_metadata("domain", "systems"),
    );
    retriever.add_document(
        Chunk::new("c2", "rust docs and markdown", "src://b", 0, 22)
            .with_metadata("domain", "docs"),
    );
    retriever.add_document(
        Chunk::new("c3", "python scripting utilities", "src://c", 0, 24)
            .with_metadata("domain", "systems"),
    );

    let query = RetrievalQuery::new("rust")
        .with_filter("domain", "systems")
        .with_min_relevance(0.05)
        .with_max_results(5);
    let results = retriever
        .retrieve(&query)
        .expect("retrieval should succeed");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].chunk_id, "c1");
    assert!(results[0].relevance_score > 0.0);
}

#[test]
fn v2_chunking_strategies_generate_chunks() {
    let fixed = Chunker::new(ChunkingStrategy::FixedSize(4));
    let paragraph = Chunker::new(ChunkingStrategy::Paragraph);
    let semantic = Chunker::new(ChunkingStrategy::Semantic);

    let fixed_chunks = fixed.chunk("abcdefgh", "doc://fixed");
    let paragraph_chunks = paragraph.chunk("para one\n\npara two", "doc://paragraph");
    let semantic_chunks = semantic.chunk("Sentence one. Sentence two!", "doc://semantic");

    assert_eq!(fixed_chunks.len(), 2);
    assert_eq!(paragraph_chunks.len(), 2);
    assert!(!semantic_chunks.is_empty());
}

#[test]
fn v2_context_assembly_is_budgeted_and_task_aware() {
    let results = vec![
        crate::retrieval_engine::RetrievalResult::new("c1", "AAAAAAAAAA", 0.9, "doc://a"),
        crate::retrieval_engine::RetrievalResult::new("c2", "BBBBBBBBBB", 0.8, "doc://b"),
    ];
    let assembler = ContextAssembler::new(12);
    let assembled = assembler.assemble(&results);
    let total_len: usize = assembled.iter().map(String::len).sum();
    assert!(total_len <= 12);

    let task = Task::new(
        "task-v2",
        crate::moe_core::TaskType::Retrieval,
        "lookup context",
    );
    let task_context = assembler.assemble_for_task(&results, &task);
    assert!(!task_context.is_empty());
    assert!(task_context[0].contains("task-v2"));
}

#[test]
fn v2_short_term_memory_supports_capacity_retrieve_and_expire() {
    let mut short = ShortTermMemory::new(2);
    short
        .store(memory_entry(
            "m1",
            "alpha",
            vec!["runtime"],
            1,
            Some(10),
            MemoryType::Short,
            0.9,
        ))
        .expect("storing m1 should succeed");
    short
        .store(memory_entry(
            "m2",
            "beta",
            vec!["runtime"],
            2,
            Some(20),
            MemoryType::Short,
            0.8,
        ))
        .expect("storing m2 should succeed");
    short
        .store(memory_entry(
            "m3",
            "gamma",
            vec!["runtime"],
            3,
            Some(30),
            MemoryType::Short,
            0.7,
        ))
        .expect("storing m3 should succeed");

    assert_eq!(short.count(), 2);
    assert!(short.remove("m1").is_none());

    let query = MemoryQuery {
        tags: Some(vec!["runtime".to_string()]),
        memory_type: Some(MemoryType::Short),
        min_relevance: Some(0.0),
        max_results: 10,
        include_expired: false,
        current_time: Some(15),
    };
    let retrieved = short
        .retrieve(&query)
        .expect("retrieval from short memory should succeed");
    assert!(!retrieved.is_empty());

    let expired = short.expire(25);
    assert!(expired >= 1);
}

#[test]
fn v2_long_term_memory_supports_retrieve_and_remove() {
    let mut long = LongTermMemory::new();
    long.store(memory_entry(
        "l1",
        "knowledge",
        vec!["history"],
        1,
        None,
        MemoryType::Long,
        0.95,
    ))
    .expect("storing long-term entry should succeed");

    let query = MemoryQuery {
        tags: Some(vec!["history".to_string()]),
        memory_type: Some(MemoryType::Long),
        min_relevance: Some(0.5),
        max_results: 5,
        include_expired: true,
        current_time: Some(0),
    };
    let retrieved = long
        .retrieve(&query)
        .expect("retrieval from long memory should succeed");
    assert_eq!(retrieved.len(), 1);

    let removed = long.remove("l1");
    assert!(removed.is_some());
    assert_eq!(long.count(), 0);
}
