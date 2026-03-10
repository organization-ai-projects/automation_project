//! tools/versioning_automation/src/pr/parse.rs
use std::fs;
use std::io::{self, Read};
use std::path::Path;

use crate::pr::commands::pr_action::PrAction;
use crate::pr::commands::pr_auto_add_closes_options::PrAutoAddClosesOptions;
use crate::pr::commands::pr_closure_marker_options::PrClosureMarkerOptions;
use crate::pr::commands::pr_closure_refs_options::PrClosureRefsOptions;
use crate::pr::commands::pr_directive_conflict_guard_options::PrDirectiveConflictGuardOptions;
use crate::pr::commands::pr_directive_conflicts_options::PrDirectiveConflictsOptions;
use crate::pr::commands::pr_directives_format::PrDirectivesFormat;
use crate::pr::commands::pr_directives_options::PrDirectivesOptions;
use crate::pr::commands::pr_directives_state_options::PrDirectivesStateOptions;
use crate::pr::commands::pr_duplicate_actions_options::PrDuplicateActionsOptions;
use crate::pr::commands::pr_group_by_category_options::PrGroupByCategoryOptions;
use crate::pr::commands::pr_issue_category_from_labels_options::PrIssueCategoryFromLabelsOptions;
use crate::pr::commands::pr_issue_category_from_title_options::PrIssueCategoryFromTitleOptions;
use crate::pr::commands::pr_issue_decision_options::PrIssueDecisionOptions;
use crate::pr::commands::pr_issue_ref_kind_options::PrIssueRefKindOptions;
use crate::pr::commands::pr_non_closing_refs_options::PrNonClosingRefsOptions;
use crate::pr::commands::pr_resolve_category_options::PrResolveCategoryOptions;

pub(crate) fn parse(args: &[String]) -> Result<PrAction, String> {
    if args.is_empty() {
        return Ok(PrAction::Help);
    }

    match args[0].as_str() {
        "help" | "--help" | "-h" => Ok(PrAction::Help),
        "directives" => parse_directives(&args[1..]).map(PrAction::Directives),
        "closure-refs" => parse_closure_refs(&args[1..]).map(PrAction::ClosureRefs),
        "directives-state" => parse_directives_state(&args[1..]).map(PrAction::DirectivesState),
        "directive-conflicts" => {
            parse_directive_conflicts(&args[1..]).map(PrAction::DirectiveConflicts)
        }
        "directive-conflict-guard" => {
            parse_directive_conflict_guard(&args[1..]).map(PrAction::DirectiveConflictGuard)
        }
        "duplicate-actions" => parse_duplicate_actions(&args[1..]).map(PrAction::DuplicateActions),
        "group-by-category" => parse_group_by_category(&args[1..]).map(PrAction::GroupByCategory),
        "issue-category-from-labels" => {
            parse_issue_category_from_labels(&args[1..]).map(PrAction::IssueCategoryFromLabels)
        }
        "issue-category-from-title" => {
            parse_issue_category_from_title(&args[1..]).map(PrAction::IssueCategoryFromTitle)
        }
        "issue-ref-kind" => parse_issue_ref_kind(&args[1..]).map(PrAction::IssueRefKind),
        "issue-decision" => parse_issue_decision(&args[1..]).map(PrAction::IssueDecision),
        "closure-marker" => parse_closure_marker(&args[1..]).map(PrAction::ClosureMarker),
        "non-closing-refs" => parse_non_closing_refs(&args[1..]).map(PrAction::NonClosingRefs),
        "resolve-category" => parse_resolve_category(&args[1..]).map(PrAction::ResolveCategory),
        "auto-add-closes" => parse_auto_add_closes(&args[1..]).map(PrAction::AutoAddCloses),
        unknown => Err(format!("Unknown pr subcommand: {unknown}")),
    }
}

