// projects/products/stable/core/launcher/src/supervisor/supervise_loop.rs
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{
    child_handle::ChildHandle, logging::log_message, restart_policy::RestartPolicy,
    service::Service,
};

use super::{
    locks::{lock_recover, lock_recover_arc},
    restart::handle_restart,
};

// Final improvements: block-based `is_shutting_down` and strict ready fail policy
pub(crate) fn supervise_loop(
    svc: Service,
    workspace_root: PathBuf,
    bin_path: PathBuf,
    running: Arc<Mutex<HashMap<String, ChildHandle>>>,
    shutting_down: Arc<Mutex<bool>>,
    startup_timeout: Duration,
) {
    let mut restarts: u32 = 0;

    loop {
        if *lock_recover(&shutting_down, "shutting_down") {
            // Optionally kill the process if shutting down
            if let Some(h) = {
                let map = lock_recover(&running, "running map");
                map.get(&svc.name).cloned()
            } {
                let mut c = lock_recover_arc(&h.child, "child");
                let _ = c.kill();
            }
            return;
        }

        let child = {
            let map = lock_recover(&running, "running map");
            map.get(&svc.name).cloned()
        };

        if let Some(child) = child {
            loop {
                if *lock_recover(&shutting_down, "shutting_down") {
                    // Kill the process if shutting down
                    let mut c = lock_recover_arc(&child.child, "child");
                    let _ = c.kill();
                    return;
                }

                let mut child_guard = lock_recover_arc(&child.child, "child");
                match child_guard.try_wait() {
                    Ok(Some(status)) => {
                        {
                            let mut map = lock_recover(&running, "running map");
                            map.remove(&svc.name);
                        }
                        let code: i32 = status.code().unwrap_or(-1);
                        let success = status.success();
                        log_message(&format!("exited (code={})", code), true, &svc.name);

                        let should_restart = match svc.restart {
                            RestartPolicy::Never => false,
                            RestartPolicy::Always => true,
                            RestartPolicy::OnFailure => !success,
                        };

                        if !should_restart {
                            return;
                        }
                        break;
                    }
                    Ok(None) => {
                        drop(child_guard);
                        thread::sleep(Duration::from_millis(100));
                    }
                    Err(e) => {
                        log_message(
                            &format!("[{}] try_wait() failed: {}", svc.name, e),
                            true,
                            &svc.name,
                        );
                        {
                            let mut map = lock_recover(&running, "running map");
                            map.remove(&svc.name);
                        }
                        return;
                    }
                }
            }
        }

        // Added a condition to block restart when RestartPolicy is Never
        if matches!(svc.restart, RestartPolicy::Never) {
            log_message(
                "not running and restart policy is Never; stopping supervisor",
                true,
                &svc.name,
            );
            return;
        }

        if !handle_restart(
            &svc,
            &running,
            &shutting_down,
            workspace_root.as_path(),
            bin_path.as_path(),
            startup_timeout,
            &mut restarts,
        ) {
            return;
        }
    }
}
