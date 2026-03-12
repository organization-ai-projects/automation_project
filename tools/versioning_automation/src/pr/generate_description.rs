use std::path::{Path, PathBuf};
use std::process::Command;

use crate::pr::commands::pr_generate_description_options::PrGenerateDescriptionOptions;

pub(crate) fn run_generate_description(opts: PrGenerateDescriptionOptions) -> i32 {
    let script_path = match resolve_script_path() {
        Some(path) => path,
        None => {
            eprintln!(
                "Unable to locate scripts/versioning/file_versioning/github/generate_pr_description.sh"
            );
            return 4;
        }
    };

    let mut cmd = Command::new("bash");
    cmd.arg(script_path);
    for arg in opts.passthrough {
        cmd.arg(arg);
    }

    match cmd.status() {
        Ok(status) => status.code().unwrap_or(1),
        Err(err) => {
            eprintln!("Failed to execute generator script: {err}");
            1
        }
    }
}

fn resolve_script_path() -> Option<PathBuf> {
    let rel = Path::new("scripts/versioning/file_versioning/github/generate_pr_description.sh");

    if let Ok(cwd) = std::env::current_dir() {
        let candidate = cwd.join(rel);
        if candidate.is_file() {
            return Some(candidate);
        }
    }

    let manifest_candidate = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .join(rel);
    if manifest_candidate.is_file() {
        return Some(manifest_candidate);
    }

    None
}