fn parse_directive_conflict_guard(
    args: &[String],
) -> Result<PrDirectiveConflictGuardOptions, String> {
    let mut pr_number = String::new();
    let mut repo: Option<String> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--pr" => {
                pr_number = take_value("--pr", args, &mut i)?;
            }
            "--repo" => {
                repo = Some(take_value("--repo", args, &mut i)?);
            }
            unknown => {
                return Err(format!(
                    "Unknown option for directive-conflict-guard: {unknown}"
                ));
            }
        }
    }

    require_positive_number("--pr", &pr_number)?;
    Ok(PrDirectiveConflictGuardOptions { pr_number, repo })
}

fn parse_duplicate_actions(args: &[String]) -> Result<PrDuplicateActionsOptions, String> {
    let mut text: Option<String> = None;
    let mut read_stdin = false;
    let mut input_file: Option<String> = None;
    let mut mode = String::new();
    let mut repo = String::new();
    let mut assume_yes = false;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--text" => text = Some(take_value("--text", args, &mut i)?),
            "--stdin" => {
                read_stdin = true;
                i += 1;
            }
            "--input-file" => input_file = Some(take_value("--input-file", args, &mut i)?),
            "--mode" => mode = take_value("--mode", args, &mut i)?,
            "--repo" => repo = take_value("--repo", args, &mut i)?,
            "--assume-yes" => {
                assume_yes =
                    parse_bool_value("--assume-yes", &take_value("--assume-yes", args, &mut i)?)?
            }
            unknown => return Err(format!("Unknown option for duplicate-actions: {unknown}")),
        }
    }

    let resolved_text = resolve_input_text(text, read_stdin, input_file)?;
    if resolved_text.is_empty() {
        return Err("duplicate-actions requires --text <value> or --stdin".to_string());
    }
    if mode.is_empty() {
        return Err("--mode is required".to_string());
    }
    if repo.is_empty() {
        return Err("--repo is required".to_string());
    }

    Ok(PrDuplicateActionsOptions {
        text: resolved_text,
        mode,
        repo,
        assume_yes,
    })
}

fn parse_group_by_category(args: &[String]) -> Result<PrGroupByCategoryOptions, String> {
    let mut text: Option<String> = None;
    let mut read_stdin = false;
    let mut input_file: Option<String> = None;
    let mut mode = String::new();

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--text" => text = Some(take_value("--text", args, &mut i)?),
            "--stdin" => {
                read_stdin = true;
                i += 1;
            }
            "--input-file" => input_file = Some(take_value("--input-file", args, &mut i)?),
            "--mode" => mode = take_value("--mode", args, &mut i)?,
            unknown => return Err(format!("Unknown option for group-by-category: {unknown}")),
        }
    }

    let resolved_text = resolve_input_text(text, read_stdin, input_file)?;
    if resolved_text.is_empty() {
        return Err("group-by-category requires --text <value> or --stdin".to_string());
    }
    if mode.is_empty() {
        return Err("--mode is required".to_string());
    }

    Ok(PrGroupByCategoryOptions {
        text: resolved_text,
        mode,
    })
}

fn parse_issue_ref_kind(args: &[String]) -> Result<PrIssueRefKindOptions, String> {
    let mut issue_number = String::new();
    let mut repo: Option<String> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--issue" => issue_number = take_value("--issue", args, &mut i)?,
            "--repo" => repo = Some(take_value("--repo", args, &mut i)?),
            unknown => return Err(format!("Unknown option for issue-ref-kind: {unknown}")),
        }
    }

    require_positive_number("--issue", &issue_number)?;
    Ok(PrIssueRefKindOptions { issue_number, repo })
}

fn parse_issue_category_from_labels(
    args: &[String],
) -> Result<PrIssueCategoryFromLabelsOptions, String> {
    let mut labels_raw = String::new();

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--labels-raw" => labels_raw = take_value("--labels-raw", args, &mut i)?,
            unknown => {
                return Err(format!(
                    "Unknown option for issue-category-from-labels: {unknown}"
                ));
            }
        }
    }

    if labels_raw.is_empty() {
        return Err("--labels-raw is required".to_string());
    }
    Ok(PrIssueCategoryFromLabelsOptions { labels_raw })
}

