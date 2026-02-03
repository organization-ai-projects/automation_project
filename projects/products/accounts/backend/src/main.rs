// projects/products/accounts/backend/src/main.rs
mod router;
mod store;

use std::str::FromStr;
use std::{path::PathBuf, time::Duration};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::Context;
use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use protocol::protocol_id::ProtocolId;
use protocol::{Command, CommandType, Metadata, Payload};
use security::{Role, TokenService};
use serde::Serialize;
use tokio_tungstenite::tungstenite::Message;
use tracing::{info, warn};

use crate::store::account_manager::AccountManager;

async fn flush_and_stop_periodic_task(
    flush_handle: &tokio::task::JoinHandle<()>,
    manager: &AccountManager,
    context: &str,
) {
    flush_handle.abort();
    if let Err(err) = manager.flush_if_dirty().await {
        warn!(%err, "Failed to flush {}", context);
    }
}

#[derive(Debug, Serialize)]
struct BackendHello {
    product_id: ProtocolId,
    instance_id: ProtocolId,
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

    // Get configurable flush interval (default: 5 minutes)
    let flush_interval_secs = std::env::var("ACCOUNTS_FLUSH_INTERVAL_SECS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(300); // 5 minutes default
    info!("Login metadata flush interval: {}s", flush_interval_secs);

    // Spawn periodic flush task
    let flush_manager = account_manager.clone();
    let flush_handle = tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(flush_interval_secs));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            interval.tick().await;
            if let Err(err) = flush_manager.flush_if_dirty().await {
                warn!(%err, "Failed to flush login metadata");
            }
        }
    });

    let token_service = TokenService::new_hs256(&jwt_secret).context("invalid jwt secret")?;
    let subject = ProtocolId::default();
    let token = token_service
        .issue(subject, Role::Admin, 24 * 60 * 60 * 1000, None)
        .context("issue token")?;

    let product_id_raw = std::env::var("ENGINE_ACCOUNTS_PRODUCT_ID")
        .or_else(|_| std::env::var("ACCOUNTS_PRODUCT_ID"))
        .context("ACCOUNTS_PRODUCT_ID env var is required")?;
    let product_id =
        ProtocolId::from_str(&product_id_raw).context("invalid ACCOUNTS_PRODUCT_ID")?;

    let instance_id = subject;

    let ws_url = format!("{engine_ws}?token={token}");
    info!(%ws_url, "Connecting accounts backend");

    let (ws_stream, _) = tokio_tungstenite::connect_async(ws_url)
        .await
        .context("connect to engine ws")?;
    let (mut tx, mut rx) = ws_stream.split();

    let hello = BackendHello {
        product_id,
        instance_id,
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

    // Set up graceful shutdown signal handler
    let shutdown_manager = account_manager.clone();
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let shutdown_flag_clone = shutdown_flag.clone();
    
    tokio::spawn(async move {
        #[cfg(unix)]
        {
            let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("failed to create SIGTERM handler");
            let mut sigint = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())
                .expect("failed to create SIGINT handler");
            tokio::select! {
                _ = sigterm.recv() => info!("Received SIGTERM"),
                _ = sigint.recv() => info!("Received SIGINT"),
            }
        }
        #[cfg(not(unix))]
        {
            tokio::signal::ctrl_c().await.expect("failed to listen for ctrl-c");
            info!("Received Ctrl-C");
        }
        shutdown_flag_clone.store(true, Ordering::Relaxed);
    });

    loop {
        // Check for shutdown signal
        if shutdown_flag.load(Ordering::Relaxed) {
            info!("Shutting down gracefully...");
            flush_and_stop_periodic_task(&flush_handle, &shutdown_manager, "on shutdown").await;
            break;
        }
        
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
    
    // Final flush on exit
    info!("Flushing login metadata before exit...");
    flush_and_stop_periodic_task(&flush_handle, &account_manager, "on exit").await;
    
    Ok(())
}
