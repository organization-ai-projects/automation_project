use crate::diagnostics::error::Error;

#[test]
fn serialization_error_display() {
    let err = Error::Serialization("test".to_string());
    assert_eq!(format!("{err}"), "serialization error: test");
}

#[test]
fn deserialization_error_display() {
    let err = Error::Deserialization("bad json".to_string());
    assert_eq!(format!("{err}"), "deserialization error: bad json");
}

#[test]
fn checksum_mismatch_display() {
    let err = Error::ChecksumMismatch {
        expected: "abc".to_string(),
        actual: "def".to_string(),
    };
    let msg = format!("{err}");
    assert!(msg.contains("abc"));
    assert!(msg.contains("def"));
}

#[test]
fn plugin_not_found_display() {
    let err = Error::PluginNotFound("test_plugin".to_string());
    assert!(format!("{err}").contains("test_plugin"));
}