fn parse_issue_category_from_title(
    args: &[String],
) -> Result<PrIssueCategoryFromTitleOptions, String> {
    let mut title = String::new();

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--title" => title = take_value("--title", args, &mut i)?,
            unknown => {
                return Err(format!(
                    "Unknown option for issue-category-from-title: {unknown}"
                ));
            }
        }
    }

    if title.is_empty() {
        return Err("--title is required".to_string());
    }
    Ok(PrIssueCategoryFromTitleOptions { title })
}

fn parse_non_closing_refs(args: &[String]) -> Result<PrNonClosingRefsOptions, String> {
    let mut text: Option<String> = None;
    let mut read_stdin = false;
    let mut input_file: Option<String> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--text" => text = Some(take_value("--text", args, &mut i)?),
            "--stdin" => {
                read_stdin = true;
                i += 1;
            }
            "--input-file" => input_file = Some(take_value("--input-file", args, &mut i)?),
            unknown => return Err(format!("Unknown option for non-closing-refs: {unknown}")),
        }
    }

    let resolved_text = resolve_input_text(text, read_stdin, input_file)?;
    if resolved_text.is_empty() {
        return Err("non-closing-refs requires --text <value> or --stdin".to_string());
    }

    Ok(PrNonClosingRefsOptions {
        text: resolved_text,
    })
}

fn parse_resolve_category(args: &[String]) -> Result<PrResolveCategoryOptions, String> {
    let mut label_category = String::new();
    let mut title_category = String::new();
    let mut default_category = String::new();

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--label-category" => label_category = take_value("--label-category", args, &mut i)?,
            "--title-category" => title_category = take_value("--title-category", args, &mut i)?,
            "--default-category" => {
                default_category = take_value("--default-category", args, &mut i)?
            }
            unknown => return Err(format!("Unknown option for resolve-category: {unknown}")),
        }
    }

    if label_category.is_empty() {
        return Err("--label-category is required".to_string());
    }
    if title_category.is_empty() {
        return Err("--title-category is required".to_string());
    }
    if default_category.is_empty() {
        return Err("--default-category is required".to_string());
    }

    Ok(PrResolveCategoryOptions {
        label_category,
        title_category,
        default_category,
    })
}

fn parse_closure_marker(args: &[String]) -> Result<PrClosureMarkerOptions, String> {
    let mut text: Option<String> = None;
    let mut read_stdin = false;
    let mut input_file: Option<String> = None;
    let mut keyword_pattern = String::new();
    let mut issue = String::new();
    let mut mode = String::new();

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--text" => text = Some(take_value("--text", args, &mut i)?),
            "--stdin" => {
                read_stdin = true;
                i += 1;
            }
            "--input-file" => input_file = Some(take_value("--input-file", args, &mut i)?),
            "--keyword-pattern" => keyword_pattern = take_value("--keyword-pattern", args, &mut i)?,
            "--issue" => issue = take_value("--issue", args, &mut i)?,
            "--mode" => mode = take_value("--mode", args, &mut i)?,
            unknown => return Err(format!("Unknown option for closure-marker: {unknown}")),
        }
    }

    let resolved_text = resolve_input_text(text, read_stdin, input_file)?;
    if resolved_text.is_empty() {
        return Err("closure-marker requires --text <value> or --stdin".to_string());
    }
    if keyword_pattern.is_empty() {
        return Err("--keyword-pattern is required".to_string());
    }
    if issue.is_empty() {
        return Err("--issue is required".to_string());
    }
    if mode.is_empty() {
        return Err("--mode is required".to_string());
    }

    Ok(PrClosureMarkerOptions {
        text: resolved_text,
        keyword_pattern,
        issue,
        mode,
    })
}

