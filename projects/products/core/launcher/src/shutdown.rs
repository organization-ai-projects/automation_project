// projects/products/core/launcher/src/shutdown.rs
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::ChildHandle;
use anyhow::Result;

/// Installs a shutdown handler to gracefully terminate running services.
///
/// # Arguments
/// * `running` - A shared map of running services and their handles.
/// * `shutting_down` - A shared flag indicating if shutdown is in progress.
/// * `grace_ms` - The grace period in milliseconds before forcing termination.
pub fn install_shutdown_handler(
    running: Arc<Mutex<HashMap<String, ChildHandle>>>,
    shutting_down: Arc<Mutex<bool>>,
    grace_ms: u64,
) -> Result<()> {
    ctrlc::set_handler(move || {
        eprintln!("\n🛑 shutdown requested");
        {
            let mut sd = shutting_down.lock().unwrap();
            *sd = true;
        }

        let mut map = running.lock().unwrap();
        for (name, handle) in map.iter_mut() {
            eprintln!("Stopping {name}");
            let _ = handle.child.lock().unwrap().kill();
        }

        // give some grace
        thread::sleep(Duration::from_millis(grace_ms));

        // best effort: wait then exit
        for (name, handle) in map.iter_mut() {
            let _ = handle.child.lock().unwrap().wait();
            eprintln!("Stopped {name}");
        }
        std::process::exit(0);
    })?;
    Ok(())
}
