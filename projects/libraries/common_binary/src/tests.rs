use crate::{BinaryDecode, BinaryEncode, BinaryError, BinaryOptions, read_binary, write_binary};
use std::fs;

#[derive(Debug, PartialEq, Clone, bincode::Encode, bincode::Decode)]
struct TestData {
    id: u64,
    name: String,
    values: Vec<i32>,
}

impl BinaryEncode for TestData {
    fn encode_binary(&self, out: &mut Vec<u8>) -> Result<(), BinaryError> {
        bincode::encode_into_std_write(self, out, bincode::config::standard())
            .map(|_| ())
            .map_err(|e| BinaryError::Encode(Box::leak(e.to_string().into_boxed_str())))
    }
}

impl BinaryDecode for TestData {
    fn decode_binary(input: &[u8]) -> Result<Self, BinaryError> {
        bincode::decode_from_slice(input, bincode::config::standard())
            .map(|(data, _)| data)
            .map_err(|e| BinaryError::Decode(Box::leak(e.to_string().into_boxed_str())))
    }
}

#[test]
fn test_write_read_round_trip() {
    let temp_dir = std::env::temp_dir();
    let path = temp_dir.join("test_round_trip.bin");

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

    // Write
    write_binary(&data, &path, &opts).unwrap();

    // Read
    let loaded: TestData = read_binary(&path, &opts).unwrap();

    // Verify
    assert_eq!(data, loaded);

    // Cleanup
    let _ = fs::remove_file(&path);
}

#[test]
fn test_invalid_magic_rejected() {
    let temp_dir = std::env::temp_dir();
    let path = temp_dir.join("test_invalid_magic.bin");

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

    // Write with one magic
    write_binary(&data, &path, &write_opts).unwrap();

    // Try to read with different magic
    let result: Result<TestData, BinaryError> = read_binary(&path, &read_opts);
    assert!(matches!(result, Err(BinaryError::Incompatible(_))));

    if let Err(BinaryError::Incompatible(msg)) = result {
        assert_eq!(msg, "Magic mismatch");
    }

    // Cleanup
    let _ = fs::remove_file(&path);
}

#[test]
fn test_invalid_schema_id_rejected() {
    let temp_dir = std::env::temp_dir();
    let path = temp_dir.join("test_invalid_schema.bin");

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

    // Write with one schema_id
    write_binary(&data, &path, &write_opts).unwrap();

    // Try to read with different schema_id
    let result: Result<TestData, BinaryError> = read_binary(&path, &read_opts);
    assert!(matches!(result, Err(BinaryError::Incompatible(_))));

    if let Err(BinaryError::Incompatible(msg)) = result {
        assert_eq!(msg, "Schema ID mismatch");
    }

    // Cleanup
    let _ = fs::remove_file(&path);
}

#[test]
fn test_corrupted_payload_detected() {
    let temp_dir = std::env::temp_dir();
    let path = temp_dir.join("test_corrupted.bin");

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

    // Write valid data
    write_binary(&data, &path, &opts).unwrap();

    // Corrupt the file by modifying a byte in the payload
    let mut contents = fs::read(&path).unwrap();
    if contents.len() > 32 {
        contents[32] ^= 0xFF; // Flip all bits in first payload byte
        fs::write(&path, contents).unwrap();
    }

    // Try to read corrupted data
    let result: Result<TestData, BinaryError> = read_binary(&path, &opts);
    assert!(matches!(result, Err(BinaryError::Corrupt(_))));

    if let Err(BinaryError::Corrupt(msg)) = result {
        assert_eq!(msg, "Checksum mismatch");
    }

    // Cleanup
    let _ = fs::remove_file(&path);
}

