// projects/products/core/launcher/src/supervisor.rs
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{
    ChildHandle, Paths, RestartPolicy, Service, log_message, {resolve_bin_path, spawn_service},
};
use anyhow::{Result, bail};
use std::time::Instant;

/// Starts and supervises a service, handling its lifecycle and restart policy.
///
/// # Arguments
/// * `svc` - The service to start and supervise.
/// * `paths` - Paths related to the workspace and binaries.
/// * `running` - A shared map of running services and their handles.
/// * `shutting_down` - A shared flag indicating if shutdown is in progress.
/// * `startup_timeout` - The timeout duration for the service to become ready.
/// * `dry_run` - If true, the service will not actually be started.
pub fn start_and_supervise(
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

use std::io::{BufRead, BufReader};

// Define an enum to replace the boolean `is_err`
#[derive(Debug, Clone, Copy)]
pub enum LogStream {
    Stdout,
    Stderr,
}

// Replaced direct calls to println! and eprintln! with the centralized log_message function
fn spawn_logger<R: std::io::Read + Send + 'static>(
    stream: Option<R>,
    name: String,
    log_stream: LogStream,
) {
    if let Some(stream) = stream {
        thread::spawn(move || {
            let reader = BufReader::new(stream);
            for line in reader.lines() {
                match line {
                    Ok(line) => match log_stream {
                        LogStream::Stdout => log_message(&line, false, &name),
                        LogStream::Stderr => log_message(&line, true, &name),
                    },
                    Err(e) => {
                        log_message(&format!("log stream error: {}", e), true, &name);
                        break;
                    }
                }
            }
        });
    }
}

// Added a new function pipe_child_outputs to handle both stdout and stderr at once
pub fn pipe_child_outputs(name: String, handle: ChildHandle) {
    let (stdout, stderr) = {
        let mut child = lock_recover(&handle.child, "child");
        (child.stdout.take(), child.stderr.take())
    };

    spawn_logger(stdout, name.clone(), LogStream::Stdout);
    spawn_logger(stderr, name, LogStream::Stderr);
}

pub fn wait_ready(svc: &Service, timeout: Duration) -> Result<()> {
    let Some(url) = &svc.ready_http else {
        return Ok(());
    };

    let start = Instant::now();
    while start.elapsed() < timeout {
        if is_http_healthy(url) {
            log_message(&format!("✅ ready: {}", svc.name), false, &svc.name);
            return Ok(());
        }
        thread::sleep(Duration::from_millis(200));
    }

    bail!("service `{}` not ready within {:?}", svc.name, timeout);
}

fn is_http_healthy(url: &str) -> bool {
    let resp = ureq::get(url).timeout(Duration::from_millis(800)).call();
    resp.ok()
        .map(|r| r.status() >= 200 && r.status() < 300)
        .unwrap_or(false)
}

// Simplified `lock_recover` by introducing a type alias for MutexGuard
use std::sync::MutexGuard;

type LockGuard<'a, T> = MutexGuard<'a, T>;

fn lock_recover<'a, T>(m: &'a Mutex<T>, what: &str) -> LockGuard<'a, T> {
    match m.lock() {
        Ok(g) => g,
        Err(p) => {
            log_message(
                &format!("{what} lock poisoned; recovering"),
                true,
                "launcher",
            );
            p.into_inner()
        }
    }
}

// Added a new lock_recover_arc function to handle Arc<Mutex<T>>

fn lock_recover_arc<'a, T>(m: &'a Arc<Mutex<T>>, what: &str) -> MutexGuard<'a, T> {
    match m.lock() {
        Ok(g) => g,
        Err(p) => {
            log_message(
                &format!("{what} lock poisoned; recovering"),
                true,
                "launcher",
            );
            p.into_inner()
        }
    }
}

// Helper function to handle child process restart logic
fn handle_restart(
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

// Final improvements: block-based `is_shutting_down` and strict ready fail policy
pub fn supervise_loop(
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