fn parse_issue_decision(args: &[String]) -> Result<PrIssueDecisionOptions, String> {
    let mut action = String::new();
    let mut issue = String::new();
    let mut default_category = String::new();
    let mut seen_reopen = false;
    let mut reopen_category = String::new();
    let mut inferred_decision = String::new();
    let mut explicit_decision = String::new();
    let mut allow_inferred = true;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--action" => action = take_value("--action", args, &mut i)?,
            "--issue" => issue = take_value("--issue", args, &mut i)?,
            "--default-category" => {
                default_category = take_value("--default-category", args, &mut i)?
            }
            "--seen-reopen" => {
                seen_reopen =
                    parse_bool_value("--seen-reopen", &take_value("--seen-reopen", args, &mut i)?)?
            }
            "--reopen-category" => reopen_category = take_value("--reopen-category", args, &mut i)?,
            "--inferred-decision" => {
                inferred_decision = take_value("--inferred-decision", args, &mut i)?
            }
            "--explicit-decision" => {
                explicit_decision = take_value("--explicit-decision", args, &mut i)?
            }
            "--allow-inferred" => {
                allow_inferred = parse_bool_value(
                    "--allow-inferred",
                    &take_value("--allow-inferred", args, &mut i)?,
                )?
            }
            unknown => return Err(format!("Unknown option for issue-decision: {unknown}")),
        }
    }

    if action.is_empty() {
        return Err("--action is required".to_string());
    }
    if issue.is_empty() {
        return Err("--issue is required".to_string());
    }
    if default_category.is_empty() {
        return Err("--default-category is required".to_string());
    }

    Ok(PrIssueDecisionOptions {
        action,
        issue,
        default_category,
        seen_reopen,
        reopen_category,
        inferred_decision,
        explicit_decision,
        allow_inferred,
    })
}

fn parse_closure_refs(args: &[String]) -> Result<PrClosureRefsOptions, String> {
    let mut text: Option<String> = None;
    let mut read_stdin = false;
    let mut input_file: Option<String> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--text" => {
                text = Some(take_value("--text", args, &mut i)?);
            }
            "--stdin" => {
                read_stdin = true;
                i += 1;
            }
            "--input-file" => {
                input_file = Some(take_value("--input-file", args, &mut i)?);
            }
            unknown => return Err(format!("Unknown option for closure-refs: {unknown}")),
        }
    }

    let resolved_text = resolve_input_text(text, read_stdin, input_file)?;
    if resolved_text.is_empty() {
        return Err("closure-refs requires --text <value> or --stdin".to_string());
    }

    Ok(PrClosureRefsOptions {
        text: resolved_text,
    })
}

fn parse_directives_state(args: &[String]) -> Result<PrDirectivesStateOptions, String> {
    let mut text: Option<String> = None;
    let mut read_stdin = false;
    let mut input_file: Option<String> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--text" => {
                text = Some(take_value("--text", args, &mut i)?);
            }
            "--stdin" => {
                read_stdin = true;
                i += 1;
            }
            "--input-file" => {
                input_file = Some(take_value("--input-file", args, &mut i)?);
            }
            unknown => return Err(format!("Unknown option for directives-state: {unknown}")),
        }
    }

    let resolved_text = resolve_input_text(text, read_stdin, input_file)?;
    if resolved_text.is_empty() {
        return Err("directives-state requires --text <value> or --stdin".to_string());
    }

    Ok(PrDirectivesStateOptions {
        text: resolved_text,
    })
}

