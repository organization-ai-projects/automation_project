use std::fs;
use std::path::PathBuf;

use crate::bundle::BundlePacker;
use crate::bundle::bundle_format::MAGIC;
use crate::output::canonical_json::render_verify_report;
use crate::verify::verify_report::{EntryStatus, VerifyReport};
use crate::verify::Verifier;

fn make_temp_dir(suffix: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("artifact_lab_gv_{suffix}"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

#[test]
fn verify_clean_bundle_produces_all_ok_report() {
    let src = make_temp_dir("gv_ok_src");
    let bundle_path = make_temp_dir("gv_ok_bnd").join("ok.artl");

    fs::write(src.join("a.txt"), b"hello").unwrap();
    fs::write(src.join("b.txt"), b"world").unwrap();

    BundlePacker::pack(&src, &bundle_path).expect("pack");

    let report = Verifier::verify(&bundle_path).expect("verify");

    assert!(report.ok);
    assert_eq!(report.entry_count, 2);
    assert_eq!(report.results.len(), 2);
    for r in &report.results {
        assert_eq!(r.status_label(), "ok");
    }
}

#[test]
fn verify_corrupted_bundle_reports_hash_mismatch() {
    let src = make_temp_dir("gv_corrupt_src");
    let bundle_path = make_temp_dir("gv_corrupt_bnd").join("corrupt.artl");

    fs::write(src.join("file.txt"), b"original content").unwrap();
    BundlePacker::pack(&src, &bundle_path).expect("pack");

    // Corrupt the last few bytes of the bundle (file content area)
    let mut bytes = fs::read(&bundle_path).expect("read bundle");
    let len = bytes.len();
    for b in bytes.iter_mut().take(len).skip(len - 5) {
        *b ^= 0xFF;
    }
    fs::write(&bundle_path, &bytes).expect("write corrupted bundle");

    let report = Verifier::verify(&bundle_path).expect("verify corrupted");
    assert!(!report.ok, "corrupted bundle must fail verification");
    assert!(
        report.results.iter().any(|r| r.status_label() == "hash_mismatch"),
        "must report hash_mismatch"
    );
}

#[test]
fn verify_report_json_structure_is_correct() {
    let report = VerifyReport {
        ok: true,
        entry_count: 1,
        results: vec![crate::verify::EntryResult {
            path: "some/file.txt".to_string(),
            status: EntryStatus::Ok,
        }],
    };

    let json = render_verify_report(&report);

    assert!(json.contains("\"ok\": true"));
    assert!(json.contains("\"entry_count\": 1"));
    assert!(json.contains("\"status\": \"ok\""));
    assert!(json.contains("\"path\": \"some/file.txt\""));
}

#[test]
fn verify_report_json_hash_mismatch_contains_details() {
    let report = VerifyReport {
        ok: false,
        entry_count: 1,
        results: vec![crate::verify::EntryResult {
            path: "broken.txt".to_string(),
            status: EntryStatus::HashMismatch {
                expected: "expected_hash_value".to_string(),
                actual: "actual_hash_value".to_string(),
            },
        }],
    };

    let json = render_verify_report(&report);

    assert!(json.contains("\"ok\": false"));
    assert!(json.contains("\"status\": \"hash_mismatch\""));
    assert!(json.contains("\"expected_hash\": \"expected_hash_value\""));
    assert!(json.contains("\"actual_hash\": \"actual_hash_value\""));
}

#[test]
fn invalid_magic_returns_manifest_format_error() {
    let bundle_path = make_temp_dir("gv_bad_magic").join("bad.artl");
    let bad_data = b"XXXX\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00";
    fs::write(&bundle_path, bad_data).expect("write bad bundle");

    let result = Verifier::verify(&bundle_path);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.exit_code(), 3, "format error must have exit code 3");
}

#[test]
fn invalid_version_returns_manifest_format_error() {
    let bundle_path = make_temp_dir("gv_bad_ver").join("bad_ver.artl");
    // Correct magic, wrong version (99)
    let mut data = Vec::new();
    data.extend_from_slice(&MAGIC);
    data.extend_from_slice(&99u32.to_be_bytes());
    data.extend_from_slice(&0u64.to_le_bytes()); // manifest len = 0
    fs::write(&bundle_path, &data).expect("write bad version bundle");

    let result = Verifier::verify(&bundle_path);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().exit_code(), 3);
}

#[test]
fn golden_verify_report_json_is_stable() {
    let src = make_temp_dir("gv_golden_src");
    let bundle_path = make_temp_dir("gv_golden_bnd").join("golden.artl");

    fs::write(src.join("alpha.txt"), b"alpha content").unwrap();
    fs::write(src.join("beta.txt"), b"beta content").unwrap();

    BundlePacker::pack(&src, &bundle_path).expect("pack");
    let report = Verifier::verify(&bundle_path).expect("verify");
    let json = render_verify_report(&report);

    // The JSON must have the canonical structure
    let first_entry_pos = json.find("\"entry_count\"").expect("entry_count key");
    let ok_pos = json.find("\"ok\"").expect("ok key");
    let results_pos = json.find("\"results\"").expect("results key");
    // entry_count < ok < results (alphabetical key order)
    assert!(first_entry_pos < ok_pos, "entry_count before ok");
    assert!(ok_pos < results_pos, "ok before results");

    // Both files verified ok
    assert!(report.ok);
    assert_eq!(report.entry_count, 2);

    // Run verify again: identical output
    let report2 = Verifier::verify(&bundle_path).expect("verify 2");
    let json2 = render_verify_report(&report2);
    assert_eq!(json, json2, "verify report must be deterministic");
}

#[test]
fn verify_empty_bundle_is_ok() {
    let src = make_temp_dir("gv_empty_src");
    let bundle_path = make_temp_dir("gv_empty_bnd").join("empty.artl");

    BundlePacker::pack(&src, &bundle_path).expect("pack");
    let report = Verifier::verify(&bundle_path).expect("verify empty");

    assert!(report.ok);
    assert_eq!(report.entry_count, 0);
    assert!(report.results.is_empty());

    let json = render_verify_report(&report);
    assert!(json.contains("\"ok\": true"));
    assert!(json.contains("\"entry_count\": 0"));
    assert!(json.contains("\"results\": []") || json.contains("\"results\": [\n  ]"));
}
