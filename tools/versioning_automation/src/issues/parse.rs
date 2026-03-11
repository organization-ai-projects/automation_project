//! tools/versioning_automation/src/issues/parse.rs
use crate::issues::commands::{
    AssigneeLoginsOptions, AutoLinkOptions, CloseOptions, CreateOptions, DoneStatusMode,
    DoneStatusOptions, FetchNonComplianceReasonOptions, HasLabelOptions, IssueAction,
    IssueFieldName, IssueFieldOptions, IssueTarget, LabelExistsOptions, ListByLabelOptions,
    NeutralizeOptions, NonComplianceReasonOptions, OpenNumbersOptions, ReadOptions,
    ReevaluateOptions, RequiredFieldsValidateOptions, RequiredFieldsValidationMode, StateOptions,
    SubissueRefsOptions, SyncProjectStatusOptions, TasklistRefsOptions, UpdateOptions,
    UpsertMarkerCommentOptions,
};

pub(crate) fn parse(args: &[String]) -> Result<IssueAction, String> {
    if args.is_empty() {
        return Ok(IssueAction::Help);
    }
    match args[0].as_str() {
        "help" | "--help" | "-h" => Ok(IssueAction::Help),
        "create" => parse_create(&args[1..]).map(IssueAction::Create),
        "done-status" => parse_done_status(&args[1..]).map(IssueAction::DoneStatus),
        "reopen-on-dev" => parse_reopen_on_dev(&args[1..]).map(IssueAction::ReopenOnDev),
        "read" => parse_read(&args[1..]).map(IssueAction::Read),
        "update" => parse_update(&args[1..]).map(IssueAction::Update),
        "repo-name" => parse_repo_name(&args[1..]),
        "close" => parse_close(&args[1..]).map(IssueAction::Close),
        "reopen" => parse_target("reopen", &args[1..]).map(IssueAction::Reopen),
        "delete" => parse_target("delete", &args[1..]).map(IssueAction::Delete),
        "reevaluate" => parse_reevaluate(&args[1..]).map(IssueAction::Reevaluate),
        "neutralize" => parse_neutralize(&args[1..]).map(IssueAction::Neutralize),
        "auto-link" => parse_auto_link(&args[1..]).map(IssueAction::AutoLink),
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
        "sync-project-status" => {
            parse_sync_project_status(&args[1..]).map(IssueAction::SyncProjectStatus)
        }
        "tasklist-refs" => parse_tasklist_refs(&args[1..]).map(IssueAction::TasklistRefs),
        "subissue-refs" => parse_subissue_refs(&args[1..]).map(IssueAction::SubissueRefs),
        "upsert-marker-comment" => {
            parse_upsert_marker_comment(&args[1..]).map(IssueAction::UpsertMarkerComment)
        }
        "open-numbers" => parse_open_numbers(&args[1..]).map(IssueAction::OpenNumbers),
        "assignee-logins" => parse_assignee_logins(&args[1..]).map(IssueAction::AssigneeLogins),
        "state" => parse_state(&args[1..]).map(IssueAction::State),
        "has-label" => parse_has_label(&args[1..]).map(IssueAction::HasLabel),
        "list-by-label" => parse_list_by_label(&args[1..]).map(IssueAction::ListByLabel),
        "field" => parse_field(&args[1..]).map(IssueAction::Field),
        unknown => Err(format!("Unknown issue subcommand: {unknown}")),
    }
}

fn parse_field(args: &[String]) -> Result<IssueFieldOptions, String> {
    let mut issue = String::new();
    let mut repo: Option<String> = None;
    let mut name = String::new();

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--issue" => issue = take_value("--issue", args, &mut i)?,
            "--repo" => repo = Some(take_value("--repo", args, &mut i)?),
            "--name" => name = take_value("--name", args, &mut i)?,
            unknown => return Err(format!("Unknown option for field: {unknown}")),
        }
    }

    require_positive_number("--issue", &issue)?;
    ensure_non_empty_or("field requires: --issue and --name", &[&name])?;

    let name = match name.as_str() {
        "title" => IssueFieldName::Title,
        "body" => IssueFieldName::Body,
        "labels-raw" => IssueFieldName::LabelsRaw,
        _ => return Err("--name must be one of: title | body | labels-raw".to_string()),
    };

    Ok(IssueFieldOptions { issue, repo, name })
}

