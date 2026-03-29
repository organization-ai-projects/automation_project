//! tools/versioning_automation/src/pr/tests/sort_bullets.rs
use std::fs;

use crate::pr::{commands::PrSortBulletsOptions, sort_bullets::run_sort_bullets};

#[test]
fn sort_bullets_command_runs_with_existing_file() {
    let file_path = "/tmp/va_pr_sort_bullets.txt";
    fs::write(file_path, "- b (#20)\n- a (#3)\n").expect("write input");
    let opts = PrSortBulletsOptions {
        input_file: file_path.to_string(),
    };
    let code = run_sort_bullets(opts);
    assert_eq!(code, 0);
    fs::remove_file(file_path).expect("remove input");
}

#[test]
fn sort_bullets_command_fails_with_missing_file() {
    let opts = PrSortBulletsOptions {
        input_file: "/tmp/va_pr_sort_bullets_missing.txt".to_string(),
    };
    let code = run_sort_bullets(opts);
    assert_eq!(code, 2);
}
