// projects/products/core/launcher/src/supervisor/restart.rs
use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{
    child_handle::ChildHandle, logging::log_message, process::spawn_service, service::Service,
};

use super::{locks::lock_recover, logger::pipe_child_outputs, readiness::wait_ready};

pub(crate) fn handle_restart(
    svc: &Service,
    running: &Arc<Mutex<HashMap<String, ChildHandle>>>,
    shutting_down: &Arc<Mutex<bool>>,
    workspace_root: &Path,
    bin_path: &Path,
    startup_timeout: Duration,
    restarts: &mut u32,
) -> bool {
    if svc.restart_max != 0 && *restarts >= svc.restart_max {
        log_message(
            &format!("[{}] restart limit reached ({})", svc.name, svc.restart_max),
            true,
            &svc.name,
        );
        lock_recover(running, "running map").remove(&svc.name);
        return false;
    }

    thread::sleep(Duration::from_millis(svc.restart_backoff_ms));

    let is_shutting_down = {
        let g = lock_recover(shutting_down, "shutting_down");
        *g
    };
    if is_shutting_down {
        return false;
    }

    let child = match spawn_service(svc, workspace_root, bin_path, false) {
        Ok(c) => {
            *restarts += 1; // Increment only after a successful spawn
            c
        }
        Err(e) => {
            log_message(
                &format!("[{}] restart spawn failed: {:?}", svc.name, e),
                true,
                &svc.name,
            );
            return true; // Retry will occur in the next loop iteration
        }
    };

    let handle = ChildHandle {
        child: Arc::new(Mutex::new(child)),
    };
    lock_recover(running, "running map").insert(svc.name.clone(), handle.clone());

    pipe_child_outputs(svc.name.clone(), handle.clone());

    if let Err(e) = wait_ready(svc, startup_timeout) {
        log_message(
            &format!("[{}] not ready after restart: {}", svc.name, e),
            true,
            &svc.name,
        );
        {
            let mut map = lock_recover(running, "running map");
            map.remove(&svc.name); // Remove immediately after failure
        }
        let mut child = lock_recover(&handle.child, "child");
        let _ = child.kill();
        return true;
    }

    true
}