fn parse_done_status(args: &[String]) -> Result<DoneStatusOptions, String> {
    let mut mode: Option<DoneStatusMode> = None;
    let mut pr: Option<String> = None;
    let mut issue: Option<String> = None;
    let mut label = "done-in-dev".to_string();
    let mut repo: Option<String> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--on-dev-merge" => {
                mode = Some(DoneStatusMode::OnDevMerge);
                i += 1;
            }
            "--on-issue-closed" => {
                mode = Some(DoneStatusMode::OnIssueClosed);
                i += 1;
            }
            "--pr" => pr = Some(take_value("--pr", args, &mut i)?),
            "--issue" => issue = Some(take_value("--issue", args, &mut i)?),
            "--label" => label = take_value("--label", args, &mut i)?,
            "--repo" => repo = Some(take_value("--repo", args, &mut i)?),
            unknown => return Err(format!("Unknown option for done-status: {unknown}")),
        }
    }

    let Some(mode) = mode else {
        return Err(
            "done-status requires one mode: --on-dev-merge or --on-issue-closed".to_string(),
        );
    };

    match mode {
        DoneStatusMode::OnDevMerge => {
            let pr_value = pr
                .as_deref()
                .ok_or_else(|| "done-status --on-dev-merge requires: --pr".to_string())?;
            require_positive_number("--pr", pr_value)?;
        }
        DoneStatusMode::OnIssueClosed => {
            let issue_value = issue
                .as_deref()
                .ok_or_else(|| "done-status --on-issue-closed requires: --issue".to_string())?;
            require_positive_number("--issue", issue_value)?;
        }
    }

    Ok(DoneStatusOptions {
        mode,
        pr,
        issue,
        label,
        repo,
    })
}

fn parse_reopen_on_dev(
    args: &[String],
) -> Result<crate::issues::commands::ReopenOnDevOptions, String> {
    let mut pr = String::new();
    let mut label = "done-in-dev".to_string();
    let mut repo: Option<String> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--pr" => pr = take_value("--pr", args, &mut i)?,
            "--label" => label = take_value("--label", args, &mut i)?,
            "--repo" => repo = Some(take_value("--repo", args, &mut i)?),
            unknown => return Err(format!("Unknown option for reopen-on-dev: {unknown}")),
        }
    }

    require_positive_number("--pr", &pr)?;

    Ok(crate::issues::commands::ReopenOnDevOptions { pr, label, repo })
}

fn parse_neutralize(args: &[String]) -> Result<NeutralizeOptions, String> {
    let mut pr = String::new();
    let mut repo: Option<String> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--pr" => pr = take_value("--pr", args, &mut i)?,
            "--repo" => repo = Some(take_value("--repo", args, &mut i)?),
            unknown => return Err(format!("Unknown option for neutralize: {unknown}")),
        }
    }
    if pr.is_empty() {
        return Err("--pr is required".to_string());
    }
    require_positive_number("--pr", &pr)?;
    Ok(NeutralizeOptions { pr, repo })
}

fn parse_auto_link(args: &[String]) -> Result<AutoLinkOptions, String> {
    let mut issue = String::new();
    let mut repo: Option<String> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--issue" => issue = take_value("--issue", args, &mut i)?,
            "--repo" => repo = Some(take_value("--repo", args, &mut i)?),
            unknown => return Err(format!("Unknown option for auto-link: {unknown}")),
        }
    }
    if issue.is_empty() {
        return Err("--issue is required".to_string());
    }
    require_positive_number("--issue", &issue)?;
    Ok(AutoLinkOptions { issue, repo })
}

