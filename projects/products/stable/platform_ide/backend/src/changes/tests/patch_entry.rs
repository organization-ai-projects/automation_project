//! projects/products/stable/platform_ide/backend/src/changes/tests/patch_entry.rs
use crate::changes::PatchEntry;
use crate::slices::AllowedPath;

#[test]
fn patch_entry_serializes_content_as_hex() {
    let entry = PatchEntry {
        path: AllowedPath::new_validated("src/main.rs".to_string()),
        content: vec![0xde, 0xad, 0xbe, 0xef],
    };

    let json = common_json::to_string(&entry).expect("serialize patch entry");
    assert!(json.contains("deadbeef"));

    let parsed: PatchEntry = common_json::from_json_str(&json).expect("deserialize patch entry");
    assert_eq!(parsed.path.as_str(), "src/main.rs");
    assert_eq!(parsed.content, vec![0xde, 0xad, 0xbe, 0xef]);
}
