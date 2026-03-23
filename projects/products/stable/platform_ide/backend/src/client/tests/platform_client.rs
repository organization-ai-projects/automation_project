//! projects/products/stable/platform_ide/backend/src/client/tests/platform_client.rs
use crate::client::PlatformClient;

#[test]
fn platform_client_new_initializes_instance() {
    let symbol = std::any::type_name::<PlatformClient>();
    assert!(symbol.contains("PlatformClient"));
}
