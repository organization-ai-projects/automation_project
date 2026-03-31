//! tools/versioning_automation/src/automation/commands/check_priority_issues_options.rs
use std::collections::BTreeMap;

use crate::{automation::execute, gh_cli};

#[derive(Debug)]
pub(crate) struct CheckPriorityIssuesOptions {
    pub(crate) repo: Option<String>,
}

impl CheckPriorityIssuesOptions {
    pub(crate) fn run_check_priority_issues(self) -> Result<(), String> {
        let mut by_number: BTreeMap<u64, (String, String)> = BTreeMap::new();
        for label in ["high priority", "security"] {
            let mut args = vec![
                "issue",
                "list",
                "--state",
                "open",
                "--limit",
                "200",
                "--label",
                label,
                "--json",
                "number,title,url",
            ];
            if let Some(repo) = self.repo.as_deref() {
                args.push("-R");
                args.push(repo);
            }
            let output = gh_cli::output_trim(&args)
                .map_err(|e| format!("Failed to run gh issue list: {e}"))?;
            let issues = execute::parse_json_array(&output, "issues JSON")?;
            for issue in issues {
                let Some(issue_object) = issue.as_object() else {
                    continue;
                };
                let number = execute::object_u64(issue_object, "number");
                if number == 0 {
                    continue;
                }
                by_number.insert(
                    number,
                    (
                        execute::object_string(issue_object, "title"),
                        execute::object_string(issue_object, "url"),
                    ),
                );
            }
        }

        if by_number.is_empty() {
            println!("No high priority or security issues found.");
            return Ok(());
        }

        println!("HIGH PRIORITY & SECURITY ISSUES");
        println!();
        for (idx, (number, (title, url))) in by_number.iter().enumerate() {
            println!("[{}] Issue #{}", idx + 1, number);
            println!("    Title: {}", title);
            println!("    URL:   {}", url);
            println!();
        }
        println!("Total priority issues: {}", by_number.len());

        Ok(())
    }
}
