use crate::cli_action::CliAction;

#[test]
fn parse_without_subcommand_returns_help() {
    let args = vec!["va".to_string()];
    let action = CliAction::parse(&args).expect("parse should succeed");
    match action {
        CliAction::ShowHelp(help) => assert!(help.contains("Usage:")),
        _ => panic!("expected help action"),
    }
}

#[test]
fn parse_pr_with_passthrough_args() {
    let args = vec![
        "va".to_string(),
        "pr".to_string(),
        "--dry-run".to_string(),
        "--yes".to_string(),
    ];
    let action = CliAction::parse(&args).expect("parse should succeed");
    match action {
        CliAction::RunPr(passthrough) => {
            assert_eq!(
                passthrough,
                vec!["--dry-run".to_string(), "--yes".to_string()]
            );
        }
        _ => panic!("expected pr action"),
    }
}

#[test]
fn parse_issue_with_passthrough_args() {
    let args = vec![
        "va".to_string(),
        "issue".to_string(),
        "read".to_string(),
        "--issue".to_string(),
        "42".to_string(),
    ];
    let action = CliAction::parse(&args).expect("parse should succeed");
    match action {
        CliAction::RunIssue(passthrough) => {
            assert_eq!(
                passthrough,
                vec!["read".to_string(), "--issue".to_string(), "42".to_string()]
            );
        }
        _ => panic!("expected issue action"),
    }
}

#[test]
fn parse_unknown_subcommand_fails() {
    let args = vec!["va".to_string(), "unknown".to_string()];
    match CliAction::parse(&args) {
        Ok(_) => panic!("parse should fail"),
        Err(err) => assert!(err.contains("Unknown subcommand")),
    }
}
