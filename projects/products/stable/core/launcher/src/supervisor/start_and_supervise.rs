// projects/products/stable/core/launcher/src/supervisor/start_and_supervise.rs
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use anyhow::Result;

use crate::{
    child_handle::ChildHandle,
    entry::Paths,
    logging::log_message,
    process::{resolve_bin_path, spawn_service},
    service::Service,
};

use super::{
    locks::lock_recover, logger::pipe_child_outputs, readiness::wait_ready,
    supervise_loop::supervise_loop,
};

/// Starts and supervises a service, handling its lifecycle and restart policy.
///
/// # Arguments
/// * `svc` - The service to start and supervise.
/// * `paths` - Paths related to the workspace and binaries.
/// * `running` - A shared map of running services and their handles.
/// * `shutting_down` - A shared flag indicating if shutdown is in progress.
/// * `startup_timeout` - The timeout duration for the service to become ready.
/// * `dry_run` - If true, the service will not actually be started.
pub(crate) fn start_and_supervise(
    svc: Service,
    paths: Paths,
    running: Arc<Mutex<HashMap<String, ChildHandle>>>,
    shutting_down: Arc<Mutex<bool>>,
    startup_timeout: Duration,
    dry_run: bool,
) -> Result<()> {
    let bin_path = resolve_bin_path(&paths.workspace.root, &paths.profile_dir, &svc.bin);

    // initial spawn
    let child = spawn_service(&svc, &paths.workspace.root, &bin_path, dry_run)?;
    if dry_run {
        return Ok(());
    }

    let handle = ChildHandle {
        child: Arc::new(Mutex::new(child)),
    };

    // Insert into running map and drop lock immediately
    {
        let mut running_map = lock_recover(&running, "running map");
        running_map.insert(svc.name.clone(), handle.clone());
    }

    // log piping (stdout/stderr)
    pipe_child_outputs(svc.name.clone(), handle.clone());

    // readiness
    if let Err(e) = wait_ready(&svc, startup_timeout) {
        log_message(&format!("failed to become ready: {}", e), true, &svc.name);
        let mut running_map = lock_recover(&running, "running map");
        running_map.remove(&svc.name);
        let mut child = lock_recover(&handle.child, "child");
        let _ = child.kill();
        return Err(e);
    }

    // supervisor thread (restart policy)
    let running = Arc::clone(&running);
    let shutting_down = Arc::clone(&shutting_down);
    thread::spawn(move || {
        supervise_loop(
            svc,
            paths.workspace.root,
            bin_path,
            running,
            shutting_down,
            startup_timeout,
        );
    });

    Ok(())
}
