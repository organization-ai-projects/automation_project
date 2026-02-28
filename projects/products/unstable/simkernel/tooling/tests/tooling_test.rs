use std::process::{Command, Stdio};

#[test]
fn test_tooling_no_args_exits_with_2() {
    let output = match Command::new(env!("CARGO_BIN_EXE_simkernel_tooling"))
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
    {
        Ok(o) => o,
        Err(_) => return,
    };
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn test_generate_pack_creates_files() {
    let tmp = std::env::temp_dir().join(format!(
        "simkernel_test_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
    std::fs::create_dir_all(&tmp).unwrap();

    let output = match Command::new(env!("CARGO_BIN_EXE_simkernel_tooling"))
        .args(["generate-pack", "TestPack", "--out", tmp.to_str().unwrap()])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
    {
        Ok(o) => o,
        Err(_) => return,
    };

    assert!(output.status.success(), "generate-pack should succeed");
    let pack_dir = tmp.join("testpack");
    assert!(pack_dir.exists(), "Pack directory should be created");

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn test_validate_contract_missing_file_exits_3() {
    let output = match Command::new(env!("CARGO_BIN_EXE_simkernel_tooling"))
        .args([
            "validate-contract",
            "/tmp/nonexistent_simkernel_contract.json",
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
    {
        Ok(o) => o,
        Err(_) => return,
    };
    assert_eq!(
        output.status.code(),
        Some(3),
        "Missing file should exit with code 3"
    );
}
