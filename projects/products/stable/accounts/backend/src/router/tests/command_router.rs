//! projects/products/stable/accounts/backend/src/router/tests/command_router.rs
use protocol::{Command, CommandType, EventType, Metadata};
use security::TokenService;

use crate::{router::command_router::handle_command, store::AccountManager};

async fn new_manager() -> AccountManager {
    let path = std::env::temp_dir().join(format!(
        "accounts_router_command_{}_{}",
        std::process::id(),
        Metadata::current_timestamp_ms()
    ));
    AccountManager::load(path)
        .await
        .expect("manager should load")
}

fn token_service() -> TokenService {
    TokenService::new_hs256("CHANGE_ME_CHANGE_ME_CHANGE_ME_32CHARS_MIN!!")
        .expect("token service should initialize")
}

#[tokio::test]
async fn handle_command_rejects_missing_action() {
    let manager = new_manager().await;
    let service = token_service();

    let command = Command {
        metadata: Metadata::now(),
        command_type: CommandType::Execute,
        action: None,
        payload: None,
    };

    let event = handle_command(command, &manager, &service).await;

    assert_eq!(event.event_type, EventType::Error);
    assert_eq!(event.message.as_deref(), Some("Command action is missing"));
}

#[tokio::test]
async fn handle_command_rejects_unknown_action() {
    let manager = new_manager().await;
    let service = token_service();

    let command = Command {
        metadata: Metadata::now(),
        command_type: CommandType::Execute,
        action: Some("accounts.unknown".to_string()),
        payload: None,
    };

    let event = handle_command(command, &manager, &service).await;

    assert_eq!(event.event_type, EventType::Error);
    assert_eq!(event.message.as_deref(), Some("Unsupported action"));
}