#[test]
fn test_checksum_validation_optional() {
    let temp_dir = std::env::temp_dir();
    let path = temp_dir.join("test_no_checksum.bin");

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
        verify_checksum: false, // Don't verify checksum
    };

    // Write valid data
    write_binary(&data, &path, &write_opts).unwrap();

    // Corrupt the file by changing the checksum in the header (not the payload)
    let mut contents = fs::read(&path).unwrap();
    if contents.len() >= 32 {
        // Corrupt the checksum field (last 8 bytes of header, bytes 24-31)
        contents[24] ^= 0xFF;
        fs::write(&path, contents).unwrap();
    }

    // Reading with checksum verification should fail
    let result_verify: Result<TestData, BinaryError> = read_binary(&path, &read_opts_verify);
    assert!(matches!(result_verify, Err(BinaryError::Corrupt(_))));

    // Reading without checksum verification should succeed (payload is still valid)
    let result_no_verify: Result<TestData, BinaryError> = read_binary(&path, &read_opts_no_verify);
    assert!(result_no_verify.is_ok());

    // Cleanup
    let _ = fs::remove_file(&path);
}

#[test]
fn test_empty_data() {
    let temp_dir = std::env::temp_dir();
    let path = temp_dir.join("test_empty.bin");

    #[derive(Debug, PartialEq, bincode::Encode, bincode::Decode)]
    struct EmptyData {}

    impl BinaryEncode for EmptyData {
        fn encode_binary(&self, out: &mut Vec<u8>) -> Result<(), BinaryError> {
            bincode::encode_into_std_write(self, out, bincode::config::standard())
                .map(|_| ())
                .map_err(|e| BinaryError::Encode(Box::leak(e.to_string().into_boxed_str())))
        }
    }

    impl BinaryDecode for EmptyData {
        fn decode_binary(input: &[u8]) -> Result<Self, BinaryError> {
            bincode::decode_from_slice(input, bincode::config::standard())
                .map(|(data, _)| data)
                .map_err(|e| BinaryError::Decode(Box::leak(e.to_string().into_boxed_str())))
        }
    }

    let data = EmptyData {};

    let opts = BinaryOptions {
        magic: *b"TEST",
        container_version: 1,
        schema_id: 100,
        verify_checksum: true,
    };

    // Write
    write_binary(&data, &path, &opts).unwrap();

    // Read
    let loaded: EmptyData = read_binary(&path, &opts).unwrap();

    // Verify
    assert_eq!(data, loaded);

    // Cleanup
    let _ = fs::remove_file(&path);
}

#[test]
fn test_large_data() {
    let temp_dir = std::env::temp_dir();
    let path = temp_dir.join("test_large.bin");

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

    // Write
    write_binary(&data, &path, &opts).unwrap();

    // Read
    let loaded: TestData = read_binary(&path, &opts).unwrap();

    // Verify
    assert_eq!(data, loaded);

    // Cleanup
    let _ = fs::remove_file(&path);
}

#[test]
fn test_different_container_versions() {
    let temp_dir = std::env::temp_dir();
    let path = temp_dir.join("test_version.bin");

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

    // Write with version 1
    write_binary(&data, &path, &write_opts).unwrap();

    // Try to read with version 2
    let result: Result<TestData, BinaryError> = read_binary(&path, &read_opts);
    assert!(matches!(result, Err(BinaryError::Incompatible(_))));

    // Cleanup
    let _ = fs::remove_file(&path);
}

#[test]
fn test_file_not_found() {
    let temp_dir = std::env::temp_dir();
    let path = temp_dir.join("nonexistent_file.bin");

    let opts = BinaryOptions::default();

    let result: Result<TestData, BinaryError> = read_binary(&path, &opts);
    assert!(matches!(result, Err(BinaryError::Io(_))));
}

#[test]
fn test_truncated_header() {
    let temp_dir = std::env::temp_dir();
    let path = temp_dir.join("test_truncated.bin");

    // Write a file with only partial header
    fs::write(&path, [1, 2, 3, 4, 5]).unwrap();

    let opts = BinaryOptions::default();

    let result: Result<TestData, BinaryError> = read_binary(&path, &opts);
    assert!(matches!(result, Err(BinaryError::Corrupt(_))));

    // Cleanup
    let _ = fs::remove_file(&path);
}

#[test]
fn test_default_options() {
    let opts = BinaryOptions::default();
    assert_eq!(opts.magic, *b"CBIN");
    assert_eq!(opts.container_version, 1);
    assert_eq!(opts.schema_id, 0);
    assert!(opts.verify_checksum);
}
