#![cfg(test)]

use crate::model::doc_id::DocId;
use crate::model::document::Document;
use crate::persistence::doc_snapshot::DocSnapshot;

#[test]
fn test_snapshot_determinism_same_doc_same_checksum() {
    let doc = Document::new(DocId::new("doc1"), "Hello");
    let first = DocSnapshot::create(&doc, 1, vec![]);
    let second = DocSnapshot::create(&doc, 1, vec![]);

    if let (Ok(first), Ok(second)) = (first, second) {
        assert_eq!(first.checksum, second.checksum);
    } else {
        assert!(false);
    }
}
