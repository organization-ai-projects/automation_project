// projects/products/accounts/backend/src/main.rs
mod router;
mod store;

use std::{path::PathBuf, time::Duration};

use anyhow::Context;
use bytes::Bytes;
use common::Id128;
use futures_util::{SinkExt, StreamExt};
use protocol::{Command, CommandType, Metadata, Payload};
use security::{Role, TokenService};
use serde::Serialize;
use tokio_tungstenite::tungstenite::Message;
use tracing::{info, warn};

use crate::store::account_manager::AccountManager;

#[derive(Debug, Serialize)]
struct BackendHello {
    product_id: String,
    instance_id: String,
    capabilities: Vec<String>,
    routes: Vec<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let _data_dir = store::account_manager::AccountManager::default_data_dir();

    let engine_ws = std::env::var("ACCOUNTS_BACKEND_ENGINE_WS")
        .unwrap_or_else(|_| "ws://127.0.0.1:3030/ws".to_string());
    let jwt_secret = std::env::var("ACCOUNTS_BACKEND_JWT_SECRET")
        .or_else(|_| std::env::var("ENGINE_JWT_SECRET"))
        .unwrap_or_else(|_| "CHANGE_ME_CHANGE_ME_CHANGE_ME_32CHARS_MIN!!".to_string());

    let data_dir = std::env::var("ACCOUNTS_DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| AccountManager::default_data_dir());
    let account_manager = AccountManager::load(data_dir)
        .await
        .context("load accounts store")?;
    info!("Accounts data dir: {:?}", account_manager.data_dir());

    let token_service = TokenService::new_hs256(&jwt_secret).context("invalid jwt secret")?;
    let subject = Id128::new(1, None, None);
    let token = token_service
        .issue(subject, Role::Admin, 24 * 60 * 60 * 1000, None)
        .context("issue token")?;

    let ws_url = format!("{engine_ws}?token={token}");
    info!(%ws_url, "Connecting accounts backend");

    let (ws_stream, _) = tokio_tungstenite::connect_async(ws_url)
        .await
        .context("connect to engine ws")?;
    let (mut tx, mut rx) = ws_stream.split();

    let hello = BackendHello {
        product_id: "accounts".to_string(),
        instance_id: format!("accounts-{}", subject.to_hex()),
        capabilities: vec!["auth".to_string(), "accounts".to_string()],
        routes: vec![],
    };

    let payload = Payload {
        payload_type: Some("backend/hello".to_string()),
        payload: Some(common_json::to_value(&hello).context("hello json")?),
    };

    let cmd = Command {
        metadata: Metadata::now(),
        command_type: CommandType::Create,
        action: Some("backend.hello".to_string()),
        payload: Some(payload),
    };

    let cmd_json = common_json::to_string(&cmd).context("serialize command")?;
    let cmd_json_bytes = Bytes::from(cmd_json);
    tx.send(Message::Text(
        String::from_utf8_lossy(&cmd_json_bytes).to_string().into(),
    ))
    .await
    .context("send hello")?;
    info!("Accounts backend registered");

    loop {
        tokio::select! {
            msg = rx.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        match common_json::from_json_str::<Command>(&text) {
                            Ok(cmd) => {
                                let event = router::handle_command(cmd, &account_manager, &token_service).await;
                                if let Ok(event_json) = common_json::to_string(&event)
                                    && let Err(err) = tx.send(Message::Text(event_json.into())).await {
                                        warn!(%err, "Failed to send response event");
                                    }
                            }
                            Err(err) => {
                                warn!(%err, "Invalid command JSON");
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) => {
                        warn!("Engine closed WS connection");
                        break;
                    }
                    Some(Ok(_)) => {}
                    Some(Err(err)) => {
                        warn!(%err, "WS error");
                        break;
                    }
                    None => break,
                }
            }
            _ = tokio::time::sleep(Duration::from_secs(30)) => {
                if tx.send(Message::Ping(Bytes::from_static(&[]))).await.is_err() {
                    warn!("WS ping failed");
                    break;
                }
            }
        }
    }
    Ok(())
}
