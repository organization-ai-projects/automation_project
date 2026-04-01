use std::fs;
use std::path::PathBuf;

use crate::bundle::{BundlePacker, BundleUnpacker};
use crate::verify::Verifier;

fn make_temp_dir(suffix: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("artifact_lab_rt_{suffix}"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn populate_dir(dir: &PathBuf) {
    fs::write(dir.join("readme.txt"), b"hello world").unwrap();
    fs::write(dir.join("data.bin"), b"\xDE\xAD\xBE\xEF\xCA\xFE").unwrap();
    let sub = dir.join("logs");
    fs::create_dir_all(&sub).unwrap();
    fs::write(sub.join("run.log"), b"run log line\n").unwrap();
    fs::write(sub.join("err.log"), b"").unwrap(); // empty file
}

#[test]
fn pack_unpack_round_trip_preserves_all_files() {
    let src = make_temp_dir("src_rt");
    let bundle = make_temp_dir("bundle_rt").join("test.artl");
    let dest = make_temp_dir("dest_rt");

    populate_dir(&src);

    BundlePacker::pack(&src, &bundle).expect("pack");
    BundleUnpacker::unpack(&bundle, &dest).expect("unpack");

    // Verify all files are present and content matches
    for (rel, expected) in [
        ("readme.txt", b"hello world".as_ref()),
        ("data.bin", b"\xDE\xAD\xBE\xEF\xCA\xFE".as_ref()),
        ("logs/run.log", b"run log line\n".as_ref()),
        ("logs/err.log", b"".as_ref()),
    ] {
        let dest_file = dest.join(rel.replace('/', std::path::MAIN_SEPARATOR_STR));
        assert!(dest_file.exists(), "file '{rel}' should exist after unpack");
        let actual = fs::read(&dest_file).unwrap();
        assert_eq!(actual, expected, "content mismatch for '{rel}'");
    }
}

#[test]
fn pack_unpack_verify_succeeds() {
    let src = make_temp_dir("src_v");
    let bundle = make_temp_dir("bundle_v").join("v.artl");
    let dest = make_temp_dir("dest_v");

    populate_dir(&src);

    BundlePacker::pack(&src, &bundle).expect("pack");
    BundleUnpacker::unpack(&bundle, &dest).expect("unpack");

    let report = Verifier::verify(&bundle).expect("verify");
    assert!(report.ok, "verify must pass after clean pack");
    assert_eq!(report.entry_count, 4);
    for r in &report.results {
        assert_eq!(r.status_label(), "ok", "entry '{}' must be ok", r.path);
    }
}

#[test]
fn empty_directory_produces_empty_bundle() {
    let src = make_temp_dir("src_empty");
    let bundle = make_temp_dir("bundle_empty").join("empty.artl");
    let dest = make_temp_dir("dest_empty");

    BundlePacker::pack(&src, &bundle).expect("pack empty");
    BundleUnpacker::unpack(&bundle, &dest).expect("unpack empty");

    let report = Verifier::verify(&bundle).expect("verify empty");
    assert!(report.ok);
    assert_eq!(report.entry_count, 0);
}

#[test]
fn unpacked_files_have_correct_content_hashes() {
    use crate::hash::FileHasher;

    let src = make_temp_dir("src_hash");
    let bundle = make_temp_dir("bundle_hash").join("hash.artl");
    let dest = make_temp_dir("dest_hash");

    fs::write(src.join("a.txt"), b"content for a").unwrap();
    fs::write(src.join("b.txt"), b"content for b").unwrap();

    BundlePacker::pack(&src, &bundle).expect("pack");
    BundleUnpacker::unpack(&bundle, &dest).expect("unpack");

    let hash_a = FileHasher::hash_bytes(b"content for a");
    let hash_b = FileHasher::hash_bytes(b"content for b");

    let actual_a = fs::read(dest.join("a.txt")).unwrap();
    let actual_b = fs::read(dest.join("b.txt")).unwrap();

    assert_eq!(FileHasher::hash_bytes(&actual_a), hash_a);
    assert_eq!(FileHasher::hash_bytes(&actual_b), hash_b);
}
