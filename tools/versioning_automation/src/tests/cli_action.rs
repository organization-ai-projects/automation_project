use crate::cli_action::{CliAction, parse};

#[test]
fn parse_routes_automation_subcommand() {
    let args = vec![
        "va".to_string(),
        "automation".to_string(),
        "labels-sync".to_string(),
    ];
    let action = parse(&args).expect("parse should succeed");
    match action {
        CliAction::RunAutomation(passthrough) => {
            assert_eq!(passthrough, vec!["labels-sync".to_string()]);
        }
        _ => panic!("expected RunAutomation"),
    }
}

#[test]
fn parse_routes_git_subcommand() {
    let args = vec![
        "va".to_string(),
        "git".to_string(),
        "push-branch".to_string(),
    ];
    let action = parse(&args).expect("parse should succeed");
    match action {
        CliAction::RunGit(passthrough) => {
            assert_eq!(passthrough, vec!["push-branch".to_string()]);
        }
        _ => panic!("expected RunGit"),
    }
}

#[test]
fn parse_rejects_unknown_subcommand() {
    let args = vec!["va".to_string(), "unknown".to_string()];
    match parse(&args) {
        Ok(_) => panic!("parse should fail"),
        Err(err) => assert!(err.contains("Unknown subcommand")),
    }
}