fn parse_open_numbers(args: &[String]) -> Result<OpenNumbersOptions, String> {
    let mut repo: Option<String> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--repo" => repo = Some(take_value("--repo", args, &mut i)?),
            unknown => return Err(format!("Unknown option for open-numbers: {unknown}")),
        }
    }
    Ok(OpenNumbersOptions { repo })
}

fn parse_assignee_logins(args: &[String]) -> Result<AssigneeLoginsOptions, String> {
    let (issue, repo) = parse_issue_and_optional_repo(args, "assignee-logins")?;
    Ok(AssigneeLoginsOptions { issue, repo })
}

fn parse_state(args: &[String]) -> Result<StateOptions, String> {
    let (issue, repo) = parse_issue_and_optional_repo(args, "state")?;
    Ok(StateOptions { issue, repo })
}

fn parse_has_label(args: &[String]) -> Result<HasLabelOptions, String> {
    let mut issue = String::new();
    let mut repo: Option<String> = None;
    let mut label = String::new();

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--issue" => issue = take_value("--issue", args, &mut i)?,
            "--repo" => repo = Some(take_value("--repo", args, &mut i)?),
            "--label" => label = take_value("--label", args, &mut i)?,
            unknown => return Err(format!("Unknown option for has-label: {unknown}")),
        }
    }

    require_positive_number("--issue", &issue)?;
    ensure_non_empty_or("has-label requires: --issue and --label", &[&label])?;

    Ok(HasLabelOptions { issue, label, repo })
}

fn parse_list_by_label(args: &[String]) -> Result<ListByLabelOptions, String> {
    let mut label = String::new();
    let mut repo: Option<String> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--label" => label = take_value("--label", args, &mut i)?,
            "--repo" => repo = Some(take_value("--repo", args, &mut i)?),
            unknown => return Err(format!("Unknown option for list-by-label: {unknown}")),
        }
    }

    ensure_non_empty_or("list-by-label requires: --label", &[&label])?;
    Ok(ListByLabelOptions { label, repo })
}

fn parse_upsert_marker_comment(args: &[String]) -> Result<UpsertMarkerCommentOptions, String> {
    let mut repo = String::new();
    let mut issue = String::new();
    let mut marker = String::new();
    let mut body = String::new();
    let mut announce = false;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--repo" => repo = take_value("--repo", args, &mut i)?,
            "--issue" => issue = take_value("--issue", args, &mut i)?,
            "--marker" => marker = take_value("--marker", args, &mut i)?,
            "--body" => body = take_value("--body", args, &mut i)?,
            "--announce" => {
                announce = parse_bool_value("--announce", &take_value("--announce", args, &mut i)?)?
            }
            unknown => {
                return Err(format!(
                    "Unknown option for upsert-marker-comment: {unknown}"
                ));
            }
        }
    }

    require_positive_number("--issue", &issue)?;
    ensure_non_empty_or(
        "upsert-marker-comment requires: --repo, --issue, --marker and --body",
        &[&repo, &marker, &body],
    )?;

    Ok(UpsertMarkerCommentOptions {
        repo,
        issue,
        marker,
        body,
        announce,
    })
}

fn parse_subissue_refs(args: &[String]) -> Result<SubissueRefsOptions, String> {
    let mut owner = String::new();
    let mut repo = String::new();
    let mut issue = String::new();

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--owner" => owner = take_value("--owner", args, &mut i)?,
            "--repo" => repo = take_value("--repo", args, &mut i)?,
            "--issue" => issue = take_value("--issue", args, &mut i)?,
            unknown => return Err(format!("Unknown option for subissue-refs: {unknown}")),
        }
    }

    require_positive_number("--issue", &issue)?;
    ensure_non_empty_or(
        "subissue-refs requires: --owner, --repo and --issue",
        &[&owner, &repo],
    )?;

    Ok(SubissueRefsOptions { owner, repo, issue })
}

