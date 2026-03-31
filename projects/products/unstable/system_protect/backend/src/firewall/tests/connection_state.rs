use crate::firewall::connection_state::ConnectionState;

#[test]
fn connection_states_are_distinct() {
    assert_ne!(ConnectionState::New, ConnectionState::Established);
    assert_ne!(ConnectionState::Established, ConnectionState::Invalid);
}
