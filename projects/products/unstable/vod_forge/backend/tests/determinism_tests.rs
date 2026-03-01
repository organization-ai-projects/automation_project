use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

fn backend_binary() -> String {
    let mut p = std::env::current_exe().unwrap();
    p.pop();
    let candidate = p.join("vod_forge_backend");
    if candidate.exists() {
        return candidate.to_str().unwrap().to_string();
    }
    let candidate2 = p.join("../vod_forge_backend");
    if candidate2.exists() {
        return candidate2.canonicalize().unwrap().to_str().unwrap().to_string();
    }
    "vod_forge_backend".to_string()
}

fn send_recv(binary: &str, requests: &[&str]) -> Vec<String> {
    let mut child = Command::new(binary)
        .arg("serve")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to spawn backend");

    let mut stdin = child.stdin.take().unwrap();
    for req in requests {
        writeln!(stdin, "{}", req).unwrap();
    }
    drop(stdin);

    let stdout = child.stdout.take().unwrap();
    let reader = BufReader::new(stdout);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    child.wait().unwrap();
    lines
}

#[test]
fn test_manifest_encoding_determinism() {
    let dir = tempfile::tempdir().unwrap();
    let f1 = dir.path().join("video_001.bin");
    let f2 = dir.path().join("video_002.bin");
    std::fs::write(&f1, b"DUMMY_VIDEO_DATA_001").unwrap();
    std::fs::write(&f2, b"DUMMY_VIDEO_DATA_002").unwrap();
    let bundle1 = dir.path().join("bundle1.vbun");
    let bundle2 = dir.path().join("bundle2.vbun");

    let binary = backend_binary();

    let req1 = format!(
        r#"{{"id":1,"payload":{{"type":"PackageCreate","input_files":["{}","{}"],"out_bundle":"{}"}}}}"#,
        f1.display(), f2.display(), bundle1.display()
    );
    let req2 = format!(
        r#"{{"id":2,"payload":{{"type":"PackageCreate","input_files":["{}","{}"],"out_bundle":"{}"}}}}"#,
        f1.display(), f2.display(), bundle2.display()
    );

    let responses1 = send_recv(&binary, &[&req1]);
    let responses2 = send_recv(&binary, &[&req2]);

    assert_eq!(responses1.len(), 1, "expected 1 response, got: {:?}", responses1);
    assert_eq!(responses2.len(), 1, "expected 1 response, got: {:?}", responses2);

    // Extract bundle_hash from each response and compare
    let extract_hash = |s: &str| -> String {
        s.split("bundle_hash")
            .nth(1)
            .unwrap_or("")
            .chars()
            .skip_while(|c| !c.is_alphanumeric())
            .take_while(|c| c.is_alphanumeric())
            .collect()
    };

    let hash1 = extract_hash(&responses1[0]);
    let hash2 = extract_hash(&responses2[0]);
    assert!(!hash1.is_empty(), "no bundle_hash in: {}", responses1[0]);
    assert!(!hash2.is_empty(), "no bundle_hash in: {}", responses2[0]);
    // Same input files -> same bundle hash
    assert_eq!(hash1, hash2, "bundle hashes differ for same input");
}

#[test]
fn test_package_round_trip() {
    let dir = tempfile::tempdir().unwrap();
    let f1 = dir.path().join("video_001.bin");
    let f2 = dir.path().join("video_002.bin");
    std::fs::write(&f1, b"DUMMY_VIDEO_DATA_001").unwrap();
    std::fs::write(&f2, b"DUMMY_VIDEO_DATA_002").unwrap();
    let bundle = dir.path().join("bundle.vbun");

    let binary = backend_binary();

    let create_req = format!(
        r#"{{"id":1,"payload":{{"type":"PackageCreate","input_files":["{}","{}"],"out_bundle":"{}"}}}}"#,
        f1.display(), f2.display(), bundle.display()
    );
    let verify_req = format!(
        r#"{{"id":2,"payload":{{"type":"PackageVerify","bundle":"{}"}}}}"#,
        bundle.display()
    );

    let create_resp = send_recv(&binary, &[&create_req]);
    assert!(create_resp[0].contains("PackageResult"), "create failed: {}", create_resp[0]);

    let verify_resp = send_recv(&binary, &[&verify_req]);
    assert!(verify_resp[0].contains("PackageResult"), "verify failed: {}", verify_resp[0]);

    let create_hash: String = create_resp[0]
        .split("bundle_hash")
        .nth(1)
        .unwrap()
        .chars()
        .skip_while(|c| !c.is_alphanumeric())
        .take_while(|c| c.is_alphanumeric())
        .collect();
    let verify_hash: String = verify_resp[0]
        .split("bundle_hash")
        .nth(1)
        .unwrap()
        .chars()
        .skip_while(|c| !c.is_alphanumeric())
        .take_while(|c| c.is_alphanumeric())
        .collect();
    assert_eq!(create_hash, verify_hash);
}

#[test]
fn test_playback_session_initial_state() {
    let binary = backend_binary();

    let start_req = r#"{"id":1,"payload":{"type":"PlaybackStart","profile":"alice","episode_id":"ep001"}}"#;

    let resp1 = send_recv(&binary, &[start_req]);
    let resp2 = send_recv(&binary, &[start_req]);

    assert!(resp1[0].contains("\"tick\":0"), "unexpected: {}", resp1[0]);
    assert!(resp2[0].contains("\"tick\":0"), "unexpected: {}", resp2[0]);
}

#[test]
fn test_analytics_report_determinism() {
    let binary = backend_binary();

    let analytics_req = r#"{"id":10,"payload":{"type":"AnalyticsReport","profile":"alice"}}"#;
    let resp1 = send_recv(&binary, &[analytics_req]);
    let resp2 = send_recv(&binary, &[analytics_req]);

    // Both responses must contain the same fields (field order may vary)
    assert!(resp1[0].contains("AnalyticsReport"), "unexpected: {}", resp1[0]);
    assert!(resp2[0].contains("AnalyticsReport"), "unexpected: {}", resp2[0]);
    assert!(resp1[0].contains("\"episodes_watched\":0"), "unexpected: {}", resp1[0]);
    assert!(resp2[0].contains("\"episodes_watched\":0"), "unexpected: {}", resp2[0]);
    assert!(resp1[0].contains("\"total_watch_ticks\":0"), "unexpected: {}", resp1[0]);
    assert!(resp2[0].contains("\"total_watch_ticks\":0"), "unexpected: {}", resp2[0]);
}

#[test]
fn test_catalog_add_and_list() {
    let binary = backend_binary();

    let add_title = r#"{"id":1,"payload":{"type":"CatalogAddTitle","title_id":"tt001","name":"Space Odyssey","year":2020}}"#;
    let add_ep = r#"{"id":2,"payload":{"type":"CatalogAddEpisode","title_id":"tt001","season":1,"episode":1,"name":"Pilot","duration_secs":2700}}"#;
    let list = r#"{"id":3,"payload":{"type":"CatalogList"}}"#;

    let responses = send_recv(&binary, &[add_title, add_ep, list]);
    assert_eq!(responses.len(), 3, "expected 3 responses, got: {:?}", responses);
    assert!(responses[0].contains("\"Ok\"") || responses[0].contains("Ok"), "add title failed: {}", responses[0]);
    assert!(responses[2].contains("Space Odyssey"), "list missing title: {}", responses[2]);
}