fn parse_tasklist_refs(args: &[String]) -> Result<TasklistRefsOptions, String> {
    let mut body = String::new();

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--body" => body = take_value("--body", args, &mut i)?,
            unknown => return Err(format!("Unknown option for tasklist-refs: {unknown}")),
        }
    }

    ensure_non_empty_or("tasklist-refs requires: --body", &[&body])?;

    Ok(TasklistRefsOptions { body })
}

fn parse_repo_name(args: &[String]) -> Result<IssueAction, String> {
    if args.is_empty() {
        Ok(IssueAction::RepoName)
    } else {
        Err("repo-name does not accept additional options".to_string())
    }
}

fn parse_sync_project_status(args: &[String]) -> Result<SyncProjectStatusOptions, String> {
    let mut repo = String::new();
    let mut issue = String::new();
    let mut status = String::new();

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--repo" => repo = take_value("--repo", args, &mut i)?,
            "--issue" => issue = take_value("--issue", args, &mut i)?,
            "--status" => status = take_value("--status", args, &mut i)?,
            unknown => return Err(format!("Unknown option for sync-project-status: {unknown}")),
        }
    }

    require_positive_number("--issue", &issue)?;
    ensure_non_empty_or(
        "sync-project-status requires: --repo, --issue and --status",
        &[&repo, &status],
    )?;

    Ok(SyncProjectStatusOptions {
        repo,
        issue,
        status,
    })
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

    ensure_non_empty_or(
        "label-exists requires: --repo and --label",
        &[&repo, &label],
    )?;

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
    let (issue, repo) = parse_issue_and_optional_repo(args, "fetch-non-compliance-reason")?;
    Ok(FetchNonComplianceReasonOptions { issue, repo })
}

fn parse_reevaluate(args: &[String]) -> Result<ReevaluateOptions, String> {
    let (issue, repo) = parse_issue_and_optional_repo(args, "reevaluate")?;
    Ok(ReevaluateOptions { issue, repo })
}

fn parse_create(args: &[String]) -> Result<CreateOptions, String> {
    let mut title = String::new();
    let mut context = String::new();
    let mut problem = String::new();
    let mut acceptances: Vec<String> = Vec::new();
    let mut parent = "none".to_string();
    let mut labels: Vec<String> = Vec::new();
    let mut assignees: Vec<String> = Vec::new();
    let mut related_issues: Vec<String> = Vec::new();
    let mut related_prs: Vec<String> = Vec::new();
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
            "--assignee" => assignees.push(take_value("--assignee", args, &mut i)?),
            "--related-issue" => related_issues.push(take_value("--related-issue", args, &mut i)?),
            "--related-pr" => related_prs.push(take_value("--related-pr", args, &mut i)?),
            "--template" => {
                let _ = take_value("--template", args, &mut i)?;
            }
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
        assignees,
        related_issues,
        related_prs,
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
    let mut comment: Option<String> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--issue" => issue = take_value("--issue", args, &mut i)?,
            "--repo" => repo = Some(take_value("--repo", args, &mut i)?),
            "--reason" => reason = take_value("--reason", args, &mut i)?,
            "--comment" => comment = Some(take_value("--comment", args, &mut i)?),
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
        comment,
    })
}

fn parse_target(command: &str, args: &[String]) -> Result<IssueTarget, String> {
    let (issue, repo) = parse_issue_and_optional_repo(args, command)?;
    Ok(IssueTarget { issue, repo })
}

fn parse_issue_and_optional_repo(
    args: &[String],
    command: &str,
) -> Result<(String, Option<String>), String> {
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
    Ok((issue, repo))
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

fn parse_bool_value(flag: &str, raw: &str) -> Result<bool, String> {
    match raw {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(format!("{flag} expects true|false")),
    }
}

fn ensure_non_empty_or(error_message: &str, values: &[&str]) -> Result<(), String> {
    if values.iter().all(|value| !value.trim().is_empty()) {
        return Ok(());
    }
    Err(error_message.to_string())
}
