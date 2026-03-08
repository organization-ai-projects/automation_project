use crate::cli_args::get_arg;

#[test]
fn get_arg_returns_value_for_matching_flag() {
    let args = vec![
        "diplo_sim".to_string(),
        "run".to_string(),
        "--seed".to_string(),
        "123".to_string(),
    ];

    assert_eq!(get_arg(&args, "--seed"), Some("123".to_string()));
    assert_eq!(get_arg(&args, "--missing"), None);
}
