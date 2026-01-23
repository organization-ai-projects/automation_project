// projects/products/core/launcher/src/entry.rs
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{
    ChildHandle, Cli, Config, Workspace, cargo_build, install_shutdown_handler, normalize_path,
    parse_csv, start_and_supervise, {topo_sort, validate_services},
};
use clap::Parser;
use std::path::Path;
use thiserror::Error;

use crate::cargo_commands::CargoCommandError;
use crate::service::ServiceError;
use crate::shutdown::ShutdownError;
use crate::supervisor::SupervisorError;

#[derive(Debug, Error)]
pub enum EntryError {
    #[error("Failed to resolve parent directory of config path")]
    ResolveParentDir,

    #[error("failed to read config")]
    ReadConfig,

    #[error("invalid TOML in configuration file")]
    InvalidToml,

    #[error("Service error: {0}")]
    Service(#[from] ServiceError),

    #[error("Cargo command error: {0}")]
    CargoCommand(#[from] CargoCommandError),

    #[error("Shutdown error: {0}")]
    Shutdown(#[from] ShutdownError),

    #[error("Supervisor error: {0}")]
    Supervisor(#[from] SupervisorError),

    #[error("Service not found: {0}")]
    ServiceNotFound(String),
}

#[derive(Clone)]
pub struct Paths {
    pub workspace: Workspace,
    pub target_dir: PathBuf,
    pub profile_dir: PathBuf,
}

pub fn main() -> Result<(), EntryError> {
    let cli = Cli::parse();

    let config_path = PathBuf::from(&cli.config);
    let config_dir = config_path.parent().ok_or(EntryError::ResolveParentDir)?;

    let cfg_text = fs::read_to_string(&config_path).map_err(|_| EntryError::ReadConfig)?;
    let mut cfg: Config = toml::from_str(&cfg_text).map_err(|_| EntryError::InvalidToml)?;

    // Apply CLI overrides
    if cli.build {
        cfg.build.enabled = true;
    }
    if cli.no_build {
        cfg.build.enabled = false;
    }

    // Resolve workspace root using `Workspace` struct
    let workspace = Workspace {
        root: normalize_path(&config_dir.join(&cfg.workspace.root))
            .map_err(|_| EntryError::ResolveParentDir)?,
    };

    // Filter only/skip
    let only = parse_csv(&cli.only);
    let skip = parse_csv(&cli.skip);

    let mut services = cfg.service.clone();
    if let Some(only_set) = &only {
        services.retain(|s| only_set.contains(&s.name));
    }
    if let Some(skip_set) = &skip {
        services.retain(|s| !skip_set.contains(&s.name));
    }
    if services.is_empty() {
        return Err(EntryError::InvalidToml);
    }

    // Validate uniqueness + deps
    validate_services(&services)?;

    // Compute start order (topological sort)
    let start_order = topo_sort(&services)?;

    println!("🚀 core-launcher");
    println!("Workspace: {}", workspace.root.display());
    println!("Services: {}", start_order.join(", "));
    if cli.dry_run {
        println!("(dry-run) no processes will be spawned");
    }

    // Build once
    if cfg.build.enabled {
        cargo_build(&workspace.root, &cfg.build, cli.dry_run)?;
    }

    // Determine target dir + binary paths
    let (target_dir, profile_dir) = target_paths(&workspace.root, &cfg.build.profile);

    // State
    let running: Arc<Mutex<HashMap<String, ChildHandle>>> = Arc::new(Mutex::new(HashMap::new()));
    let shutting_down = Arc::new(Mutex::new(false));

    install_shutdown_handler(
        running.clone(),
        shutting_down.clone(),
        cfg.launcher.shutdown_grace_ms,
    )?;

    // Start services in order, waiting for readiness if configured
    for name in start_order {
        let svc = services
            .iter()
            .find(|s| s.name == name)
            .ok_or(EntryError::ServiceNotFound(name.clone()))?
            .clone();
        let paths = Paths {
            workspace: workspace.clone(),
            target_dir: target_dir.clone(),
            profile_dir: profile_dir.clone(),
        };
        start_and_supervise(
            svc,
            paths,
            running.clone(),
            shutting_down.clone(),
            Duration::from_millis(cfg.launcher.startup_timeout_ms),
            cli.dry_run,
        )?;
    }

    println!("✅ all services started");

    // Keep main alive; supervisor threads do the rest
    loop {
        thread::park();
    }
}

pub fn target_paths(workspace_root: &Path, profile: &str) -> (PathBuf, PathBuf) {
    let target_dir = workspace_root.join("target");
    let profile_dir = target_dir.join(profile);
    (target_dir, profile_dir)
}
