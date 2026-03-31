use crate::header::Header;
use crate::{
    ApFileOptions, ContentType, ImageData, PixelFormat, read_binary, read_image, read_json,
    read_markdown, read_raw, read_ron, read_text, write_binary, write_image, write_json,
    write_markdown, write_ron, write_text,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn test_file_path(file_name: &str) -> (TempDir, PathBuf) {
    let temp_dir = tempfile::tempdir().unwrap();
    let path = temp_dir.path().join(file_name);
    (temp_dir, path)
}

// ---- Text round-trips ----

#[test]
fn test_text_round_trip() {
    let (_td, path) = test_file_path("text.apf");
    let opts = ApFileOptions::default();

    write_text(&path, "Hello, world!", &opts).unwrap();
    let loaded = read_text(&path, &opts).unwrap();
    assert_eq!(loaded, "Hello, world!");
}

#[test]
fn test_text_empty() {
    let (_td, path) = test_file_path("empty.apf");
    let opts = ApFileOptions::default();

    write_text(&path, "", &opts).unwrap();
    let loaded = read_text(&path, &opts).unwrap();
    assert_eq!(loaded, "");
}

#[test]
fn test_text_unicode() {
    let (_td, path) = test_file_path("unicode.apf");
    let opts = ApFileOptions::default();
    let text = "Héllo 世界 🌍";

    write_text(&path, text, &opts).unwrap();
    let loaded = read_text(&path, &opts).unwrap();
    assert_eq!(loaded, text);
}

// ---- Markdown round-trips ----

#[test]
fn test_markdown_round_trip() {
    let (_td, path) = test_file_path("doc.apf");
    let opts = ApFileOptions::default();
    let md = "# Title\n\nParagraph with **bold** text.\n";

    write_markdown(&path, md, &opts).unwrap();
    let loaded = read_markdown(&path, &opts).unwrap();
    assert_eq!(loaded, md);
}

// ---- JSON round-trips ----

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TestData {
    id: u64,
    name: String,
    values: Vec<i32>,
}

#[test]
fn test_json_round_trip() {
    let (_td, path) = test_file_path("data.apf");
    let opts = ApFileOptions::default();

    let data = TestData {
        id: 42,
        name: "test".to_string(),
        values: vec![1, 2, 3],
    };

    write_json(&path, &data, &opts).unwrap();
    let loaded: TestData = read_json(&path, &opts).unwrap();
    assert_eq!(loaded, data);
}

// ---- RON round-trips ----

#[test]
fn test_ron_round_trip() {
    let (_td, path) = test_file_path("data_ron.apf");
    let opts = ApFileOptions::default();

    let data = TestData {
        id: 99,
        name: "ron-test".to_string(),
        values: vec![10, 20],
    };

    write_ron(&path, &data, &opts).unwrap();
    let loaded: TestData = read_ron(&path, &opts).unwrap();
    assert_eq!(loaded, data);
}

// ---- Binary round-trips ----

#[test]
fn test_binary_round_trip() {
    let (_td, path) = test_file_path("raw.apf");
    let opts = ApFileOptions::default();
    let data = vec![0xDE, 0xAD, 0xBE, 0xEF];

    write_binary(&path, &data, &opts).unwrap();
    let loaded = read_binary(&path, &opts).unwrap();
    assert_eq!(loaded, data);
}

#[test]
fn test_binary_empty() {
    let (_td, path) = test_file_path("empty_bin.apf");
    let opts = ApFileOptions::default();

    write_binary(&path, &[], &opts).unwrap();
    let loaded = read_binary(&path, &opts).unwrap();
    assert!(loaded.is_empty());
}

// ---- Image round-trips ----

#[test]
fn test_image_round_trip() {
    let (_td, path) = test_file_path("image.apf");
    let opts = ApFileOptions::default();

    let pixels = vec![255u8; 4 * 4 * 4]; // 4x4 RGBA
    let image = ImageData::new(4, 4, PixelFormat::Rgba8, pixels).unwrap();

    write_image(&path, &image, &opts).unwrap();
    let loaded = read_image(&path, &opts).unwrap();
    assert_eq!(loaded, image);
}

#[test]
fn test_image_grayscale() {
    let (_td, path) = test_file_path("gray.apf");
    let opts = ApFileOptions::default();

    let pixels = vec![128u8; 8 * 8]; // 8x8 Gray8
    let image = ImageData::new(8, 8, PixelFormat::Gray8, pixels).unwrap();

    write_image(&path, &image, &opts).unwrap();
    let loaded = read_image(&path, &opts).unwrap();
    assert_eq!(loaded, image);
}

// ---- read_raw ----

#[test]
fn test_read_raw_returns_content_type() {
    let (_td, path) = test_file_path("raw_ct.apf");
    let opts = ApFileOptions::default();

    write_text(&path, "hi", &opts).unwrap();
    let (ct, payload) = read_raw(&path, &opts).unwrap();
    assert_eq!(ct, ContentType::PlainText);
    assert_eq!(payload, b"hi");
}

// ---- Content type mismatch ----

#[test]
fn test_content_type_mismatch_rejected() {
    let (_td, path) = test_file_path("mismatch.apf");
    let opts = ApFileOptions::default();

    write_text(&path, "hello", &opts).unwrap();

    // Attempting to read as binary should fail
    let result = read_binary(&path, &opts);
    assert!(result.is_err());
}

// ---- Schema mismatch ----

#[test]
fn test_schema_mismatch_rejected() {
    let (_td, path) = test_file_path("schema.apf");
    let write_opts = ApFileOptions {
        schema_id: 1,
        verify_checksum: true,
    };
    let read_opts = ApFileOptions {
        schema_id: 2,
        verify_checksum: true,
    };

    write_text(&path, "hello", &write_opts).unwrap();
    let result = read_text(&path, &read_opts);
    assert!(result.is_err());
}

// ---- Corruption detection ----

#[test]
fn test_corrupted_payload_detected() {
    let (_td, path) = test_file_path("corrupt.apf");
    let opts = ApFileOptions::default();

    write_text(&path, "hello world", &opts).unwrap();

    let mut contents = fs::read(&path).unwrap();
    if contents.len() > Header::SIZE {
        contents[Header::SIZE] ^= 0xFF;
        fs::write(&path, contents).unwrap();
    }

    let result = read_text(&path, &opts);
    assert!(result.is_err());
}

#[test]
fn test_checksum_validation_optional() {
    let (_td, path) = test_file_path("no_check.apf");
    let opts = ApFileOptions::default();

    write_text(&path, "hello world", &opts).unwrap();

    // Corrupt the checksum in the header
    let mut contents = fs::read(&path).unwrap();
    if contents.len() >= Header::SIZE {
        let cksum_offset = Header::SIZE - std::mem::size_of::<u64>();
        contents[cksum_offset] ^= 0xFF;
        fs::write(&path, contents).unwrap();
    }

    // Should fail with checksum verification
    let result_verify = read_text(
        &path,
        &ApFileOptions {
            schema_id: 0,
            verify_checksum: true,
        },
    );
    assert!(result_verify.is_err());

    // Should succeed without checksum verification
    let result_no_verify = read_text(
        &path,
        &ApFileOptions {
            schema_id: 0,
            verify_checksum: false,
        },
    );
    assert!(result_no_verify.is_ok());
}

// ---- File not found ----

#[test]
fn test_file_not_found() {
    let (_td, path) = test_file_path("nonexistent.apf");
    let opts = ApFileOptions::default();
    let result = read_text(&path, &opts);
    assert!(result.is_err());
}

// ---- Truncated header ----

#[test]
fn test_truncated_header() {
    let (_td, path) = test_file_path("truncated.apf");
    fs::write(&path, [1, 2, 3, 4, 5]).unwrap();
    let opts = ApFileOptions::default();
    let result = read_text(&path, &opts);
    assert!(result.is_err());
}

// ---- Overwrite existing ----

#[test]
fn test_overwrite_preserves_latest() {
    let (_td, path) = test_file_path("overwrite.apf");
    let opts = ApFileOptions::default();

    write_text(&path, "first", &opts).unwrap();
    write_text(&path, "second", &opts).unwrap();

    let loaded = read_text(&path, &opts).unwrap();
    assert_eq!(loaded, "second");
}

// ---- Large data ----

#[test]
fn test_large_text() {
    let (_td, path) = test_file_path("large.apf");
    let opts = ApFileOptions::default();
    let text = "x".repeat(100_000);

    write_text(&path, &text, &opts).unwrap();
    let loaded = read_text(&path, &opts).unwrap();
    assert_eq!(loaded, text);
}

// ---- Default options ----

#[test]
fn test_default_options() {
    let opts = ApFileOptions::default();
    assert_eq!(opts.schema_id, 0);
    assert!(opts.verify_checksum);
}
