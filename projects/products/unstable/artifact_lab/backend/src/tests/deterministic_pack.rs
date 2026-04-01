use std::fs;
use std::path::PathBuf;

use crate::bundle::BundlePacker;

fn make_temp_dir(suffix: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("artifact_lab_det_{suffix}"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn populate_dir(dir: &PathBuf) {
    fs::write(dir.join("alpha.txt"), b"hello world").unwrap();
    fs::write(dir.join("beta.bin"), b"\x00\x01\x02\x03").unwrap();
    let sub = dir.join("subdir");
    fs::create_dir_all(&sub).unwrap();
    fs::write(sub.join("gamma.log"), b"log line 1\nlog line 2\n").unwrap();
}

#[test]
fn pack_same_dir_twice_produces_identical_bytes() {
    let src = make_temp_dir("src");
    let out1 = make_temp_dir("out1").join("bundle.artl");
    let out2 = make_temp_dir("out2").join("bundle.artl");

    populate_dir(&src);

    BundlePacker::pack(&src, &out1).expect("pack 1");
    BundlePacker::pack(&src, &out2).expect("pack 2");

    let bytes1 = fs::read(&out1).expect("read bundle 1");
    let bytes2 = fs::read(&out2).expect("read bundle 2");

    assert_eq!(
        bytes1.len(),
        bytes2.len(),
        "bundle sizes must match for identical input"
    );
    assert_eq!(bytes1, bytes2, "bundle bytes must be identical for identical input");
}

#[test]
fn pack_order_independent_of_readdir_order() {
    // Create the same files in two separate directories (same content, same names)
    let src_a = make_temp_dir("order_a");
    let src_b = make_temp_dir("order_b");
    let out_a = make_temp_dir("out_a").join("bundle.artl");
    let out_b = make_temp_dir("out_b").join("bundle.artl");

    // Populate both with identical content
    populate_dir(&src_a);
    populate_dir(&src_b);

    BundlePacker::pack(&src_a, &out_a).expect("pack a");
    BundlePacker::pack(&src_b, &out_b).expect("pack b");

    let bytes_a = fs::read(&out_a).expect("read bundle a");
    let bytes_b = fs::read(&out_b).expect("read bundle b");

    assert_eq!(bytes_a, bytes_b, "identical content must produce identical bundle");
}

#[test]
fn different_content_produces_different_bundle() {
    let src1 = make_temp_dir("diff_src1");
    let src2 = make_temp_dir("diff_src2");
    let out1 = make_temp_dir("diff_out1").join("bundle.artl");
    let out2 = make_temp_dir("diff_out2").join("bundle.artl");

    fs::write(src1.join("file.txt"), b"content A").unwrap();
    fs::write(src2.join("file.txt"), b"content B").unwrap();

    BundlePacker::pack(&src1, &out1).expect("pack 1");
    BundlePacker::pack(&src2, &out2).expect("pack 2");

    let bytes1 = fs::read(&out1).expect("read 1");
    let bytes2 = fs::read(&out2).expect("read 2");

    assert_ne!(bytes1, bytes2, "different content must produce different bundle");
}
