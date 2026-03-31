use crate::antivirus::signature::Signature;
use crate::antivirus::signature_db::SignatureDb;

#[test]
fn default_db_has_signatures() {
    let db = SignatureDb::with_defaults();
    assert!(db.count() > 0);
}

#[test]
fn scan_returns_matching_signatures() {
    let mut db = SignatureDb::new();
    db.add(Signature::new("test-virus", "VIRUS_PAYLOAD", "high"));
    db.add(Signature::new("test-clean", "CLEAN_PATTERN", "low"));

    let matches = db.scan("file contains VIRUS_PAYLOAD data");
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].name, "test-virus");
}

#[test]
fn scan_returns_empty_for_clean_content() {
    let db = SignatureDb::with_defaults();
    let matches = db.scan("perfectly normal file content");
    assert!(matches.is_empty());
}
