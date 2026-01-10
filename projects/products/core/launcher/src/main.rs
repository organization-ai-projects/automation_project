use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use anyhow::{Context, Result, anyhow, bail};
use clap::Parser;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(name = "core-launcher")]
struct Cli {
    /// Path to launcher.toml
    #[arg(long, default_value = "launcher.toml")]
    config: String,

    /// Only start these services (comma-separated)
    #[arg(long)]
    only: Option<String>,

    /// Skip these services (comma-separated)
    #[arg(long)]
    skip: Option<String>,

    /// Do not spawn processes; print what would happen
    #[arg(long)]
    dry_run: bool,

    /// Build before launch (overrides config)
    #[arg(long)]
    build: bool,

    /// Don't build before launch (overrides config)
    #[arg(long)]
    no_build: bool,
}

#[derive(Debug, Deserialize)]
struct Config {
    workspace: Workspace,
    #[serde(default)]
    build: Build,
    #[serde(default)]
    launcher: Launcher,
    #[serde(default)]
    service: Vec<Service>,
}

#[derive(Debug, Deserialize)]
struct Workspace {
    root: String,
}

#[derive(Debug, Default, Deserialize)]
struct Build {
    #[serde(default = "default_build_enabled")]
    enabled: bool,
    #[serde(default = "default_profile")]
    profile: String, // "debug" | "release"
    #[serde(default)]
    extra_args: Vec<String>,
}

fn default_build_enabled() -> bool {
    true
}
fn default_profile() -> String {
    "debug".to_string()
}

#[derive(Debug, Default, Deserialize)]
struct Launcher {
    #[serde(default = "default_startup_timeout")]
    startup_timeout_ms: u64,
    #[serde(default = "default_shutdown_grace")]
    shutdown_grace_ms: u64,
}

fn default_startup_timeout() -> u64 {
    15_000
}
fn default_shutdown_grace() -> u64 {
    2_500
}

#[derive(Debug, Deserialize, Clone)]
struct Service {
    name: String,
    bin: String,

    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    env: Vec<String>,
    #[serde(default)]
    cwd: Option<String>,

    #[serde(default)]
    depends_on: Vec<String>,

    #[serde(default = "default_restart")]
    restart: RestartPolicy,
    #[serde(default)]
    restart_max: u32, // 0 = infinite
    #[serde(default = "default_backoff")]
    restart_backoff_ms: u64,

