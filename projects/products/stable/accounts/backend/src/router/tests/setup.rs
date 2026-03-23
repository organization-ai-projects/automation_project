use common_json::{Json, JsonMap};
use protocol::{Command, CommandType, EventType, Metadata, Payload};
use protocol_accounts::SetupStatusResponse;

use crate::{
    router::setup::{handle_setup_admin, handle_setup_status},
    store::AccountManager,
};

async fn new_manager() -> AccountManager {
    let path = std::env::temp_dir().join(format!(
        "accounts_router_setup_{}_{}",
        std::process::id(),
        Metadata::current_timestamp_ms()
    ));
    AccountManager::load(path)
        .await
        .expect("manager should load")
}

fn setup_command(payload: Json) -> Command {
    Command {
        metadata: Metadata::now(),
        command_type: CommandType::Create,
        action: Some("accounts.setup_admin".to_string()),
        payload: Some(Payload {
            payload_type: Some("accounts/setup_admin".to_string()),
            payload: Some(payload),
        }),
    }
}

#[tokio::test]
async fn handle_setup_status_reports_setup_mode_when_empty() {
    let manager = new_manager().await;
    let metadata = Metadata::now();

    let event = handle_setup_status(&metadata, &manager).await;

    assert_eq!(event.event_type, EventType::Payload);
    let payload = event.payload.expect("payload should exist");
    let payload_value = payload.payload.expect("payload value should exist");
    let parsed: SetupStatusResponse =
        common_json::from_value(payload_value).expect("payload should parse");
    assert!(parsed.setup_mode);
}

#[tokio::test]
async fn handle_setup_admin_requires_non_empty_claim() {
    let manager = new_manager().await;
    let metadata = Metadata::now();

    let mut map = JsonMap::new();
    map.insert(
        "user_id".to_string(),
        Json::String("00000000000000000000000000000001".to_string()),
    );
    map.insert("password".to_string(), Json::String("pw-demo".to_string()));
    map.insert("claim".to_string(), Json::String(" ".to_string()));
    let command = setup_command(Json::Object(map));

    let event = handle_setup_admin(&metadata, &command, &manager).await;

    assert_eq!(event.event_type, EventType::Error);
    assert_eq!(event.message.as_deref(), Some("Missing setup claim"));
}
