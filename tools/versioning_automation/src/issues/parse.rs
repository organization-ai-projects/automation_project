//! tools/versioning_automation/src/issues/parse.rs
use crate::issues::commands::{
    CloseOptions, CreateOptions, FetchNonComplianceReasonOptions, IssueAction, IssueTarget,
    LabelExistsOptions, NonComplianceReasonOptions, ReadOptions, ReevaluateOptions,
    RequiredFieldsValidateOptions, RequiredFieldsValidationMode, UpdateOptions,
};

pub(crate) fn parse(args: &[String]) -> Result<IssueAction, String> {
    if args.is_empty() {
        return Ok(IssueAction::Help);
    }
    match args[0].as_str() {
        "help" | "--help" | "-h" => Ok(IssueAction::Help),
        "create" => parse_create(&args[1..]).map(IssueAction::Create),
        "read" => parse_read(&args[1..]).map(IssueAction::Read),
        "update" => parse_update(&args[1..]).map(IssueAction::Update),
        "close" => parse_close(&args[1..]).map(IssueAction::Close),
        "reopen" => parse_target("reopen", &args[1..]).map(IssueAction::Reopen),
        "delete" => parse_target("delete", &args[1..]).map(IssueAction::Delete),
        "reevaluate" => parse_reevaluate(&args[1..]).map(IssueAction::Reevaluate),
        "required-fields-validate" => {
            parse_required_fields_validate(&args[1..]).map(IssueAction::RequiredFieldsValidate)
        }
        "non-compliance-reason" => {
            parse_non_compliance_reason(&args[1..]).map(IssueAction::NonComplianceReason)
        }
        "fetch-non-compliance-reason" => {
            parse_fetch_non_compliance_reason(&args[1..]).map(IssueAction::FetchNonComplianceReason)
        }
        "label-exists" => parse_label_exists(&args[1..]).map(IssueAction::LabelExists),
        unknown => Err(format!("Unknown issue subcommand: {unknown}")),
    }
}

fn parse_label_exists(args: &[String]) -> Result<LabelExistsOptions, String> {
    let mut repo = String::new();
    let mut label = String::new();

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--repo" => repo = take_value("--repo", args, &mut i)?,
            "--label" => label = take_value("--label", args, &mut i)?,
            unknown => return Err(format!("Unknown option for label-exists: {unknown}")),
        }
    }

    if repo.trim().is_empty() || label.trim().is_empty() {
        return Err("label-exists requires: --repo and --label".to_string());
    }

    Ok(LabelExistsOptions { repo, label })
}

fn parse_required_fields_validate(
    args: &[String],
) -> Result<RequiredFieldsValidateOptions, String> {
    let mut title = String::new();
    let mut body = String::new();
    let mut labels_raw = String::new();
    let mut mode = RequiredFieldsValidationMode::Content;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--title" => title = take_value("--title", args, &mut i)?,
            "--body" => body = take_value("--body", args, &mut i)?,
            "--labels-raw" => labels_raw = take_value("--labels-raw", args, &mut i)?,
            "--mode" => {
                let raw_mode = take_value("--mode", args, &mut i)?;
                mode = match raw_mode.as_str() {
                    "title" => RequiredFieldsValidationMode::Title,
                    "body" => RequiredFieldsValidationMode::Body,
                    "content" => RequiredFieldsValidationMode::Content,
                    _ => return Err("--mode must be one of: title | body | content".to_string()),
                };
            }
            unknown => {
                return Err(format!(
                    "Unknown option for required-fields-validate: {unknown}"
                ));
            }
        }
    }

    Ok(RequiredFieldsValidateOptions {
        title,
        body,
        labels_raw,
        mode,
    })
}

fn parse_non_compliance_reason(args: &[String]) -> Result<NonComplianceReasonOptions, String> {
    let mut title = String::new();
    let mut body = String::new();
    let mut labels_raw = String::new();

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--title" => title = take_value("--title", args, &mut i)?,
            "--body" => body = take_value("--body", args, &mut i)?,
            "--labels-raw" => labels_raw = take_value("--labels-raw", args, &mut i)?,
            unknown => {
                return Err(format!(
                    "Unknown option for non-compliance-reason: {unknown}"
                ));
            }
        }
    }

    Ok(NonComplianceReasonOptions {
        title,
        body,
        labels_raw,
    })
}

fn parse_fetch_non_compliance_reason(
    args: &[String],
) -> Result<FetchNonComplianceReasonOptions, String> {
    let mut issue = String::new();
    let mut repo: Option<String> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--issue" => issue = take_value("--issue", args, &mut i)?,
            "--repo" => repo = Some(take_value("--repo", args, &mut i)?),
            unknown => {
                return Err(format!(
                    "Unknown option for fetch-non-compliance-reason: {unknown}"
                ));
            }
        }
    }

    require_positive_number("--issue", &issue)?;
    Ok(FetchNonComplianceReasonOptions { issue, repo })
}

fn parse_reevaluate(args: &[String]) -> Result<ReevaluateOptions, String> {
    let mut issue = String::new();
    let mut repo: Option<String> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--issue" => issue = take_value("--issue", args, &mut i)?,
            "--repo" => repo = Some(take_value("--repo", args, &mut i)?),
            unknown => return Err(format!("Unknown option for reevaluate: {unknown}")),
        }
    }

    require_positive_number("--issue", &issue)?;
    Ok(ReevaluateOptions { issue, repo })
}

