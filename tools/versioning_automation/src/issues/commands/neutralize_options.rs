//! tools/versioning_automation/src/issues/commands/neutralize_options.rs
use std::collections::{HashMap, HashSet};

use crate::{
    gh_cli::{add_repo_arg, gh_command, push_arg, status_code_owned},
    issues::{
        NeutralizeRefBuckets,
        execute::{
            apply_rejected_marker, build_neutralize_comment_body, gh_pr_body_or_empty,
            neutralize_reason_for_issue_cached, remove_rejected_marker, upsert_pr_marker_comment,
        },
    },
    repo_name::resolve_repo_name,
};

#[derive(Debug, Clone)]
pub(crate) struct NeutralizeOptions {
    pub(crate) pr: String,
    pub(crate) repo: Option<String>,
}

impl NeutralizeOptions {
    pub(crate) fn run_neutralize(self) -> i32 {
        let repo_name = match resolve_repo_name(self.repo) {
            Ok(repo) => repo,
            Err(msg) => {
                eprintln!("{msg}");
                return 3;
            }
        };

        let marker = format!("<!-- closure-neutralizer:{} -->", self.pr);
        let original_body = gh_pr_body_or_empty(&repo_name, &self.pr);
        if original_body.trim().is_empty() {
            eprintln!("Error: unable to read PR #{}.", self.pr);
            return 4;
        }

        let buckets = NeutralizeRefBuckets::collect_neutralize_refs(&original_body);
        let closing_refs = buckets.0;
        let pre_neutralized_refs = buckets.1;
        let mut updated_body = original_body.clone();
        let mut seen_refs: HashSet<String> = HashSet::new();
        let mut reason_cache: HashMap<String, String> = HashMap::new();
        let mut neutralized_reason: HashMap<String, String> = HashMap::new();
        let mut neutralized_action: HashMap<String, String> = HashMap::new();
        let mut neutralized_count = 0usize;

        for neutralize_ref in closing_refs {
            let action = &neutralize_ref.0;
            let issue_key = &neutralize_ref.1;
            let dedupe_key = format!("{action}|{issue_key}");
            if !seen_refs.insert(dedupe_key) {
                continue;
            }
            let issue_number: &str = issue_key.trim_start_matches('#');
            let reason =
                neutralize_reason_for_issue_cached(issue_number, &repo_name, &mut reason_cache);
            if reason.is_empty() {
                continue;
            }

            match apply_rejected_marker(&updated_body, "closes|fixes", issue_key) {
                Ok(body) => updated_body = body,
                Err(_) => continue,
            }
            neutralized_reason.insert(issue_key.clone(), reason);
            neutralized_action.insert(issue_key.clone(), action.clone());
            neutralized_count += 1;
        }

        for neutralize_ref in pre_neutralized_refs {
            let action = &neutralize_ref.0;
            let issue_key = &neutralize_ref.1;
            let dedupe_key = format!("{action}|{issue_key}");
            if !seen_refs.insert(dedupe_key) {
                continue;
            }
            let issue_number: &str = issue_key.trim_start_matches('#');
            let reason =
                neutralize_reason_for_issue_cached(issue_number, &repo_name, &mut reason_cache);
            if reason.is_empty() {
                match remove_rejected_marker(&updated_body, "closes|fixes", issue_key) {
                    Ok(body) => updated_body = body,
                    Err(_) => continue,
                }
                continue;
            }

            match apply_rejected_marker(&updated_body, "closes|fixes", issue_key) {
                Ok(body) => updated_body = body,
                Err(_) => continue,
            }
            neutralized_reason.insert(issue_key.clone(), reason);
            neutralized_action.insert(issue_key.clone(), action.clone());
            neutralized_count += 1;
        }

        if updated_body != original_body {
            let status = status_code_owned({
                let mut cmd = gh_command(&["pr", "edit", &self.pr]);
                add_repo_arg(&mut cmd, Some(repo_name.as_str()));
                push_arg(&mut cmd, "--body");
                push_arg(&mut cmd, &updated_body);
                cmd
            });
            if status != 0 {
                return status;
            }
        }

        let comment_body = build_neutralize_comment_body(
            &marker,
            neutralized_count,
            &neutralized_reason,
            &neutralized_action,
        );
        let status = upsert_pr_marker_comment(&repo_name, &self.pr, &marker, &comment_body);
        if status != 0 {
            return status;
        }

        println!("Closure neutralization evaluated for PR #{}.", self.pr);
        0
    }
}
