use crate::validation_invocation_parser::parse_validation_pending_invocations;

#[test]
fn parses_multiple_validation_invocations_with_scoped_args_and_env() {
    let raw_args = vec![
        "--validation-bin".to_string(),
        "validator-a".to_string(),
        "--validation-arg".to_string(),
        "--strict".to_string(),
        "--validation-env".to_string(),
        "MODE=fast".to_string(),
        "--validation-bin".to_string(),
        "validator-b".to_string(),
        "--validation-arg".to_string(),
        "--json".to_string(),
    ];

    let parsed = parse_validation_pending_invocations(&raw_args).expect("parsing should succeed");
    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0].command, "validator-a");
    assert_eq!(parsed[0].args, vec!["--strict".to_string()]);
    assert_eq!(
        parsed[0].env,
        vec![("MODE".to_string(), "fast".to_string())]
    );
    assert_eq!(parsed[1].command, "validator-b");
    assert_eq!(parsed[1].args, vec!["--json".to_string()]);
    assert!(parsed[1].env.is_empty());
}

#[test]
fn rejects_validation_arg_without_preceding_binary() {
    let raw_args = vec!["--validation-arg".to_string(), "--strict".to_string()];

    let err = parse_validation_pending_invocations(&raw_args).expect_err("expected parsing error");
    assert_eq!(
        err,
        "--validation-arg requires a preceding --validation-bin"
    );
}

#[test]
fn rejects_invalid_validation_env_pair() {
    let raw_args = vec![
        "--validation-bin".to_string(),
        "validator-a".to_string(),
        "--validation-env".to_string(),
        "BROKEN".to_string(),
    ];

    let err = parse_validation_pending_invocations(&raw_args).expect_err("expected parsing error");
    assert_eq!(err, "Invalid env pair 'BROKEN', expected KEY=VALUE");
}
