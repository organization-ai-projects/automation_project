//! tools/versioning_automation/src/issues/commands/closure_hygiene_options.rs
use crate::{
    gh_cli::{gh_command, output_trim_or_empty, push_arg, status_code_owned},
    issues::execute::{evaluate_parent_issue, split_repo_name},
    repo_name::resolve_repo_name,
};

#[derive(Debug, Clone)]
pub(crate) struct ClosureHygieneOptions {
    pub(crate) repo: Option<String>,
}

impl ClosureHygieneOptions {
    pub(crate) fn run_closure_hygiene(self) -> i32 {
        let repo_name = match resolve_repo_name(self.repo) {
            Ok(repo) => repo,
            Err(msg) => {
                eprintln!("{msg}");
                return 3;
            }
        };
        let (repo_owner, repo_short_name) = split_repo_name(&repo_name);

        let open_issue_numbers = output_trim_or_empty(&[
            "issue",
            "list",
            "--state",
            "open",
            "--limit",
            "300",
            "--json",
            "number",
            "--jq",
            ".[].number",
            "-R",
            &repo_name,
        ]);
        for issue_number in open_issue_numbers
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
        {
            let status = evaluate_parent_issue(
                false,
                &repo_name,
                &repo_owner,
                &repo_short_name,
                issue_number,
            );
            if status != 0 {
                return status;
            }
        }

        let milestones_tsv = output_trim_or_empty(&[
            "api",
            &format!("repos/{repo_name}/milestones?state=open"),
            "--paginate",
            "--jq",
            ".[] | [.number, (.title // \"\"), (.open_issues // 0)] | @tsv",
        ]);
        for line in milestones_tsv.lines() {
            let mut parts = line.splitn(3, '\t');
            let number = parts.next().unwrap_or("").trim();
            let title = parts.next().unwrap_or("").trim();
            let open_issues = parts.next().unwrap_or("").trim();
            if number.is_empty() || open_issues != "0" {
                continue;
            }
            let status = status_code_owned({
                let mut cmd = gh_command(&[
                    "api",
                    "-X",
                    "PATCH",
                    &format!("repos/{repo_name}/milestones/{number}"),
                ]);
                push_arg(&mut cmd, "-f");
                push_arg(&mut cmd, "state=closed");
                cmd
            });
            if status != 0 {
                return status;
            }
            println!("Closed milestone #{} ({}).", number, title);
        }

        println!("Closure hygiene completed.");
        0
    }
}