fn parse_directive_conflicts(args: &[String]) -> Result<PrDirectiveConflictsOptions, String> {
    let mut text: Option<String> = None;
    let mut read_stdin = false;
    let mut input_file: Option<String> = None;
    let mut source_branch_count = 1u32;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--text" => {
                text = Some(take_value("--text", args, &mut i)?);
            }
            "--stdin" => {
                read_stdin = true;
                i += 1;
            }
            "--input-file" => {
                input_file = Some(take_value("--input-file", args, &mut i)?);
            }
            "--source-branch-count" => {
                let value = take_value("--source-branch-count", args, &mut i)?;
                source_branch_count = value.parse::<u32>().map_err(|_| {
                    "--source-branch-count requires a positive numeric value".to_string()
                })?;
                if source_branch_count == 0 {
                    return Err(
                        "--source-branch-count requires a positive numeric value".to_string()
                    );
                }
            }
            unknown => return Err(format!("Unknown option for directive-conflicts: {unknown}")),
        }
    }

    let resolved_text = resolve_input_text(text, read_stdin, input_file)?;
    if resolved_text.is_empty() {
        return Err("directive-conflicts requires --text <value> or --stdin".to_string());
    }

    Ok(PrDirectiveConflictsOptions {
        text: resolved_text,
        source_branch_count,
    })
}

fn parse_auto_add_closes(args: &[String]) -> Result<PrAutoAddClosesOptions, String> {
    let mut pr_number = String::new();
    let mut repo: Option<String> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--pr" => {
                pr_number = take_value("--pr", args, &mut i)?;
            }
            "--repo" => {
                repo = Some(take_value("--repo", args, &mut i)?);
            }
            unknown => return Err(format!("Unknown option for auto-add-closes: {unknown}")),
        }
    }

    require_positive_number("--pr", &pr_number)?;
    Ok(PrAutoAddClosesOptions { pr_number, repo })
}

fn parse_directives(args: &[String]) -> Result<PrDirectivesOptions, String> {
    let mut text: Option<String> = None;
    let mut read_stdin = false;
    let mut input_file: Option<String> = None;
    let mut format = PrDirectivesFormat::Plain;
    let mut unique = false;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--text" => {
                text = Some(take_value("--text", args, &mut i)?);
            }
            "--stdin" => {
                read_stdin = true;
                i += 1;
            }
            "--input-file" => {
                input_file = Some(take_value("--input-file", args, &mut i)?);
            }
            "--format" => {
                let value = take_value("--format", args, &mut i)?;
                format = match value.as_str() {
                    "plain" => PrDirectivesFormat::Plain,
                    "json" => PrDirectivesFormat::Json,
                    _ => return Err("--format must be 'plain' or 'json'".to_string()),
                };
            }
            "--unique" => {
                unique = true;
                i += 1;
            }
            unknown => return Err(format!("Unknown option for directives: {unknown}")),
        }
    }

    let resolved_text = resolve_input_text(text, read_stdin, input_file)?;

    if resolved_text.is_empty() {
        return Err("directives requires --text <value> or --stdin".to_string());
    }

    Ok(PrDirectivesOptions {
        text: resolved_text,
        format,
        unique,
    })
}

fn resolve_input_text(
    text: Option<String>,
    read_stdin: bool,
    input_file: Option<String>,
) -> Result<String, String> {
    if read_stdin {
        read_stdin_text()
    } else if let Some(file_path) = input_file {
        read_file_text(&file_path)
    } else {
        Ok(text.unwrap_or_default())
    }
}

fn take_value(flag: &str, args: &[String], index: &mut usize) -> Result<String, String> {
    let value_index = *index + 1;
    if value_index >= args.len() {
        return Err(format!("{flag} requires a value"));
    }
    let value = args[value_index].clone();
    *index += 2;
    Ok(value)
}

fn read_stdin_text() -> Result<String, String> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|err| format!("failed to read stdin: {err}"))?;
    Ok(input)
}

fn read_file_text(file_path: &str) -> Result<String, String> {
    let path = Path::new(file_path);
    fs::read_to_string(path)
        .map_err(|err| format!("failed to read input file '{file_path}': {err}"))
}

fn require_positive_number(flag: &str, value: &str) -> Result<(), String> {
    if !value.is_empty() && value.chars().all(|c| c.is_ascii_digit()) && value != "0" {
        return Ok(());
    }
    Err(format!("{flag} requires a positive numeric value"))
}

fn parse_bool_value(flag: &str, value: &str) -> Result<bool, String> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(format!("{flag} must be true or false")),
    }
}
