use crate::index::doc_id::DocId;
use crate::index::index_store::IndexStore;
use crate::index::inverted_index::InvertedIndex;
use crate::persistence::index_snapshot::IndexSnapshot;
use crate::persistence::snapshot_codec::SnapshotCodec;
use crate::query::query_engine::QueryEngine;
use crate::query::query_parser::QueryParser;
use crate::tokenize::tokenizer::Tokenizer;

/// Helper: create a temp directory with fixture files.
fn create_fixture_corpus(dir: &std::path::Path) {
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(
        dir.join("doc1.txt"),
        "Rust is a systems programming language focused on safety.",
    )
    .unwrap();
    std::fs::write(
        dir.join("doc2.txt"),
        "Python is a popular programming language for data science.",
    )
    .unwrap();
    std::fs::write(
        dir.join("doc3.md"),
        "Rust and Python are both great programming languages.",
    )
    .unwrap();
}

#[test]
fn integration_index_query_golden_report() {
    let tmp = std::env::temp_dir().join("tiny_search_test_integration");
    let corpus_dir = tmp.join("corpus");
    let snapshot_path = tmp.join("snapshot.json");

    // Clean up from any previous run
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();

    create_fixture_corpus(&corpus_dir);

    // Build index
    let index = IndexStore::build_from_dir(&corpus_dir).unwrap();
    assert_eq!(index.doc_count, 3);

    // Save snapshot
    SnapshotCodec::save(&index, &snapshot_path).unwrap();
    assert!(snapshot_path.exists());

    // Load snapshot
    let loaded_index = SnapshotCodec::load(&snapshot_path).unwrap();
    assert_eq!(loaded_index.doc_count, index.doc_count);

    // Query "rust programming"
    let query = QueryParser::parse("rust programming");
    let report = QueryEngine::execute(&loaded_index, &query);

    // All 3 docs contain "programming", doc1 and doc3 contain "rust"
    assert_eq!(report.results.len(), 3);
    // doc1.txt and doc3.md should rank higher because they have "rust"
    let top_ids: Vec<&str> = report.results.iter().map(|e| e.doc_id.as_str()).collect();
    assert!(
        top_ids[0] == "doc1.txt" || top_ids[0] == "doc3.md",
        "Top result should be doc1.txt or doc3.md, got: {}",
        top_ids[0]
    );

    // Determinism: run again, same results
    let report2 = QueryEngine::execute(&loaded_index, &query);
    assert_eq!(report.results.len(), report2.results.len());
    for (a, b) in report.results.iter().zip(report2.results.iter()) {
        assert_eq!(a.doc_id, b.doc_id);
        assert!((a.score - b.score).abs() < f64::EPSILON);
    }

    // Clean up
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn snapshot_canonical_determinism() {
    let mut index = InvertedIndex::new();
    let tokens_a = Tokenizer::tokenize("hello world foo bar");
    let tokens_b = Tokenizer::tokenize("foo baz world");
    index.add_document(&DocId::from_path("a.txt"), &tokens_a);
    index.add_document(&DocId::from_path("b.txt"), &tokens_b);

    let snap1 = IndexSnapshot::from_index(&index).unwrap();
    let snap2 = IndexSnapshot::from_index(&index).unwrap();

    // Same index => identical snapshot bytes
    assert_eq!(snap1.index_json, snap2.index_json);
    assert_eq!(snap1.checksum, snap2.checksum);
}

#[test]
fn snapshot_roundtrip() {
    let tmp = std::env::temp_dir().join("tiny_search_test_roundtrip");
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();

    let mut index = InvertedIndex::new();
    let tokens = Tokenizer::tokenize("hello world");
    index.add_document(&DocId::from_path("test.txt"), &tokens);

    let path = tmp.join("snapshot.json");
    SnapshotCodec::save(&index, &path).unwrap();
    let loaded = SnapshotCodec::load(&path).unwrap();

    assert_eq!(loaded.doc_count, index.doc_count);
    assert_eq!(loaded.postings.len(), index.postings.len());

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn query_no_matches() {
    let mut index = InvertedIndex::new();
    let tokens = Tokenizer::tokenize("hello world");
    index.add_document(&DocId::from_path("doc.txt"), &tokens);

    let query = QueryParser::parse("nonexistent");
    let report = QueryEngine::execute(&index, &query);
    assert!(report.results.is_empty());
}
