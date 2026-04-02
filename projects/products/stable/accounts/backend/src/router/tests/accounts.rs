//! projects/products/stable/accounts/backend/src/router/tests/accounts.rs
use common_json::{Json, JsonMap};
use protocol::{Command, CommandType, EventType, Metadata, Payload};

use crate::{
    router::accounts::{handle_create_user, handle_get_user, handle_list_users},
    store::AccountManager,
};

async fn new_manager() -> AccountManager {
    let path = std::env::temp_dir().join(format!(
        "accounts_router_accounts_{}_{}",
        std::process::id(),
        Metadata::current_timestamp_ms()
    ));
    AccountManager::load(path)
        .await
        .expect("manager should load")
}

fn command(action: &str, payload: Json) -> Command {
    Command {
        metadata: Metadata::now(),
        command_type: CommandType::Create,
        action: Some(action.to_string()),
        payload: Some(Payload {
            payload_type: Some("accounts/request".to_string()),
            payload: Some(payload),
        }),
    }
}

#[tokio::test]
async fn handle_get_user_returns_400_when_user_id_missing() {
    let manager = new_manager().await;
    let metadata = Metadata::now();
    let payload = Json::Object(JsonMap::new());
    let cmd = command("accounts.get", payload);

    let event = handle_get_user(&metadata, &cmd, &manager).await;

    assert_eq!(event.event_type, EventType::Error);
    assert_eq!(event.message.as_deref(), Some("Missing user_id"));
}

#[tokio::test]
async fn handle_list_users_returns_payload_event() {
    let manager = new_manager().await;
    let metadata = Metadata::now();

    let event = handle_list_users(&metadata, &manager).await;

    assert_eq!(event.event_type, EventType::Payload);
    assert_eq!(event.name, "AccountsList");
    assert_eq!(
        event
            .payload
            .as_ref()
            .and_then(|payload| payload.payload_type.as_deref()),
        Some("accounts/list")
    );
}

#[tokio::test]
async fn handle_create_user_rejects_invalid_role() {
    let manager = new_manager().await;
    let metadata = Metadata::now();

    let mut map = JsonMap::new();
    map.insert(
        "user_id".to_string(),
        Json::String("00000000000000000000000000000001".to_string()),
    );
    map.insert("password".to_string(), Json::String("pw-demo".to_string()));
    map.insert("role".to_string(), Json::String("not_a_role".to_string()));
    map.insert("permissions".to_string(), Json::Array(vec![]));
    let cmd = command("accounts.create", Json::Object(map));

    let event = handle_create_user(&metadata, &cmd, &manager).await;

    assert_eq!(event.event_type, EventType::Error);
    assert_eq!(event.message.as_deref(), Some("Invalid role"));
}
