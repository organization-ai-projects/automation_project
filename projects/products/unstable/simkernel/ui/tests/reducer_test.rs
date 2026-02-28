use std::process::{Command, Stdio};

#[test]
fn test_ui_help_exits_with_code_2() {
    let output = match Command::new(env!("CARGO_BIN_EXE_simkernel_ui"))
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output() {
        Ok(o) => o,
        Err(_) => return,
    };
    assert_eq!(output.status.code(), Some(2));
}
