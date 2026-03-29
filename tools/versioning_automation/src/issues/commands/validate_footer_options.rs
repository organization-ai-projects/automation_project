//! tools/versioning_automation/src/issues/commands/validate_footer_options.rs
use std::{collections::HashSet, fs};

use regex::Regex;

use crate::{
    gh_cli::output_trim_or_empty,
    issues::execute::{extract_issue_refs_for_footer, is_root_parent_issue_for_repo},
    repo_name::resolve_repo_name,
};

#[derive(Debug, Clone)]
pub(crate) struct ValidateFooterOptions {
    pub(crate) file: String,
    pub(crate) repo: Option<String>,
}

impl ValidateFooterOptions {
    pub(crate) fn run_validate_footer(self) -> i32 {
        const RC_SUBJECT_TRAILER: i32 = 4;
        const RC_ROOT_PARENT: i32 = 5;
        const RC_ASSIGNMENT_POLICY: i32 = 10;

        let content = match fs::read_to_string(&self.file) {
            Ok(value) => value,
            Err(err) => {
                eprintln!(
                    "❌ Failed to read commit message file '{}': {err}",
                    self.file
                );
                return RC_SUBJECT_TRAILER;
            }
        };

        let subject = content
            .lines()
            .find(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty() && !trimmed.starts_with('#')
            })
            .unwrap_or_default()
            .to_string();
        let issue_ref_in_subject_re = Regex::new(
        r"(?i)(^|[[:space:]])(cancel[\s_-]*closes|closes|part[[:space:]]+of|reopen|reopens|fixes)[[:space:]]+#[0-9]+([[:space:]]|$)",
    )
    .expect("static regex must compile");
        if issue_ref_in_subject_re.is_match(&subject) {
            eprintln!("❌ Issue references must be in commit footer, not in subject.");
            eprintln!("   Move 'Closes/Cancel-Closes/Part of/Reopen #...' to footer lines.");
            return RC_SUBJECT_TRAILER;
        }

        let message_lines: Vec<String> = content
            .lines()
            .filter(|line| !line.trim_start().starts_with('#'))
            .map(ToString::to_string)
            .collect();
        if message_lines.is_empty() {
            return 0;
        }

        let fixes_only_trailer_re =
            Regex::new(r"(?i)^fixes[[:space:]]+#[0-9]+$").expect("static regex must compile");
        let trailer_line_re =
        Regex::new(r"(?i)^(cancel[\s_-]*closes|closes|part[[:space:]]+of|reopen|reopens)[[:space:]]+#([0-9]+)$")
            .expect("static regex must compile");

        let mut trailers: Vec<String> = Vec::new();
        let mut trailer_keys: HashSet<String> = HashSet::new();
        let mut content_lines: Vec<String> = vec![message_lines[0].clone()];

        for line in &message_lines[1..] {
            let normalized = line.trim().to_string();
            if fixes_only_trailer_re.is_match(&normalized) {
                eprintln!("❌ Invalid issue footer keyword: 'Fixes' is not allowed.");
                eprintln!("   Use 'Closes #<issue>' for closure.");
                return RC_SUBJECT_TRAILER;
            }

            if let Some(caps) = trailer_line_re.captures(&normalized) {
                let keyword = caps
                    .get(1)
                    .map(|m| m.as_str().to_ascii_lowercase())
                    .unwrap_or_default();
                let issue_number = caps.get(2).map(|m| m.as_str()).unwrap_or_default();
                let canonical = match keyword.as_str() {
                    "cancel-closes" | "cancel closes" | "cancel_closes" => {
                        format!("Cancel-Closes #{issue_number}")
                    }
                    "closes" => format!("Closes #{issue_number}"),
                    "part of" => format!("Part of #{issue_number}"),
                    "reopen" | "reopens" => format!("Reopen #{issue_number}"),
                    _ => normalized.clone(),
                };
                let key = format!("{keyword}#{issue_number}");
                if trailer_keys.insert(key) {
                    trailers.push(canonical);
                }
                continue;
            }
            content_lines.push(line.clone());
        }

