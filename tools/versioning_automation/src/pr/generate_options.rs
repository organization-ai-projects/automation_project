//! tools/versioning_automation/src/pr/generate_options.rs
use std::fs;

use crate::{
    errors_code::{E_DEPENDENCY, E_GIT, E_NO_DATA, E_USAGE},
    issues::take_value,
    pr::{
        CommitInfo, MainPrRefSnapshot,
        commands::PrDuplicateActionsOptions,
        generate_description::{
            current_branch_name, gh_create_pr, gh_edit_pr_body, gh_read_pr_body,
            render_duplicate_mode_message, replace_validation_gate,
        },
    },
    pr_run_snapshot::PrRunSnapshot,
    repo_name::resolve_repo_name_optional,
};
#[derive(Debug, Clone)]
pub(crate) struct GenerateOptions {
    pub(crate) help: bool,
    pub(crate) dry_run: bool,
    pub(crate) main_pr_number: Option<String>,
    pub(crate) create_pr: bool,
    pub(crate) allow_partial_create: bool,
    pub(crate) assume_yes: bool,
    pub(crate) base_ref: Option<String>,
    pub(crate) head_ref: Option<String>,
    pub(crate) duplicate_mode: Option<String>,
    pub(crate) auto_edit_pr_number: Option<String>,
    pub(crate) validation_only: bool,
    pub(crate) output_file: Option<String>,
}

impl GenerateOptions {
    pub(crate) fn parse_generate_options(args: &[String]) -> Result<Self, String> {
        let mut help = false;
        let mut dry_run = false;
        let mut main_pr_number: Option<String> = None;
        let mut create_pr = false;
        let mut allow_partial_create = false;
        let mut assume_yes = false;
        let mut auto_mode = false;
        let mut mode_explicit = false;
        let mut base_ref: Option<String> = None;
        let mut head_ref: Option<String> = None;
        let mut duplicate_mode: Option<String> = None;
        let mut auto_edit_pr_number: Option<String> = None;
        let mut validation_only = false;
        let mut positionals: Vec<String> = Vec::new();

        let mut i = 0usize;
        while i < args.len() {
            match args[i].as_str() {
                "--dry-run" => {
                    dry_run = true;
                    mode_explicit = true;
                    i += 1;
                }
                "--base" => {
                    base_ref = Some(take_value("--base", args, &mut i)?);
                }
                "--head" => {
                    head_ref = Some(take_value("--head", args, &mut i)?);
                }
                "--create-pr" => {
                    create_pr = true;
                    mode_explicit = true;
                    i += 1;
                }
                "--allow-partial-create" => {
                    allow_partial_create = true;
                    mode_explicit = true;
                    i += 1;
                }
                "--yes" => {
                    assume_yes = true;
                    i += 1;
                }
                "--auto" => {
                    auto_mode = true;
                    mode_explicit = true;
                    i += 1;
                }
                "--auto-edit" | "--refresh-pr" => {
                    auto_edit_pr_number = Some(take_value(args[i].as_str(), args, &mut i)?);
                    mode_explicit = true;
                }
                "--validation-only" => {
                    validation_only = true;
                    i += 1;
                }
                "--duplicate-mode" => {
                    duplicate_mode = Some(take_value("--duplicate-mode", args, &mut i)?);
                }
                // accepted/no-op flags for compatibility in migrated CI path
                "--debug" | "--keep-artifacts" => {
                    i += 1;
                }
                "-h" | "--help" => {
                    help = true;
                    i += 1;
                }
                other if other.starts_with('-') => {
                    return Err(format!("Unknown option for generate-description: {other}"));
                }
                _ => {
                    positionals.push(args[i].clone());
                    i += 1;
                }
            }
        }

        if help {
            return Ok(Self {
                help: true,
                dry_run: false,
                main_pr_number: None,
                create_pr: false,
                allow_partial_create: false,
                assume_yes: false,
                base_ref: None,
                head_ref: None,
                duplicate_mode: None,
                auto_edit_pr_number: None,
                validation_only: false,
                output_file: None,
            });
        }

        if !mode_explicit && positionals.is_empty() {
            auto_mode = true;
        }
        if auto_mode {
            create_pr = true;
            if !positionals.is_empty() {
                return Err("--auto does not accept a positional OUTPUT_FILE.".to_string());
            }
        }

        if allow_partial_create && !create_pr {
            return Err("--allow-partial-create requires --create-pr.".to_string());
        }
        if auto_edit_pr_number.is_some() && create_pr {
            return Err("--auto-edit cannot be combined with --create-pr/--auto.".to_string());
        }
        if validation_only && auto_edit_pr_number.is_none() {
            return Err("--validation-only requires --auto-edit/--refresh-pr.".to_string());
        }
        if let Some(mode) = duplicate_mode.as_deref()
            && mode != "safe"
            && mode != "auto-close"
        {
            return Err("--duplicate-mode must be 'safe' or 'auto-close'.".to_string());
        }

        let output_file = if dry_run && auto_edit_pr_number.is_none() {
            match positionals.len() {
                0 => None,
                1 => Some(positionals.remove(0)),
                _ => {
                    return Err(
                        "Too many positional arguments for --dry-run. Only OUTPUT_FILE is allowed."
                            .to_string(),
                    );
                }
            }
        } else {
            if dry_run && auto_edit_pr_number.is_some() && !positionals.is_empty() {
                return Err(
                    "In --auto-edit dry-run mode, positional OUTPUT_FILE is not allowed."
                        .to_string(),
                );
            }
            if auto_edit_pr_number.is_some() && positionals.len() > 1 {
                return Err(
                    "In --auto-edit mode (MAIN_PR_NUMBER), positional OUTPUT_FILE is not allowed."
                        .to_string(),
                );
            }
            if auto_edit_pr_number.is_none() && !create_pr && positionals.len() > 2 {
                return Err(
                    "Too many positional arguments. Expected usage: MAIN_PR_NUMBER [OUTPUT_FILE]."
                        .to_string(),
                );
            }
            if auto_edit_pr_number.is_none()
                && !create_pr
                && let Some(first) = positionals.first()
            {
                main_pr_number = Some(first.clone());
            }
            if auto_edit_pr_number.is_none() && !create_pr && main_pr_number.is_none() {
                return Err("MAIN_PR_NUMBER is required.".to_string());
            }
            if auto_edit_pr_number.is_none() && !create_pr {
                if positionals.len() >= 2 {
                    Some(positionals[1].clone())
                } else {
                    Some("pr_description.md".to_string())
                }
            } else {
                None
            }
        };

        Ok(Self {
            help: false,
            dry_run,
            main_pr_number,
            create_pr,
            allow_partial_create,
            assume_yes,
            base_ref,
            head_ref,
            duplicate_mode,
            auto_edit_pr_number,
            validation_only,
            output_file,
        })
    }

