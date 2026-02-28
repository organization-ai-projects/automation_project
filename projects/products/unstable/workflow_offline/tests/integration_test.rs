/// Integration tests for workflow_offline.
///
/// These tests invoke the compiled binary directly using the
/// `CARGO_BIN_EXE_workflow_offline` environment variable that Cargo
/// populates during `cargo test`.
use std::path::PathBuf;
use std::process::Command;

fn bin_path() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_workflow_offline"))
}

/// Writes a temporary TOML workflow file and returns its path.
/// The caller is responsible for deleting it (or leaving it; the OS
/// will clean up temp files eventually).
fn write_workflow(content: &str) -> PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    let path = std::env::temp_dir().join(format!("wf_offline_test_{pid}_{n}.toml"));
    std::fs::write(&path, content).expect("write temp workflow");
    path
}

// ---------------------------------------------------------------------------
// 3-job dependency chain: job1 <- job2 <- job3
// ---------------------------------------------------------------------------

const THREE_JOB_WORKFLOW: &str = r#"
name = "three_job_test"

[[jobs]]
id = "job1"
command = "echo"
args = ["job1"]

[[jobs]]
id = "job2"
command = "echo"
args = ["job2"]
deps = ["job1"]

[[jobs]]
id = "job3"
command = "echo"
args = ["job3"]
deps = ["job2"]
"#;

#[test]
#[cfg(unix)]
fn three_job_workflow_exits_zero() {
    let path = write_workflow(THREE_JOB_WORKFLOW);
    let status = Command::new(bin_path())
        .args(["run", path.to_str().unwrap()])
        .status()
        .expect("spawn binary");
    assert_eq!(status.code().unwrap(), 0);
}

#[test]
#[cfg(unix)]
fn three_job_workflow_json_report_ordering() {
    let path = write_workflow(THREE_JOB_WORKFLOW);
    let output = Command::new(bin_path())
        .args(["run", path.to_str().unwrap(), "--json"])
        .output()
        .expect("spawn binary");
    assert_eq!(output.status.code().unwrap(), 0);
    let report: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("valid JSON from --json");
    let jobs = report["jobs"].as_array().expect("jobs array");
    assert_eq!(jobs.len(), 3);
    let order: Vec<&str> = jobs
        .iter()
        .map(|j| j["job_id"].as_str().unwrap())
        .collect();
    // job1 must come before job2, job2 before job3
    let pos = |name: &str| order.iter().position(|&x| x == name).unwrap();
    assert!(pos("job1") < pos("job2"));
    assert!(pos("job2") < pos("job3"));
}

#[test]
#[cfg(unix)]
fn three_job_workflow_run_report_snapshot() {
    let path = write_workflow(THREE_JOB_WORKFLOW);
    let output = Command::new(bin_path())
        .args(["run", path.to_str().unwrap(), "--json"])
        .output()
        .expect("spawn binary");
    let report: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("valid JSON");
    assert_eq!(report["workflow_name"].as_str().unwrap(), "three_job_test");
    assert!(report["success"].as_bool().unwrap());
    assert_eq!(report["seed"].as_u64().unwrap(), 0);
    // Event log must be non-empty (serialised JSON array)
    let event_log_json = report["event_log_json"].as_str().unwrap();
    let events: serde_json::Value =
        serde_json::from_str(event_log_json).expect("event_log_json is valid JSON");
    assert_eq!(events["events"].as_array().unwrap().len(), 3);
}

// ---------------------------------------------------------------------------
// Cycle detection – exit code 3
// ---------------------------------------------------------------------------

const CYCLE_WORKFLOW: &str = r#"
name = "cycle_test"

[[jobs]]
id = "a"
command = "echo"
deps = ["b"]

[[jobs]]
id = "b"
command = "echo"
deps = ["a"]
"#;

#[test]
fn cycle_workflow_exits_three() {
    let path = write_workflow(CYCLE_WORKFLOW);
    let status = Command::new(bin_path())
        .args(["run", path.to_str().unwrap()])
        .status()
        .expect("spawn binary");
    assert_eq!(status.code().unwrap(), 3);
}

// ---------------------------------------------------------------------------
// Failing job – exit code 1
// ---------------------------------------------------------------------------

#[test]
#[cfg(unix)]
fn failing_job_exits_one() {
    let content = r#"
name = "fail_test"

[[jobs]]
id = "bad"
command = "false"
"#;
    let path = write_workflow(content);
    let status = Command::new(bin_path())
        .args(["run", path.to_str().unwrap()])
        .status()
        .expect("spawn binary");
    assert_eq!(status.code().unwrap(), 1);
}

// ---------------------------------------------------------------------------
// CLI parse error – exit code 2
// ---------------------------------------------------------------------------

#[test]
fn invalid_cli_exits_two() {
    let status = Command::new(bin_path())
        .args(["unknown_command"])
        .status()
        .expect("spawn binary");
    assert_eq!(status.code().unwrap(), 2);
}

// ---------------------------------------------------------------------------
// Dry-run validation
// ---------------------------------------------------------------------------

#[test]
#[cfg(unix)]
fn dry_run_exits_zero_without_executing() {
    // Use `false` as command: dry-run must NOT execute it.
    let content = r#"
name = "dry_run_test"

[[jobs]]
id = "would_fail"
command = "false"
"#;
    let path = write_workflow(content);
    let status = Command::new(bin_path())
        .args(["run", path.to_str().unwrap(), "--dry-run"])
        .status()
        .expect("spawn binary");
    assert_eq!(status.code().unwrap(), 0);
}

// ---------------------------------------------------------------------------
// Determinism – same workflow + same seed => identical RunReport
// ---------------------------------------------------------------------------

#[test]
#[cfg(unix)]
fn same_seed_produces_identical_run_report() {
    let content = r#"
name = "determinism_test"

[[jobs]]
id = "x"
command = "echo"
args = ["x"]

[[jobs]]
id = "y"
command = "echo"
args = ["y"]
deps = ["x"]

[[jobs]]
id = "z"
command = "echo"
args = ["z"]
deps = ["x"]
"#;
    let path = write_workflow(content);

    let run = |seed: u64| -> serde_json::Value {
        let out = Command::new(bin_path())
            .args([
                "run",
                path.to_str().unwrap(),
                "--seed",
                &seed.to_string(),
                "--json",
            ])
            .output()
            .expect("spawn binary");
        serde_json::from_slice(&out.stdout).expect("valid JSON")
    };

    let r1 = run(99);
    let r2 = run(99);

    // Execution order must be identical
    let order = |r: &serde_json::Value| -> Vec<String> {
        r["jobs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|j| j["job_id"].as_str().unwrap().to_string())
            .collect()
    };
    assert_eq!(order(&r1), order(&r2));

    // Event logs must be identical
    assert_eq!(r1["event_log_json"], r2["event_log_json"]);

    // Seed must be preserved
    assert_eq!(r1["seed"].as_u64().unwrap(), 99);
    assert_eq!(r2["seed"].as_u64().unwrap(), 99);
}
