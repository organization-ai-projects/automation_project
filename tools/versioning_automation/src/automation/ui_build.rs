//! tools/versioning_automation/src/automation/ui_build/mod.rs
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::automation::{
    commands::{BuildAccountsUiOptions, BuildAndCheckUiBundlesOptions, BuildUiBundlesOptions},
    execute,
};

pub(crate) fn run_build_accounts_ui(_opts: BuildAccountsUiOptions) -> Result<(), String> {
    let root = ui_build_root()?;
    let ui_dir = root.join("projects/products/stable/accounts/ui");
    if !ui_dir.is_dir() {
        return Err(format!("UI directory not found: '{}'", ui_dir.display()));
    }
    build_ui_bundle_for_dir(&ui_dir)?;
    println!(
        "Accounts UI bundle generated in {}",
        ui_dir.join("ui_dist").display()
    );
    Ok(())
}

pub(crate) fn run_build_ui_bundles(_opts: BuildUiBundlesOptions) -> Result<(), String> {
    let root = ui_build_root()?;
    let ui_dirs = find_ui_dirs(&root.join("projects/products"))?;
    if ui_dirs.is_empty() {
        return Err("No UI crates found under projects/products".to_string());
    }

    for ui_dir in ui_dirs {
        let cargo_toml = ui_dir.join("Cargo.toml");
        if !cargo_contains_dioxus(&cargo_toml)? {
            println!("Skipping {} (no dioxus dependency)", ui_dir.display());
            continue;
        }
        println!("Building UI bundle in {}", ui_dir.display());
        build_ui_bundle_for_dir(&ui_dir)?;
    }
    println!("UI bundle build complete");
    Ok(())
}

pub(crate) fn run_build_and_check_ui_bundles(
    _opts: BuildAndCheckUiBundlesOptions,
) -> Result<(), String> {
    let root = ui_build_root()?;
    let ui_dirs = find_ui_dirs(&root.join("projects/products"))?;
    if ui_dirs.is_empty() {
        return Err("No UI crates found under projects/products".to_string());
    }

    let mut missing = Vec::new();
    for ui_dir in ui_dirs {
        let cargo_toml = ui_dir.join("Cargo.toml");
        if !cargo_contains_dioxus(&cargo_toml)? {
            println!("Skipping {} (no dioxus dependency)", ui_dir.display());
            continue;
        }
        println!("Building UI bundle in {}", ui_dir.display());
        build_ui_bundle_for_dir(&ui_dir)?;
        if !ui_bundle_artifacts_ok(&ui_dir.join("ui_dist"))? {
            missing.push(ui_dir.to_string_lossy().to_string());
        }
    }

    if missing.is_empty() {
        println!("UI bundle build + check complete");
        return Ok(());
    }

    eprintln!("Missing UI bundle artifacts in:");
    for path in missing {
        eprintln!(" - {path}");
    }
    Err("One or more UI bundles are incomplete.".to_string())
}

fn ui_build_root() -> Result<PathBuf, String> {
    execute::ensure_git_repo()?;
    execute::require_command(
        "dx",
        "dx (dioxus-cli) not found. Install with: cargo install dioxus-cli",
    )?;
    execute::repo_root()
}

fn find_ui_dirs(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut dirs = Vec::new();
    find_ui_dirs_recursive(root, &mut dirs)?;
    dirs.sort();
    Ok(dirs)
}

fn find_ui_dirs_recursive(root: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        Err(_) => return Ok(()),
    };
    for entry in entries {
        let entry =
            entry.map_err(|e| format!("Failed to read entry under '{}': {e}", root.display()))?;
        let path = entry.path();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if !file_type.is_dir() {
            continue;
        }

        if path.file_name().and_then(|v| v.to_str()) == Some("ui")
            && path.join("Cargo.toml").is_file()
        {
            out.push(path);
            continue;
        }
        find_ui_dirs_recursive(&path, out)?;
    }
    Ok(())
}

fn cargo_contains_dioxus(cargo_toml: &Path) -> Result<bool, String> {
    let content = fs::read_to_string(cargo_toml)
        .map_err(|e| format!("Failed to read '{}': {e}", cargo_toml.display()))?;
    Ok(content.contains("dioxus"))
}

fn build_ui_bundle_for_dir(ui_dir: &Path) -> Result<(), String> {
    let mut cmd = Command::new("dx");
    cmd.arg("bundle")
        .arg("--release")
        .arg("--debug-symbols")
        .arg("false")
        .arg("--out-dir")
        .arg("ui_dist")
        .current_dir(ui_dir);
    let rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();
    let merged_rustflags = if rustflags.trim().is_empty() {
        "-C debuginfo=0".to_string()
    } else {
        format!("{rustflags} -C debuginfo=0")
    };
    cmd.env("CARGO_PROFILE_RELEASE_DEBUG", "0");
    cmd.env("RUSTFLAGS", merged_rustflags);

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to run dx bundle in '{}': {e}", ui_dir.display()))?;
    if !status.success() {
        return Err(format!(
            "dx bundle failed in '{}' with exit {:?}",
            ui_dir.display(),
            status.code()
        ));
    }

    let manifest = ui_dir.join("ui_manifest.ron");
    let out_manifest = ui_dir.join("ui_dist").join("ui_manifest.ron");
    if manifest.is_file() {
        fs::copy(&manifest, &out_manifest).map_err(|e| {
            format!(
                "Failed to copy '{}' to '{}': {e}",
                manifest.display(),
                out_manifest.display()
            )
        })?;
    }
    Ok(())
}

fn ui_bundle_artifacts_ok(ui_dist: &Path) -> Result<bool, String> {
    let index = ui_dist.join("public/index.html");
    if !index.is_file() {
        return Ok(false);
    }
    let assets = ui_dist.join("public/assets");
    let mut has_js = false;
    let mut has_wasm = false;
    let entries = match fs::read_dir(&assets) {
        Ok(entries) => entries,
        Err(_) => return Ok(false),
    };
    for entry in entries {
        let entry = entry
            .map_err(|e| format!("Failed to read assets under '{}': {e}", assets.display()))?;
        let path = entry.path();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if !file_type.is_file() {
            continue;
        }
        let file_name = path
            .file_name()
            .and_then(|v| v.to_str())
            .unwrap_or_default();
        if file_name.ends_with(".js") {
            has_js = true;
        }
        if file_name.ends_with(".wasm") {
            has_wasm = true;
        }
    }
    Ok(has_js && has_wasm)
}