        if !trailers.is_empty() {
            while let Some(last) = content_lines.last() {
                if last.trim().is_empty() {
                    content_lines.pop();
                } else {
                    break;
                }
            }

            let mut compact_lines: Vec<String> = Vec::new();
            let mut prev_blank = false;
            for line in content_lines {
                let is_blank = line.trim().is_empty();
                if is_blank && prev_blank {
                    continue;
                }
                prev_blank = is_blank;
                compact_lines.push(line);
            }

            let mut output_lines = compact_lines;
            output_lines.push(String::new());
            output_lines.extend(trailers);
            let rewritten = format!("{}\n", output_lines.join("\n"));
            if let Err(err) = fs::write(&self.file, rewritten) {
                eprintln!(
                    "❌ Failed to rewrite commit message file '{}': {err}",
                    self.file
                );
                return RC_SUBJECT_TRAILER;
            }
        }

        let repo_name = match resolve_repo_name(self.repo) {
            Ok(value) => value,
            Err(_) => return RC_ROOT_PARENT,
        };

        let refreshed_content = fs::read_to_string(&self.file).unwrap_or_default();
        let refs = extract_issue_refs_for_footer(&refreshed_content);
        let mut root_parent_refs: Vec<String> = Vec::new();
        for (action, issue_number) in &refs {
            let is_root_parent = match is_root_parent_issue_for_repo(issue_number, &repo_name) {
                Ok(value) => value,
                Err(message) => {
                    eprintln!("{message}");
                    return RC_ROOT_PARENT;
                }
            };
            if is_root_parent {
                root_parent_refs.push(format!("{action} #{issue_number}"));
            }
        }
        if !root_parent_refs.is_empty() {
            eprintln!("❌ Invalid issue footer usage in commit message.");
            eprintln!(
                "   Protected parent issue references are not allowed in commit trailers: {}",
                root_parent_refs.join(" ")
            );
            eprintln!(
                "   Protected parent states: Parent: epic, or Parent: none with detected children."
            );
            eprintln!(
                "   Use issue refs on child/independent issues only (Part of/Closes/Reopen #<issue>)."
            );
            eprintln!("   Bypass (emergency only): SKIP_COMMIT_VALIDATION=1 git commit ...");
            return RC_ROOT_PARENT;
        }

        let current_login = output_trim_or_empty(&["api", "user", "--jq", ".login"]);
        if current_login.trim().is_empty() {
            return RC_ASSIGNMENT_POLICY;
        }

        let mut has_part_of: HashSet<String> = HashSet::new();
        let mut has_closing: HashSet<String> = HashSet::new();
        for (action, issue_number) in &refs {
            if action == "part of" {
                has_part_of.insert(issue_number.clone());
            }
            if action == "closes" || action == "fixes" {
                has_closing.insert(issue_number.clone());
            }
        }

        let mut violations: Vec<String> = Vec::new();
        for issue_number in &has_part_of {
            if has_closing.contains(issue_number) {
                continue;
            }
            let assignees = output_trim_or_empty(&[
                "issue",
                "view",
                issue_number,
                "-R",
                &repo_name,
                "--json",
                "assignees",
                "--jq",
                ".assignees[].login",
            ]);
            let logins = assignees
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .map(str::to_string)
                .collect::<Vec<_>>();
            if logins.len() == 1 && logins[0] == current_login.trim() {
                violations.push(format!(
                "#{issue_number} is assigned only to @{}: 'Closes #{issue_number}' is required (Part of only is not allowed)",
                current_login.trim()
            ));
            }
        }

        if !violations.is_empty() {
            eprintln!("❌ Assignment policy violation in commit footer.");
            for violation in &violations {
                eprintln!("   - {violation}");
            }
            return RC_ASSIGNMENT_POLICY;
        }

        0
    }
}