    pub(crate) fn run_generate_flow(self) -> i32 {
        let (base_ref, head_ref) = if let Some(pr_number) = self.auto_edit_pr_number.as_deref() {
            let refs = match MainPrRefSnapshot::fetch_pr_refs(pr_number) {
                Ok(value) => value,
                Err(msg) => {
                    eprintln!("{msg}");
                    return E_DEPENDENCY;
                }
            };
            let base_ref = if self.base_ref.as_deref().unwrap_or("").trim().is_empty() {
                if refs.base_ref_name.trim().is_empty() {
                    "dev".to_string()
                } else {
                    refs.base_ref_name
                }
            } else {
                self.base_ref.clone().unwrap_or_else(|| "dev".to_string())
            };
            let head_ref = if self.head_ref.as_deref().unwrap_or("").trim().is_empty() {
                if refs.head_ref_name.trim().is_empty() {
                    "dev".to_string()
                } else {
                    refs.head_ref_name
                }
            } else {
                self.head_ref.clone().unwrap_or_else(|| "dev".to_string())
            };
            (base_ref, head_ref)
        } else if let Some(main_pr_number) = self.main_pr_number.as_deref() {
            let refs = match MainPrRefSnapshot::fetch_pr_refs(main_pr_number) {
                Ok(value) => value,
                Err(msg) => {
                    eprintln!("{msg}");
                    return E_DEPENDENCY;
                }
            };
            let base_ref = if self.base_ref.as_deref().unwrap_or("").trim().is_empty() {
                if refs.base_ref_name.trim().is_empty() {
                    "dev".to_string()
                } else {
                    refs.base_ref_name
                }
            } else {
                self.base_ref.clone().unwrap_or_else(|| "dev".to_string())
            };
            let head_ref = if self.head_ref.as_deref().unwrap_or("").trim().is_empty() {
                if refs.head_ref_name.trim().is_empty() {
                    "dev".to_string()
                } else {
                    refs.head_ref_name
                }
            } else {
                self.head_ref.clone().unwrap_or_else(|| "dev".to_string())
            };
            (base_ref, head_ref)
        } else {
            let base_ref = self.base_ref.clone().unwrap_or_else(|| "dev".to_string());
            let head_ref = match self.head_ref.clone() {
                Some(value) => value,
                None => match current_branch_name() {
                    Ok(value) => value,
                    Err(msg) => {
                        eprintln!("{msg}");
                        return E_GIT;
                    }
                },
            };
            (base_ref, head_ref)
        };

        let run_snapshot = match PrRunSnapshot::load_pr_run_snapshot(&base_ref, &head_ref) {
            Ok(value) => value,
            Err(msg) => {
                eprintln!("{msg}");
                return E_DEPENDENCY;
            }
        };
        let range = format!(
            "{}..{}",
            run_snapshot.compare.base_ref, run_snapshot.compare.head_ref
        );
        let commits = run_snapshot.compare.commits;

        if commits.is_empty() {
            eprintln!("Error: unable to retrieve commit messages for {base_ref}...{head_ref}.");
            return E_NO_DATA;
        }

        let validation_gate = run_snapshot.validation_gate;
        let duplicate_targets = run_snapshot.duplicate_targets;
        let issue_outcomes = run_snapshot.issue_outcomes;
        if let Some(mode) = self.duplicate_mode.as_deref()
            && !self.dry_run
        {
            let repo = match resolve_repo_name_optional(None) {
                Some(value) => value,
                None => {
                    eprintln!("Warning: unable to resolve repository; duplicate mode skipped.");
                    String::new()
                }
            };
            if !repo.is_empty() {
                let payload = duplicate_targets
                    .iter()
                    .map(|(dup, canonical)| format!("{dup}|{canonical}"))
                    .collect::<Vec<String>>()
                    .join("\n");
                let duplicate_status =
                    PrDuplicateActionsOptions::run_duplicate_actions(PrDuplicateActionsOptions {
                        text: payload,
                        mode: mode.to_string(),
                        repo,
                        assume_yes: self.assume_yes,
                    });
                if duplicate_status != 0 {
                    return duplicate_status;
                }
            }
        }

        let duplicate_message = self.duplicate_mode.as_deref().and_then(|mode| {
            if self.dry_run {
                Some(render_duplicate_mode_message(mode, &duplicate_targets))
            } else {
                None
            }
        });
        let body = if self.validation_only {
            let pr_number = match self.auto_edit_pr_number.as_deref() {
                Some(value) => value,
                None => {
                    eprintln!("--validation-only requires --auto-edit/--refresh-pr.");
                    return E_USAGE;
                }
            };
            let current_body = match gh_read_pr_body(pr_number) {
                Ok(value) => value,
                Err(msg) => {
                    eprintln!("{msg}");
                    return E_DEPENDENCY;
                }
            };
            replace_validation_gate(&current_body, &validation_gate)
        } else {
            CommitInfo::build_full_body(
                &base_ref,
                &head_ref,
                &commits,
                &range,
                &validation_gate,
                &issue_outcomes,
            )
        };

        let exit_code = if let Some(pr_number) = self.auto_edit_pr_number {
            match gh_edit_pr_body(&pr_number, &body) {
                Ok(()) => {
                    println!("Updated PR body: #{pr_number}");
                    0
                }
                Err(msg) => {
                    eprintln!("{msg}");
                    E_DEPENDENCY
                }
            }
        } else if self.create_pr {
            if !self.assume_yes {
                eprintln!("--yes is required for native --create-pr/--auto mode.");
                return E_USAGE;
            }

            let title = CommitInfo::build_dynamic_pr_title(&base_ref, &head_ref, &commits);
            match gh_create_pr(&base_ref, &head_ref, &title, &body) {
                Ok(url_or_message) => {
                    println!("PR created: {url_or_message}");
                    0
                }
                Err(msg) => {
                    if self.allow_partial_create {
                        eprintln!("Warning: create-pr failed (partial allowed): {msg}");
                        0
                    } else {
                        eprintln!("{msg}");
                        E_DEPENDENCY
                    }
                }
            }
        } else if let Some(path) = self.output_file {
            match fs::write(&path, &body) {
                Ok(()) => {
                    println!("Generated file: {path}");
                    0
                }
                Err(err) => {
                    eprintln!("Failed to write output file '{path}': {err}");
                    1
                }
            }
        } else {
            println!("{body}");
            0
        };

        if let Some(message) = duplicate_message {
            println!("{message}");
        }

        exit_code
    }
}
