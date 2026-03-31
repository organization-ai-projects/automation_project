use std::{
    env, thread,
    time::{Duration, Instant},
};

use crate::{
    automation::{
        execute::{branch_exists_local, branch_exists_remote},
        resolve_branch_sha, run_git,
    },
    gh_cli,
};

#[derive(Debug)]
pub(crate) struct SyncMainDevCiOptions {
    pub(crate) remote: String,
    pub(crate) main: String,
    pub(crate) dev: String,
    pub(crate) sync_branch: String,
}

impl SyncMainDevCiOptions {
    pub(crate) fn run_sync_main_dev_ci(self) -> Result<(), String> {
        if env::var("CI").unwrap_or_default() != "true" {
            return Err("This command can only be executed in CI (CI=true).".to_string());
        }

        run_git(&["fetch", &self.remote])?;

        let main_ref = format!("{}/{}", self.remote, self.main);
        let dev_ref = format!("{}/{}", self.remote, self.dev);

        let main_sha = resolve_branch_sha(&main_ref)?;
        let dev_sha = resolve_branch_sha(&dev_ref)?;
        if main_sha == dev_sha {
            println!("No sync needed - dev is already up to date with main");
            return Ok(());
        }

        if run_git(&["merge-base", "--is-ancestor", &main_ref, &dev_ref]).is_ok() {
            println!("No sync needed - dev already contains all commits from main");
            return Ok(());
        }

        if branch_exists_local(&self.sync_branch) {
            let _ = run_git(&["branch", "-D", &self.sync_branch]);
        }
        if branch_exists_remote(&self.remote, &self.sync_branch) {
            let _ = run_git(&["push", &self.remote, "--delete", &self.sync_branch]);
        }

        run_git(&["switch", "-C", &self.sync_branch, &main_ref])?;
        run_git(&["push", "-f", &self.remote, &self.sync_branch])?;

        let pr_output = gh_cli::output_trim(&[
            "pr",
            "create",
            "--base",
            &self.dev,
            "--head",
            &self.sync_branch,
            "--title",
            "chore: sync main into dev",
            "--body",
            "Automated sync after merge into main.",
        ])
        .map_err(|e| {
            format!(
                "Failed to run gh pr create --base {} --head {}: {e}",
                self.dev, self.sync_branch
            )
        })?;

        let pr_url = pr_output.trim().to_string();
        if pr_url.is_empty() {
            return Err("Failed to create sync PR (empty response).".to_string());
        }

        let stable_timeout = env::var("STABLE_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(120);
        let deadline = Instant::now() + Duration::from_secs(stable_timeout);

        let mergeable = loop {
            if Instant::now() >= deadline {
                return Err("PR did not stabilize in time.".to_string());
            }
            let value = gh_cli::output_trim(&[
            "pr",
            "view",
            &pr_url,
            "--json",
            "mergeable",
            "--jq",
            ".mergeable // \"UNKNOWN\"",
        ])
        .map_err(|e| {
            format!(
                "Failed to run gh pr view {} --json mergeable --jq .mergeable // \"UNKNOWN\": {e}",
                pr_url
            )
        })?;
            if value != "UNKNOWN" {
                break value;
            }
            thread::sleep(Duration::from_secs(5));
        };

        if mergeable == "CONFLICTING" {
            return Err("PR has merge conflicts. Cannot enable auto-merge.".to_string());
        }
        if mergeable != "MERGEABLE" {
            return Err(format!("PR is not mergeable (status: {mergeable})."));
        }

        gh_cli::status(&[
            "pr",
            "merge",
            &pr_url,
            "--auto",
            "--merge",
            "--delete-branch",
        ])
        .map_err(|e| {
            format!(
                "Failed to run gh pr merge {} --auto --merge --delete-branch: {e}",
                pr_url
            )
        })?;
        Ok(())
    }
}