    #[serde(default)]
    ready_http: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
enum RestartPolicy {
    Never,
    OnFailure,
    Always,
}
fn default_restart() -> RestartPolicy {
    RestartPolicy::OnFailure
}
fn default_backoff() -> u64 {
    500
}

#[derive(Clone)]
struct ChildHandle {
    child: Arc<Mutex<Child>>,
}

fn main() -> Result<()> {
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

    // Resolve workspace root relative to config file
    let workspace_root = normalize_path(&config_dir.join(&cfg.workspace.root))
        .with_context(|| "failed to resolve workspace.root")?;

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
    println!("Workspace: {}", workspace_root.display());
    println!("Services: {}", start_order.join(", "));
    if cli.dry_run {
        println!("(dry-run) no processes will be spawned");
    }

    // Build once
    if cfg.build.enabled {
        cargo_build(&workspace_root, &cfg.build, cli.dry_run)?;
    }

    // Determine target dir + binary paths
    let (target_dir, profile_dir) = target_paths(&workspace_root, &cfg.build.profile);

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
            workspace_root: workspace_root.clone(),
            _target_dir: target_dir.clone(),
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

fn parse_csv(v: &Option<String>) -> Option<HashSet<String>> {
    v.as_ref().map(|s| {
        s.split(',')
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect()
    })
}

fn normalize_path(p: &Path) -> Result<PathBuf> {
    let abs = if p.is_absolute() {
        p.to_path_buf()
    } else {
        std::env::current_dir()?.join(p)
    };
    Ok(abs)
}

fn validate_services(services: &[Service]) -> Result<()> {
    let mut names = HashSet::new();
    for s in services {
        if s.name.trim().is_empty() {
            bail!("service with empty name");
        }
        if !names.insert(s.name.clone()) {
            bail!("duplicate service name: {}", s.name);
        }
    }
    // deps exist
    let set: HashSet<_> = services.iter().map(|s| s.name.as_str()).collect();
    for s in services {
        for d in &s.depends_on {
            if !set.contains(d.as_str()) {
                bail!("service `{}` depends on unknown service `{}`", s.name, d);
            }
        }
    }
    Ok(())
}

fn topo_sort(services: &[Service]) -> Result<Vec<String>> {
    let mut indeg: HashMap<String, usize> = HashMap::new();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for s in services {
        indeg.entry(s.name.clone()).or_insert(0);
        for d in &s.depends_on {
            graph.entry(d.clone()).or_default().push(s.name.clone());
            *indeg.entry(s.name.clone()).or_insert(0) += 1;
        }
    }

    let mut q: VecDeque<String> = indeg
        .iter()
        .filter(|(_, v)| **v == 0)
        .map(|(k, _)| k.clone())
        .collect();

    let mut out = Vec::new();
    while let Some(n) = q.pop_front() {
        out.push(n.clone());
        if let Some(nexts) = graph.get(&n) {
            for m in nexts {
                let e = indeg.get_mut(m).unwrap();
                *e -= 1;
                if *e == 0 {
                    q.push_back(m.clone());
                }
            }
        }
    }

    if out.len() != services.len() {
        bail!("dependency cycle detected in services (topo sort failed)");
    }
    Ok(out)
}

fn cargo_build(root: &Path, build: &Build, dry_run: bool) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("build");

    if build.profile == "release" {
        cmd.arg("--release");
    }

    for a in &build.extra_args {
        cmd.arg(a);
    }

    cmd.current_dir(root);

    println!("🔨 build: {:?}", cmd);
    if dry_run {
        return Ok(());
    }

    let status = cmd.status().context("failed to run cargo build")?;
    if !status.success() {
        bail!("cargo build failed with status={status}");
    }
    Ok(())
}

fn target_paths(workspace_root: &Path, profile: &str) -> (PathBuf, PathBuf) {
    // Standard Cargo layout: <workspace>/target/<profile>/
    let target = workspace_root.join("target");
    let profile_dir = if profile == "release" {
        "release"
    } else {
        "debug"
    };
    (target, PathBuf::from(profile_dir))
}

fn install_shutdown_handler(
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

struct Paths {
    workspace_root: PathBuf,
    _target_dir: PathBuf,
    profile_dir: PathBuf,
}

fn start_and_supervise(
    svc: Service,
    paths: Paths,
    running: Arc<Mutex<HashMap<String, ChildHandle>>>,
    shutting_down: Arc<Mutex<bool>>,
    startup_timeout: Duration,
    dry_run: bool,
) -> Result<()> {
    let bin_path = resolve_bin_path(&paths.workspace_root, &paths.profile_dir, &svc.bin);

    // initial spawn
    let child = spawn_service(&svc, &paths.workspace_root, &bin_path, dry_run)?;
    if dry_run {
        return Ok(());
    }

    let handle = ChildHandle {
        child: Arc::new(Mutex::new(child)),
    };
    running
        .lock()
        .unwrap()
        .insert(svc.name.clone(), handle.clone());

    // log piping (stdout/stderr)
    pipe_child_output(svc.name.clone(), handle.clone(), false);
    pipe_child_output(svc.name.clone(), handle.clone(), true);

    // readiness
    wait_ready(&svc, startup_timeout)?;

    // supervisor thread (restart policy)
    thread::spawn(move || {
        supervise_loop(svc, paths.workspace_root, bin_path, running, shutting_down);
    });

    Ok(())
}

fn resolve_bin_path(workspace_root: &Path, profile_dir: &Path, bin: &str) -> PathBuf {
    // Unix: target/debug/bin ; Windows: target\debug\bin.exe
    let mut p = workspace_root.join("target").join(profile_dir).join(bin);
    if cfg!(windows) {
        p.set_extension("exe");
    }
    p
}

fn spawn_service(
    svc: &Service,
    workspace_root: &Path,
    bin_path: &Path,
    dry_run: bool,
) -> Result<Child> {
    let cwd = svc
        .cwd
        .as_ref()
        .map(|c| workspace_root.join(c))
        .unwrap_or_else(|| workspace_root.to_path_buf());

    let mut cmd = Command::new(bin_path);
    cmd.args(&svc.args)
        .current_dir(&cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    for kv in &svc.env {
        if let Some((k, v)) = kv.split_once('=') {
            cmd.env(k, v);
        }
    }

    println!("▶ spawn {}: {:?}", svc.name, cmd);

    if dry_run {
        // dummy child is impossible; caller won't use it because dry_run returns earlier.
        return Err(anyhow!("dry-run: spawn skipped"));
    }

    cmd.spawn()
        .with_context(|| format!("failed to spawn `{}` from {}", svc.name, bin_path.display()))
}

fn pipe_child_output(name: String, handle: ChildHandle, is_err: bool) {
    // We can only take stdout/stderr once, right after spawn.
    // Here we lock and take them immediately.
    let (out, err) = {
        let mut child = handle.child.lock().unwrap();
        (child.stdout.take(), child.stderr.take())
    };

    let stream = match is_err {
        true => err.map(|e| Box::new(e) as Box<dyn std::io::Read + Send>),
        false => out.map(|o| Box::new(o) as Box<dyn std::io::Read + Send>),
    };
    let Some(stream) = stream else { return };

    thread::spawn(move || {
        let reader = BufReader::new(stream);
        for line in reader.lines().map_while(Result::ok) {
            if is_err {
                eprintln!("[{}] {}", name, line);
            } else {
                println!("[{}] {}", name, line);
            }
        }
    });
}

fn wait_ready(svc: &Service, timeout: Duration) -> Result<()> {
    let Some(url) = &svc.ready_http else {
        return Ok(());
    };

    let start = Instant::now();
    while start.elapsed() < timeout {
        if is_http_healthy(url) {
            println!("✅ ready: {}", svc.name);
            return Ok(());
        }
        thread::sleep(Duration::from_millis(200));
    }

    bail!("service `{}` not ready within {:?}", svc.name, timeout);
}

fn is_http_healthy(url: &str) -> bool {
    // keep deps minimal: use ureq
    let resp = ureq::get(url).timeout(Duration::from_millis(800)).call();
    resp.ok()
        .map(|r| r.status() >= 200 && r.status() < 300)
        .unwrap_or(false)
}

fn supervise_loop(
    svc: Service,
    workspace_root: PathBuf,
    bin_path: PathBuf,
    running: Arc<Mutex<HashMap<String, ChildHandle>>>,
    shutting_down: Arc<Mutex<bool>>,
) {
    let mut restarts: u32 = 0;

    loop {
        // stop if shutdown requested
        if *shutting_down.lock().unwrap() {
            return;
        }

        // Wait for exit
        let status = {
            let child = match running.lock().unwrap().get(&svc.name).cloned() {
                Some(h) => h,
                None => return,
            };
            let mut c = child.child.lock().unwrap();
            c.wait()
        };

        let Ok(status) = status else {
            eprintln!("[{}] wait() failed", svc.name);
            return;
        };

        let code = status.code().unwrap_or(-1);
        let success = status.success();
        eprintln!("[{}] exited (code={})", svc.name, code);

        // Decide restart
        let should_restart = match svc.restart {
            RestartPolicy::Never => false,
            RestartPolicy::Always => true,
            RestartPolicy::OnFailure => !success,
        };

        if !should_restart {
            return;
        }

        if svc.restart_max != 0 && restarts >= svc.restart_max {
            eprintln!("[{}] restart limit reached ({})", svc.name, svc.restart_max);
            return;
        }

        restarts += 1;

        // backoff
        thread::sleep(Duration::from_millis(svc.restart_backoff_ms));

        // spawn new child
        let child = match spawn_service(&svc, &workspace_root, &bin_path, false) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("[{}] restart spawn failed: {:?}", svc.name, e);
                continue;
            }
        };

        let handle = ChildHandle {
            child: Arc::new(Mutex::new(child)),
        };
        running
            .lock()
            .unwrap()
            .insert(svc.name.clone(), handle.clone());

        pipe_child_output(svc.name.clone(), handle.clone(), false);
        pipe_child_output(svc.name.clone(), handle.clone(), true);

        // best effort readiness; if it fails, we keep restarting based on policy
        if let Err(e) = wait_ready(&svc, Duration::from_millis(15_000)) {
            eprintln!("[{}] not ready after restart: {}", svc.name, e);
        }
    }
}
