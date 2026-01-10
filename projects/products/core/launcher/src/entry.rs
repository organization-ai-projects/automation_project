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
    ChildHandle, Cli, Config, cargo_build, normalize_path, parse_csv,
    service::{topo_sort, validate_services},
    shutdown::install_shutdown_handler,
    supervisor::start_and_supervise,
    workspace::Workspace,
};
use anyhow::{Context, Result, bail};
use clap::Parser;
use std::path::Path;

#[derive(Clone)]
pub struct Paths {
    pub workspace: Workspace,
    pub target_dir: PathBuf,
    pub profile_dir: PathBuf,
}

pub fn main() -> Result<()> {
    let cli = Cli::parse();

    let config_path = PathBuf::from(&cli.config);
    let config_dir = config_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));

    let cfg_text = fs::read_to_string(&config_path).with_context(|| "failed to read config")?;
    let mut cfg: Config =
        toml::from_str(&cfg_text).with_context(|| "invalid TOML in configuration file")?;

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
            .with_context(|| "failed to resolve workspace.root")?,
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
        bail!("no services selected (check config/--only/--skip)");
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
        let svc = services.iter().find(|s| s.name == name).unwrap().clone();
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
