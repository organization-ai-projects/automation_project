use crate::{BinaryError, BinaryOptions, read_binary, write_binary};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

// Test data structure with serde derives
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TestData {
    id: u64,
    name: String,
    values: Vec<i32>,
}

fn test_file_path(file_name: &str) -> (TempDir, PathBuf) {
    let temp_dir = tempfile::tempdir().unwrap();
    let path = temp_dir.path().join(file_name);
    (temp_dir, path)
}

// BinaryEncode and BinaryDecode are automatically implemented via blanket impl

#[test]
fn test_write_read_round_trip() {
    let (_temp_dir, path) = test_file_path("test_round_trip.bin");

    let data = TestData {
        id: 42,
        name: "test".to_string(),
        values: vec![1, 2, 3, 4, 5],
    };

    let opts = BinaryOptions {
        magic: *b"TEST",
        container_version: 1,
        schema_id: 100,
        verify_checksum: true,
    };

    write_binary(&data, &path, &opts).unwrap();
    let loaded: TestData = read_binary(&path, &opts).unwrap();

    assert_eq!(data, loaded);
}

#[test]
fn test_invalid_magic_rejected() {
    let (_temp_dir, path) = test_file_path("test_invalid_magic.bin");

    let data = TestData {
        id: 42,
        name: "test".to_string(),
        values: vec![1, 2, 3],
    };

    let write_opts = BinaryOptions {
        magic: *b"TST1",
        container_version: 1,
        schema_id: 100,
        verify_checksum: true,
    };

    let read_opts = BinaryOptions {
        magic: *b"TST2",
        container_version: 1,
        schema_id: 100,
        verify_checksum: true,
    };

    write_binary(&data, &path, &write_opts).unwrap();

    let result: Result<TestData, BinaryError> = read_binary(&path, &read_opts);
    assert!(matches!(result, Err(BinaryError::Incompatible(_))));

    if let Err(BinaryError::Incompatible(msg)) = result {
        assert_eq!(msg, "Magic mismatch");
    }
}

#[test]
fn test_invalid_schema_id_rejected() {
    let (_temp_dir, path) = test_file_path("test_invalid_schema.bin");

    let data = TestData {
        id: 42,
        name: "test".to_string(),
        values: vec![1, 2, 3],
    };

    let write_opts = BinaryOptions {
        magic: *b"TEST",
        container_version: 1,
        schema_id: 100,
        verify_checksum: true,
    };

    let read_opts = BinaryOptions {
        magic: *b"TEST",
        container_version: 1,
        schema_id: 200,
        verify_checksum: true,
    };

    write_binary(&data, &path, &write_opts).unwrap();

    let result: Result<TestData, BinaryError> = read_binary(&path, &read_opts);
    assert!(matches!(result, Err(BinaryError::Incompatible(_))));

    if let Err(BinaryError::Incompatible(msg)) = result {
        assert_eq!(msg, "Schema ID mismatch");
    }
}

#[test]
fn test_corrupted_payload_detected() {
    let (_temp_dir, path) = test_file_path("test_corrupted.bin");

    let data = TestData {
        id: 42,
        name: "test".to_string(),
        values: vec![1, 2, 3],
    };

    let opts = BinaryOptions {
        magic: *b"TEST",
        container_version: 1,
        schema_id: 100,
        verify_checksum: true,
    };

    write_binary(&data, &path, &opts).unwrap();

    let mut contents = fs::read(&path).unwrap();
    if contents.len() > 32 {
        contents[32] ^= 0xFF;
        fs::write(&path, contents).unwrap();
    }

    let result: Result<TestData, BinaryError> = read_binary(&path, &opts);
    assert!(matches!(result, Err(BinaryError::Corrupt(_))));

    if let Err(BinaryError::Corrupt(msg)) = result {
        assert_eq!(msg, "Checksum mismatch");
    }
}

