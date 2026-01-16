// projects/products/core/watcher/src/main.rs
use std::sync::Arc;

use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use watcher::{config::WatcherConfig, logger, supervisor};

fn env_var(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

#[tokio::main]
async fn main() {
    println!("Watcher starting...");

    // 1) Config path (env override)
    let config_path = env_var("WATCHER_CONFIG").unwrap_or_else(|| "watcher.toml".to_string());

    let config = match WatcherConfig::load_from_file(&config_path) {
        Ok(cfg) => Arc::new(cfg),
        Err(e) => {
            eprintln!("Failed to load configuration '{}': {}", config_path, e);
            std::process::exit(1);
        }
    };

    // 2) Logger
    logger::initialize_logger(
        config.logging.log_file.to_str().unwrap_or("watcher.log"),
        config.logging.log_level.as_str(),
    );

    // 3) Cancellation: allows for a clean shutdown
    let shutdown = CancellationToken::new();

    // 4) Launch all supervisions
    let mut set = JoinSet::new();
    let supervisor = supervisor::Supervisor::new();

    for component in config.components.clone() {
        let shutdown = shutdown.clone();
        let supervisor = supervisor.clone();
        set.spawn(async move {
            supervisor.supervise_component(component, shutdown).await;
        });
    }

    // 5) Wait for Ctrl+C (or external shutdown)
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            log::warn!("Watcher: Ctrl+C received, shutting down...");
        }
        // If desired: detect if a task dies (this shouldn't happen)
        res = set.join_next() => {
            if let Some(Err(e)) = res {
                log::error!("Watcher: a supervision task panicked: {}", e);
            } else {
                log::warn!("Watcher: a supervision task exited unexpectedly");
            }
        }
    }

    // 6) Trigger shutdown, then wait for tasks to finish (with timeout)
    shutdown.cancel();

    let grace = tokio::time::sleep(std::time::Duration::from_secs(5));
    tokio::pin!(grace);

    loop {
        tokio::select! {
            _ = &mut grace => {
                log::warn!("Watcher: shutdown grace period elapsed, exiting now.");
                break;
            }
            next = set.join_next() => {
                match next {
                    Some(Ok(_)) => { /* task ended */ }
                    Some(Err(e)) => log::error!("Watcher: task join error: {}", e),
                    None => break, // no more tasks
                }
            }
        }
    }

    log::info!("Watcher stopped.");
}
