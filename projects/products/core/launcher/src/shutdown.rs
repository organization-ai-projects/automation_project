// projects/products/core/launcher/src/shutdown.rs
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use anyhow::Result;

use crate::child_handle::ChildHandle;

/// Installs a shutdown handler to gracefully terminate running services.
///
/// # Arguments
/// * `running` - A shared map of running services and their handles.
/// * `shutting_down` - A shared flag indicating if shutdown is in progress.
/// * `grace_ms` - The grace period in milliseconds before forcing termination.
pub(crate) fn install_shutdown_handler(
    running: Arc<Mutex<HashMap<String, ChildHandle>>>,
    shutting_down: Arc<Mutex<bool>>,
    grace_ms: u64,
) -> Result<()> {
    ctrlc::set_handler(move || {
        eprintln!("\n🛑 shutdown requested");
        {
            if let Ok(mut sd) = shutting_down.lock() {
                *sd = true;
            } else {
                eprintln!("Failed to acquire lock on shutting_down");
            }
        }

        let mut map = match running.lock() {
            Ok(map) => map,
            Err(_) => {
                eprintln!("Failed to acquire lock on running");
                return;
            }
        };

        for (name, handle) in map.iter_mut() {
            eprintln!("Stopping {name}");
            if let Ok(mut child) = handle.child.lock() {
                let _ = child.kill();
            } else {
                eprintln!("Failed to acquire lock on child");
            }
        }

        // give some grace
        thread::sleep(Duration::from_millis(grace_ms));

        // best effort: wait then exit
        for (name, handle) in map.iter_mut() {
            if let Ok(mut child) = handle.child.lock() {
                let _ = child.wait();
            } else {
                eprintln!("Failed to acquire lock on child");
            }
            eprintln!("Stopped {name}");
        }
        std::process::exit(0);
    })?;
    Ok(())
}
