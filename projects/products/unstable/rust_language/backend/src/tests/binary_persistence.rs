use crate::engine::binary_encoder::BinaryEncoder;
use std::io::Write;

#[test]
fn write_then_read_roundtrip() {
    let rust_code = "fn main() { println!(\"hello\"); }";
    let format = BinaryEncoder::encode_rust_to_binary(rust_code).unwrap();
    assert!(format.validate_magic());
    assert_eq!(format.payload, rust_code.as_bytes());

    let dir = std::env::temp_dir().join("rhl_test_roundtrip");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("test.rhlb");

    BinaryEncoder::write_binary(&path, &format).unwrap();
    let loaded = BinaryEncoder::read_binary(&path).unwrap();

    assert_eq!(loaded.payload, format.payload);
    assert_eq!(loaded.checksum, format.checksum);
    assert!(loaded.validate_magic());

    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn read_binary_detects_corrupted_payload() {
    let rust_code = "fn foo() {}";
    let mut format = BinaryEncoder::encode_rust_to_binary(rust_code).unwrap();
    // Corrupt the checksum so it no longer matches the payload
    format.checksum = "0000000000000000000000000000000000000000000000000000000000000000".into();

    let dir = std::env::temp_dir().join("rhl_test_corrupt");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("corrupt.rhlb");

    BinaryEncoder::write_binary(&path, &format).unwrap();
    let result = BinaryEncoder::read_binary(&path);

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("checksum mismatch"),
        "expected checksum mismatch error, got: {err_msg}"
    );

    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn read_binary_rejects_wrong_magic() {
    let dir = std::env::temp_dir().join("rhl_test_bad_magic");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("bad_magic.rhlb");

    // Write a file with wrong magic bytes
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(b"BAAD_not_a_real_binary").unwrap();
    drop(file);

    let result = BinaryEncoder::read_binary(&path);
    assert!(result.is_err());

    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn encode_preserves_payload_content() {
    let code = "struct Foo { x: i32 }";
    let format = BinaryEncoder::encode_rust_to_binary(code).unwrap();
    let decoded = String::from_utf8(format.payload).unwrap();
    assert_eq!(decoded, code);
}
