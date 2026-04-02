use crate::manifest::{Manifest, ManifestEntry};

fn entry(path: &str) -> ManifestEntry {
    ManifestEntry {
        hash: "aabbcc".to_string(),
        path: path.to_string(),
        size: 0,
    }
}

#[test]
fn manifest_entries_are_sorted_by_path() {
    let entries = vec![
        entry("z_last.txt"),
        entry("a_first.txt"),
        entry("m_middle.txt"),
    ];
    let manifest = Manifest::new(entries);
    let paths: Vec<&str> = manifest.entries.iter().map(|e| e.path.as_str()).collect();
    assert_eq!(paths, vec!["a_first.txt", "m_middle.txt", "z_last.txt"]);
}

#[test]
fn manifest_subdirectory_entries_sorted_lexicographically() {
    let entries = vec![
        entry("z/file.txt"),
        entry("a/file.txt"),
        entry("a/other.txt"),
        entry("b/file.txt"),
    ];
    let manifest = Manifest::new(entries);
    let paths: Vec<&str> = manifest.entries.iter().map(|e| e.path.as_str()).collect();
    assert_eq!(
        paths,
        vec!["a/file.txt", "a/other.txt", "b/file.txt", "z/file.txt"]
    );
}

#[test]
fn manifest_canonical_json_has_sorted_entries() {
    let entries = vec![entry("c.txt"), entry("a.txt"), entry("b.txt")];
    let manifest = Manifest::new(entries);
    let json = manifest.to_canonical_json();

    let a_pos = json.find("a.txt").expect("a.txt in json");
    let b_pos = json.find("b.txt").expect("b.txt in json");
    let c_pos = json.find("c.txt").expect("c.txt in json");
    assert!(a_pos < b_pos, "a.txt must come before b.txt");
    assert!(b_pos < c_pos, "b.txt must come before c.txt");
}

#[test]
fn manifest_canonical_json_round_trips() {
    let entries = vec![
        ManifestEntry {
            hash: "deadbeef".to_string(),
            path: "sub/file.txt".to_string(),
            size: 42,
        },
        ManifestEntry {
            hash: "cafebabe".to_string(),
            path: "root.bin".to_string(),
            size: 7,
        },
    ];
    let manifest = Manifest::new(entries);
    let json = manifest.to_canonical_json();
    let parsed = Manifest::from_canonical_json(&json).expect("parse manifest JSON");

    assert_eq!(manifest.entries, parsed.entries);
    assert_eq!(manifest.format_version, parsed.format_version);
}

#[test]
fn manifest_json_keys_are_in_canonical_order() {
    let entry = ManifestEntry {
        hash: "abc".to_string(),
        path: "file.txt".to_string(),
        size: 1,
    };
    let manifest = Manifest::new(vec![entry]);
    let json = manifest.to_canonical_json();

    // Within an entry object: hash < path < size (alphabetical)
    let hash_pos = json.find("\"hash\"").expect("hash key");
    let path_pos = json.find("\"path\"").expect("path key");
    let size_pos = json.find("\"size\"").expect("size key");
    assert!(hash_pos < path_pos, "hash must precede path");
    assert!(path_pos < size_pos, "path must precede size");

    // Top-level: entries < format_version (alphabetical)
    let entries_pos = json.find("\"entries\"").expect("entries key");
    let fv_pos = json.find("\"format_version\"").expect("format_version key");
    assert!(entries_pos < fv_pos, "entries must precede format_version");
}
