// projects/libraries/layers/domain/identity/src/tests/user_id.rs
use common::common_id::CommonID;
use common::custom_uuid::Id128;
use protocol::ProtocolId;

use crate::UserId;

use super::helpers::create_test_user_id;

#[test]
fn test_user_id_new_valid() {
    let id = Id128::from_bytes_unchecked([1u8; 16]);
    let user_id = UserId::new(ProtocolId::new(id)).expect("failed to create user id");
    assert_eq!(user_id.value(), ProtocolId::new(id));
}

#[test]
fn test_user_id_new_invalid() {
    let invalid_id = Id128::from_bytes_unchecked([0u8; 16]);
    assert!(UserId::new(ProtocolId::new(invalid_id)).is_err());
}

#[test]
fn test_user_id_equality() {
    let user_id1 = create_test_user_id(1);
    let user_id2 = create_test_user_id(1);
    let user_id3 = create_test_user_id(2);

    assert_eq!(user_id1, user_id2);
    assert_ne!(user_id1, user_id3);
}

#[test]
fn test_user_id_display() {
    let id = Id128::from_bytes_unchecked([42u8; 16]);
    let user_id = UserId::new(ProtocolId::new(id)).expect("failed to create user id");
    assert_eq!(format!("{}", user_id), id.to_string());
}

#[test]
fn test_validate_user_id() {
    // Check that zero bytes are invalid
    let invalid_id = Id128::from_bytes_unchecked([0u8; 16]);
    assert!(!CommonID::is_valid(invalid_id));
}

#[test]
fn test_user_id_new_with_id128() {
    // Test directly with Id128
    let valid_id = Id128::from_bytes_unchecked([1u8; 16]);
    let invalid_id = Id128::from_bytes_unchecked([0u8; 16]);

    assert!(CommonID::is_valid(valid_id));
    assert!(!CommonID::is_valid(invalid_id));
}
