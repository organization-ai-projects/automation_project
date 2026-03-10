//! tools/versioning_automation/src/pr/parse.rs
use std::fs;
use std::io::{self, Read};
use std::path::Path;

use crate::pr::model::pr_action::PrAction;
use crate::pr::model::pr_auto_add_closes_options::PrAutoAddClosesOptions;
use crate::pr::model::pr_closure_refs_options::PrClosureRefsOptions;
use crate::pr::model::pr_directive_conflicts_options::PrDirectiveConflictsOptions;
use crate::pr::model::pr_directives_format::PrDirectivesFormat;
use crate::pr::model::pr_directives_options::PrDirectivesOptions;
use crate::pr::model::pr_directives_state_options::PrDirectivesStateOptions;

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
        "auto-add-closes" => parse_auto_add_closes(&args[1..]).map(PrAction::AutoAddCloses),
        unknown => Err(format!("Unknown pr subcommand: {unknown}")),
    }
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
