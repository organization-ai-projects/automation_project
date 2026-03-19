use std::collections::BTreeMap;

use crate::automation::commands::CheckPriorityIssuesOptions;

use super::execute::{object_string, object_u64, parse_json_array, run_gh_output};

pub(crate) fn run_check_priority_issues(opts: CheckPriorityIssuesOptions) -> Result<(), String> {
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
        if let Some(repo) = opts.repo.as_deref() {
            args.push("-R");
            args.push(repo);
        }
        let output = run_gh_output(&args)?;
        let issues = parse_json_array(&output, "issues JSON")?;
        for issue in issues {
            let Some(issue_object) = issue.as_object() else {
                continue;
            };
            let number = object_u64(issue_object, "number");
            if number == 0 {
                continue;
            }
            by_number.insert(
                number,
                (
                    object_string(issue_object, "title"),
                    object_string(issue_object, "url"),
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
