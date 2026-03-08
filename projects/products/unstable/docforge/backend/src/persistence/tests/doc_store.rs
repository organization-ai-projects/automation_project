#![cfg(test)]

use crate::model::doc_id::DocId;
use crate::model::document::Document;
use crate::persistence::doc_snapshot::DocSnapshot;
use crate::persistence::doc_store::DocStore;

#[test]
fn test_doc_store_save_and_load() {
    let mut store = DocStore::new();
    let doc = Document::new(DocId::new("doc1"), "Store");
    let snapshot = DocSnapshot::create(&doc, 1, vec![]);
    assert!(snapshot.is_ok());
    let snapshot = match snapshot {
        Ok(value) => value,
        Err(_) => return,
    };
    store.save(snapshot);

    let loaded = store.load(&DocId::new("doc1"));
    assert!(loaded.is_some());
}
