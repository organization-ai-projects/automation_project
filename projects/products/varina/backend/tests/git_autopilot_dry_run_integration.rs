use backend::router::{ACTION_GIT_AUTOPILOT_PREVIEW, handle_command};
use protocol::json::Json;
use protocol::{Command, CommandType, Metadata, Payload, pjson};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as ProcCommand;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

// Router expects EXACT payload types:
const PREVIEW_PAYLOAD_TYPE_V1: &str = "git_autopilot/preview/v1";
// Router response payload_type for preview:
const RESPONSE_TYPE_PREVIEW: &str = "preview_response";

// Global lock because std::env::set_current_dir is process-global.
static CWD_LOCK: Mutex<()> = Mutex::new(());

fn unique_tmp_dir(prefix: &str) -> PathBuf {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    env::temp_dir().join(format!("{prefix}-{ts}"))
}

fn run_ok(cmd: &mut ProcCommand) {
    let out = cmd.output().expect("failed to spawn process");
    if !out.status.success() {
        panic!(
            "command failed: {:?}\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
            cmd,
            out.status,
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
    }
}

fn git(repo: &Path, args: &[&str]) -> String {
    let out = ProcCommand::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("failed to run git");
    if !out.status.success() {
        panic!(
            "git {:?} failed\nstdout:\n{}\nstderr:\n{}",
            args,
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
    }
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn init_temp_repo_with_change() -> PathBuf {
    let dir = unique_tmp_dir("varina-autopilot-dryrun");
    fs::create_dir_all(&dir).expect("failed to create temp dir");

    run_ok(ProcCommand::new("git").arg("init").current_dir(&dir));

    // Local config for CI
    run_ok(
        ProcCommand::new("git")
            .args(["config", "user.email", "ci@example.com"])
            .current_dir(&dir),
    );
    run_ok(
        ProcCommand::new("git")
            .args(["config", "user.name", "CI"])
            .current_dir(&dir),
    );

    // baseline commit
    fs::write(dir.join("a.txt"), "hello\n").unwrap();
    fs::write(dir.join("b.txt"), "world\n").unwrap();

    run_ok(ProcCommand::new("git").args(["add", "."]).current_dir(&dir));
    run_ok(
        ProcCommand::new("git")
            .args(["commit", "-m", "baseline"])
            .current_dir(&dir),
    );

    // Uncommitted change for autopilot to "see"
    fs::write(dir.join("a.txt"), "hello changed\n").unwrap();

    dir
}

fn build_preview_command() -> Command {
    Command {
        metadata: Metadata::now(),
        command_type: CommandType::Preview,
        action: Some(ACTION_GIT_AUTOPILOT_PREVIEW.to_string()),
        payload: Some(Payload {
            payload_type: Some(PREVIEW_PAYLOAD_TYPE_V1.to_string()),
            payload: Some(pjson!({
                "request_id": "it-dryrun-001",
                "details": "integration test dry-run",
                "policy_overrides": pjson!(null)
            })),
        }),
    }
}

/// Helper: fetch payload JSON from CommandResponse
fn response_payload_json(resp_payload: &Payload) -> Json {
    resp_payload
        .payload
        .clone()
        .expect("response payload.value should be Some")
}

#[test]
fn preview_dry_run_does_not_create_commit() {
    let _guard = CWD_LOCK.lock().unwrap();

    let repo = init_temp_repo_with_change();
    let old_cwd = env::current_dir().expect("current_dir");
    env::set_current_dir(&repo).expect("set_current_dir to temp repo");

    // HEAD before
    let head_before = git(&repo, &["rev-parse", "HEAD"]);
    // also ensure we do have a diff before
    let diff_before = git(&repo, &["diff", "--name-only"]);
    assert!(
        diff_before.contains("a.txt"),
        "expected a.txt to be modified before preview (diff_before={diff_before:?})"
    );

    let resp = handle_command(build_preview_command());

    // restore cwd even if asserts later panic (best effort)
    env::set_current_dir(&old_cwd).ok();

    // --- Router contract ---
    assert_eq!(resp.status.code, 200, "status message={:?}", resp.message);
    assert_eq!(resp.status.description, "Success");
    assert!(resp.error.is_none(), "error should be None on success");

    let rp = resp
        .payload
        .as_ref()
        .expect("response.payload should be Some");
    assert_eq!(
        rp.payload_type.as_deref(),
        Some(RESPONSE_TYPE_PREVIEW),
        "unexpected response payload_type"
    );

    // --- Parse returned payload (PreviewResponse) ---
    // Your router puts serialized res in BOTH: message (stringified) and payload (value).
    // We use payload (robust).
    let body = response_payload_json(rp);

    // Minimal invariants from handlers.rs:
    // PreviewResponse { summary: "Prévisualisation réussie", payload: Some(report_as_value) }
    assert_eq!(
        body.get("summary").and_then(|v| v.as_str()),
        Some("Prévisualisation réussie"),
        "unexpected summary: {body:?}"
    );
    assert!(
        body.get("payload").is_some(),
        "PreviewResponse.payload should exist (report as JSON value)"
    );

    // --- Critical dry-run invariant: NO new commit ---
    let head_after = git(&repo, &["rev-parse", "HEAD"]);
    assert_eq!(
        head_after, head_before,
        "DryRun should not change HEAD (commit history). before={head_before} after={head_after}"
    );

    // And the working tree change should still be there (no forced commit).
    let diff_after = git(&repo, &["diff", "--name-only"]);
    assert!(
        diff_after.contains("a.txt"),
        "Expected a.txt to remain modified after dry-run preview (diff_after={diff_after:?})"
    );
}
