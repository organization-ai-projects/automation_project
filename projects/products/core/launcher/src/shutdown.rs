// projects/products/core/launcher/src/shutdown.rs
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::ChildHandle;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShutdownError {
    #[error("Failed to acquire lock on shutting_down")]
    LockShuttingDown,

    #[error("Failed to acquire lock on running")]
    LockRunning,

    #[error("Failed to acquire lock on child")]
    LockChild,

    #[error("Failed to set Ctrl+C handler")]
    CtrlCHandlerError,
}

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
) -> Result<(), ShutdownError> {
    ctrlc::set_handler(move || {
        let result: Result<(), ShutdownError> = (|| {
            let mut shutting_down = shutting_down
                .lock()
                .map_err(|_| ShutdownError::LockShuttingDown)?;
            *shutting_down = true;

            let mut running_guard = running.lock().map_err(|_| ShutdownError::LockRunning)?;
            for (_name, handle) in running_guard.iter_mut() {
                handle.kill().map_err(|_| ShutdownError::LockChild)?;
            }

            // give some grace
            thread::sleep(Duration::from_millis(grace_ms));

            // best effort: wait then exit
            for (name, handle) in running_guard.iter_mut() {
                let _ = handle
                    .child
                    .lock()
                    .map_err(|_| ShutdownError::LockChild)?
                    .wait();
                eprintln!("Stopped {name}");
            }
            Ok(())
        })();

        if let Err(err) = result {
            eprintln!("Error during shutdown: {:?}", err);
        }
    })
    .map_err(|_| ShutdownError::CtrlCHandlerError)?;
    Ok(())
}

// Ajout des conversions From pour ShutdownError
impl From<ctrlc::Error> for ShutdownError {
    fn from(_: ctrlc::Error) -> Self {
        ShutdownError::LockShuttingDown // Exemple de conversion
    }
}