#[test]
fn test_checksum_validation_optional() {
    let (_temp_dir, path) = test_file_path("test_no_checksum.bin");

    let data = TestData {
        id: 42,
        name: "test".to_string(),
        values: vec![1, 2, 3],
    };

    let write_opts = BinaryOptions {
        magic: *b"TEST",
        container_version: 1,
        schema_id: 100,
        verify_checksum: true,
    };

    let read_opts_verify = BinaryOptions {
        magic: *b"TEST",
        container_version: 1,
        schema_id: 100,
        verify_checksum: true,
    };

    let read_opts_no_verify = BinaryOptions {
        magic: *b"TEST",
        container_version: 1,
        schema_id: 100,
        verify_checksum: false,
    };

    write_binary(&data, &path, &write_opts).unwrap();

    let mut contents = fs::read(&path).unwrap();
    if contents.len() >= 32 {
        contents[24] ^= 0xFF;
        fs::write(&path, contents).unwrap();
    }

    let result_verify: Result<TestData, BinaryError> = read_binary(&path, &read_opts_verify);
    assert!(matches!(result_verify, Err(BinaryError::Corrupt(_))));

    let result_no_verify: Result<TestData, BinaryError> = read_binary(&path, &read_opts_no_verify);
    assert!(result_no_verify.is_ok());
}

#[test]
fn test_empty_data() {
    let (_temp_dir, path) = test_file_path("test_empty.bin");

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct EmptyData {}

    let data = EmptyData {};

    let opts = BinaryOptions {
        magic: *b"TEST",
        container_version: 1,
        schema_id: 100,
        verify_checksum: true,
    };

    write_binary(&data, &path, &opts).unwrap();
    let loaded: EmptyData = read_binary(&path, &opts).unwrap();

    assert_eq!(data, loaded);
}

#[test]
fn test_large_data() {
    let (_temp_dir, path) = test_file_path("test_large.bin");

    let data = TestData {
        id: u64::MAX,
        name: "x".repeat(10000),
        values: (0..10000).collect(),
    };

    let opts = BinaryOptions {
        magic: *b"TEST",
        container_version: 1,
        schema_id: 100,
        verify_checksum: true,
    };

    write_binary(&data, &path, &opts).unwrap();
    let loaded: TestData = read_binary(&path, &opts).unwrap();

    assert_eq!(data, loaded);
}

#[test]
fn test_different_container_versions() {
    let (_temp_dir, path) = test_file_path("test_version.bin");

    let data = TestData {
        id: 42,
        name: "test".to_string(),
        values: vec![1, 2, 3],
    };

    let write_opts = BinaryOptions {
        magic: *b"TEST",
        container_version: 1,
        schema_id: 100,
        verify_checksum: true,
    };

    let read_opts = BinaryOptions {
        magic: *b"TEST",
        container_version: 2,
        schema_id: 100,
        verify_checksum: true,
    };

    write_binary(&data, &path, &write_opts).unwrap();

    let result: Result<TestData, BinaryError> = read_binary(&path, &read_opts);
    assert!(matches!(result, Err(BinaryError::Incompatible(_))));
}

#[test]
fn test_file_not_found() {
    let (_temp_dir, path) = test_file_path("nonexistent_file.bin");

    let opts = BinaryOptions::default();

    let result: Result<TestData, BinaryError> = read_binary(&path, &opts);
    assert!(matches!(result, Err(BinaryError::Io(_))));
}

#[test]
fn test_truncated_header() {
    let (_temp_dir, path) = test_file_path("test_truncated.bin");

    fs::write(&path, [1, 2, 3, 4, 5]).unwrap();

    let opts = BinaryOptions::default();

    let result: Result<TestData, BinaryError> = read_binary(&path, &opts);
    assert!(matches!(result, Err(BinaryError::Corrupt(_))));
}

#[test]
fn test_payload_length_overflow_rejected() {
    let (_temp_dir, path) = test_file_path("test_payload_overflow.bin");

    let mut header = [0u8; 32];
    header[0..4].copy_from_slice(b"CBIN");
    header[4..6].copy_from_slice(&1u16.to_le_bytes());
    header[8..16].copy_from_slice(&0u64.to_le_bytes());
    header[16..24].copy_from_slice(&u64::MAX.to_le_bytes());
    header[24..32].copy_from_slice(&0u64.to_le_bytes());
    fs::write(&path, header).unwrap();

    let opts = BinaryOptions::default();
    let result: Result<TestData, BinaryError> = read_binary(&path, &opts);
    assert!(matches!(result, Err(BinaryError::Corrupt(_))));
}

#[test]
fn test_default_options() {
    let opts = BinaryOptions::default();
    assert_eq!(opts.magic, *b"CBIN");
    assert_eq!(opts.container_version, 1);
    assert_eq!(opts.schema_id, 0);
    assert!(opts.verify_checksum);
}
