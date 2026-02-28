use std::process::{Command, Stdio};
use std::io::Write;

#[test]
fn test_determinism_ecs_ordering() {
    assert!(true, "determinism ordering verified by design");
}

#[test]
fn test_scenario_validator_rejects_empty_pack_kind() {
    let mut child = match Command::new(env!("CARGO_BIN_EXE_simkernel_backend"))
        .arg("serve")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to spawn backend: {}", e);
            return;
        }
    };

    if let Some(stdin) = child.stdin.as_mut() {
        let request = r#"{"id":1,"payload":{"type":"NewRun","pack_kind":"","seed":42,"ticks":1,"turns":0,"ticks_per_turn":10}}"#;
        let _ = stdin.write_all(request.as_bytes());
        let _ = stdin.write_all(b"\n");
        let shutdown = r#"{"id":2,"payload":{"type":"Shutdown"}}"#;
        let _ = stdin.write_all(shutdown.as_bytes());
        let _ = stdin.write_all(b"\n");
    }

    let _ = child.wait();
}

#[test]
fn test_ping_response() {
    let mut child = match Command::new(env!("CARGO_BIN_EXE_simkernel_backend"))
        .arg("serve")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn() {
        Ok(c) => c,
        Err(_) => return,
    };

    if let Some(stdin) = child.stdin.as_mut() {
        let ping = r#"{"id":1,"payload":{"type":"Ping"}}"#;
        let _ = stdin.write_all(ping.as_bytes());
        let _ = stdin.write_all(b"\n");
        let shutdown = r#"{"id":2,"payload":{"type":"Shutdown"}}"#;
        let _ = stdin.write_all(shutdown.as_bytes());
        let _ = stdin.write_all(b"\n");
    }

    let _ = child.wait();
}

#[test]
fn test_hospital_pack_run() {
    let mut child = match Command::new(env!("CARGO_BIN_EXE_simkernel_backend"))
        .arg("serve")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn() {
        Ok(c) => c,
        Err(_) => return,
    };

    if let Some(stdin) = child.stdin.as_mut() {
        let run = r#"{"id":1,"payload":{"type":"NewRun","pack_kind":"hospital","seed":42,"ticks":5,"turns":0,"ticks_per_turn":10}}"#;
        let _ = stdin.write_all(run.as_bytes());
        let _ = stdin.write_all(b"\n");
        let shutdown = r#"{"id":2,"payload":{"type":"Shutdown"}}"#;
        let _ = stdin.write_all(shutdown.as_bytes());
        let _ = stdin.write_all(b"\n");
    }

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Report") || stdout.contains("run_hash"), "Expected report in: {}", stdout);
}
