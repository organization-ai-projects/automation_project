//! tools/versioning_automation/src/git/commands/branch_creation_check_options.rs
use crate::git::{run_git_output, run_git_passthrough};

#[derive(Debug)]
pub(crate) struct BranchCreationCheckOptions {
    pub(crate) command: Option<String>,
    pub(crate) args: Vec<String>,
}

impl BranchCreationCheckOptions {
    pub(crate) fn run_branch_creation_check(self) -> Result<(), String> {
        let Some(command) = self.command else {
            return run_git_passthrough(&[]);
        };

        if command != "branch" && command != "checkout" && command != "switch" {
            let mut passthrough = Vec::with_capacity(1 + self.args.len());
            passthrough.push(command);
            passthrough.extend(self.args);
            let refs = passthrough.iter().map(String::as_str).collect::<Vec<_>>();
            return run_git_passthrough(&refs);
        }

        let mut branch_to_check: Option<String> = None;
        let mut i = 0usize;
        while i < self.args.len() {
            let arg = self.args[i].as_str();
            match arg {
                "-b" | "-c" | "--branch" | "--create" | "-B" | "-C" | "--force-create" => {
                    if i + 1 < self.args.len() {
                        branch_to_check = Some(self.args[i + 1].clone());
                        i += 2;
                        continue;
                    }
                }
                _ => {
                    if command == "branch" && !arg.starts_with('-') && branch_to_check.is_none() {
                        branch_to_check = Some(arg.to_string());
                    }
                }
            }
            i += 1;
        }

        if let Some(branch) = branch_to_check {
            let marker = format!("[{branch}]");
            let worktrees = run_git_output(&["worktree", "list"])?;
            if worktrees.lines().any(|line| line.contains(&marker)) {
                eprintln!(
                    "❌ The branch '{}' is already in use by another worktree:",
                    branch
                );
                for line in worktrees.lines().filter(|line| line.contains(&marker)) {
                    eprintln!("{line}");
                }
                eprintln!("   Remove it with: git worktree remove <path>");
                return Err("branch already attached to another worktree".to_string());
            }
        }

        let mut passthrough = Vec::with_capacity(1 + self.args.len());
        passthrough.push(command);
        passthrough.extend(self.args);
        let refs = passthrough.iter().map(String::as_str).collect::<Vec<_>>();
        run_git_passthrough(&refs)
    }
}
