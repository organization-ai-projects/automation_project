//! projects/products/stable/accounts/backend/src/router/tests/auth.rs
use common_json::{Json, JsonMap};
use protocol::{Command, CommandType, EventType, Metadata, Payload};
use security::TokenService;

use crate::{router::auth::handle_login, store::AccountManager};

async fn new_manager() -> AccountManager {
    let path = std::env::temp_dir().join(format!(
        "accounts_router_auth_{}_{}",
        std::process::id(),
        Metadata::current_timestamp_ms()
    ));
    AccountManager::load(path)
        .await
        .expect("manager should load")
}

fn login_command(payload: Json) -> Command {
    Command {
        metadata: Metadata::now(),
        command_type: CommandType::Execute,
        action: Some("accounts.login".to_string()),
        payload: Some(Payload {
            payload_type: Some("accounts/login".to_string()),
            payload: Some(payload),
        }),
    }
}

#[tokio::test]
async fn handle_login_rejects_blank_password() {
    let manager = new_manager().await;
    let token_service = TokenService::new_hs256("CHANGE_ME_CHANGE_ME_CHANGE_ME_32CHARS_MIN!!")
        .expect("token service should initialize");
    let metadata = Metadata::now();

    let mut map = JsonMap::new();
    map.insert(
        "user_id".to_string(),
        Json::String("00000000000000000000000000000001".to_string()),
    );
    map.insert("password".to_string(), Json::String("   ".to_string()));
    map.insert("role".to_string(), Json::Null);
    map.insert("duration_ms".to_string(), Json::Null);
    map.insert("session_id".to_string(), Json::Null);

    let event = handle_login(
        &metadata,
        &login_command(Json::Object(map)),
        &manager,
        &token_service,
    )
    .await;

    assert_eq!(event.event_type, EventType::Error);
    assert_eq!(event.message.as_deref(), Some("Invalid credentials"));
}

#[tokio::test]
async fn handle_login_returns_400_on_missing_payload() {
    let manager = new_manager().await;
    let token_service = TokenService::new_hs256("CHANGE_ME_CHANGE_ME_CHANGE_ME_32CHARS_MIN!!")
        .expect("token service should initialize");
    let metadata = Metadata::now();

    let command = Command {
        metadata: metadata.clone(),
        command_type: CommandType::Execute,
        action: Some("accounts.login".to_string()),
        payload: None,
    };

    let event = handle_login(&metadata, &command, &manager, &token_service).await;

    assert_eq!(event.event_type, EventType::Error);
    assert_eq!(event.message.as_deref(), Some("Missing payload"));
}
