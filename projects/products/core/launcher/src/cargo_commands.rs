use std::{path::Path, process::Command};

use anyhow::{Context, Result, bail};

use crate::Build;

pub fn cargo_build(root: &Path, build: &Build, dry_run: bool) -> Result<(), anyhow::Error> {
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
