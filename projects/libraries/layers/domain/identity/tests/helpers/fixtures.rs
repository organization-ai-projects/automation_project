use common::Id128;
use identity::UserId;
use protocol::ProtocolId;

/// Creates a test UserId from a byte pattern.
/// This standardizes UserId creation across all tests.
pub fn create_test_user_id(byte: u8) -> UserId {
    let id = Id128::from_bytes_unchecked([byte; 16]);
    UserId::new(ProtocolId::new(id)).expect("failed to create test user id")
}