fn parse_create(args: &[String]) -> Result<CreateOptions, String> {
    let mut title = String::new();
    let mut context = String::new();
    let mut problem = String::new();
    let mut acceptances: Vec<String> = Vec::new();
    let mut parent = "none".to_string();
    let mut labels: Vec<String> = Vec::new();
    let mut repo: Option<String> = None;
    let mut add_default_issue_label = true;
    let mut dry_run = false;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--title" => title = take_value("--title", args, &mut i)?,
            "--context" => context = take_value("--context", args, &mut i)?,
            "--problem" => problem = take_value("--problem", args, &mut i)?,
            "--acceptance" => acceptances.push(take_value("--acceptance", args, &mut i)?),
            "--parent" => parent = take_value("--parent", args, &mut i)?,
            "--label" => labels.push(take_value("--label", args, &mut i)?),
            "--repo" => repo = Some(take_value("--repo", args, &mut i)?),
            "--no-default-issue-label" => {
                add_default_issue_label = false;
                i += 1;
            }
            "--dry-run" => {
                dry_run = true;
                i += 1;
            }
            "-h" | "--help" => return Err("Use: va issue help".to_string()),
            unknown => return Err(format!("Unknown option for create: {unknown}")),
        }
    }

    if title.is_empty() || context.is_empty() || problem.is_empty() || acceptances.is_empty() {
        return Err(
            "create requires: --title --context --problem and at least one --acceptance"
                .to_string(),
        );
    }
    if add_default_issue_label
        && !labels
            .iter()
            .any(|label| label.eq_ignore_ascii_case("issue"))
    {
        labels.push("issue".to_string());
    }

    Ok(CreateOptions {
        title,
        context,
        problem,
        acceptances,
        parent,
        labels,
        repo,
        dry_run,
    })
}

fn parse_read(args: &[String]) -> Result<ReadOptions, String> {
    let mut issue: Option<String> = None;
    let mut repo: Option<String> = None;
    let mut json: Option<String> = None;
    let mut jq: Option<String> = None;
    let mut template: Option<String> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--issue" => issue = Some(take_value("--issue", args, &mut i)?),
            "--repo" => repo = Some(take_value("--repo", args, &mut i)?),
            "--json" => json = Some(take_value("--json", args, &mut i)?),
            "--jq" => jq = Some(take_value("--jq", args, &mut i)?),
            "--template" => template = Some(take_value("--template", args, &mut i)?),
            "-h" | "--help" => {
                return Ok(ReadOptions {
                    issue,
                    repo,
                    json,
                    jq,
                    template,
                });
            }
            unknown => return Err(format!("Unknown option for read: {unknown}")),
        }
    }

    if let Some(value) = &issue {
        require_positive_number("--issue", value)?;
    }

    Ok(ReadOptions {
        issue,
        repo,
        json,
        jq,
        template,
    })
}

fn parse_update(args: &[String]) -> Result<UpdateOptions, String> {
    let mut issue = String::new();
    let mut repo: Option<String> = None;
    let mut edit_args: Vec<(String, String)> = Vec::new();
    let allowed = [
        "--title",
        "--body",
        "--add-label",
        "--remove-label",
        "--add-assignee",
        "--remove-assignee",
    ];

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--issue" => issue = take_value("--issue", args, &mut i)?,
            "--repo" => repo = Some(take_value("--repo", args, &mut i)?),
            flag if allowed.contains(&flag) => {
                let value = take_value(flag, args, &mut i)?;
                edit_args.push((flag.to_string(), value));
            }
            unknown => return Err(format!("Unknown option for update: {unknown}")),
        }
    }

    require_positive_number("--issue", &issue)?;
    if edit_args.is_empty() {
        return Err("update requires at least one edit option".to_string());
    }

    Ok(UpdateOptions {
        issue,
        repo,
        edit_args,
    })
}

fn parse_close(args: &[String]) -> Result<CloseOptions, String> {
    let mut issue = String::new();
    let mut repo: Option<String> = None;
    let mut reason = "completed".to_string();

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--issue" => issue = take_value("--issue", args, &mut i)?,
            "--repo" => repo = Some(take_value("--repo", args, &mut i)?),
            "--reason" => reason = take_value("--reason", args, &mut i)?,
            unknown => return Err(format!("Unknown option for close: {unknown}")),
        }
    }

    require_positive_number("--issue", &issue)?;
    if reason != "completed" && reason != "not_planned" {
        return Err("--reason must be 'completed' or 'not_planned'".to_string());
    }

    Ok(CloseOptions {
        issue,
        repo,
        reason,
    })
}

fn parse_target(command: &str, args: &[String]) -> Result<IssueTarget, String> {
    let mut issue = String::new();
    let mut repo: Option<String> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--issue" => issue = take_value("--issue", args, &mut i)?,
            "--repo" => repo = Some(take_value("--repo", args, &mut i)?),
            unknown => return Err(format!("Unknown option for {command}: {unknown}")),
        }
    }

    require_positive_number("--issue", &issue)?;
    Ok(IssueTarget { issue, repo })
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

fn require_positive_number(flag: &str, value: &str) -> Result<(), String> {
    if !value.is_empty() && value.chars().all(|c| c.is_ascii_digit()) && value != "0" {
        return Ok(());
    }
    Err(format!("{flag} requires a positive numeric value"))
}
