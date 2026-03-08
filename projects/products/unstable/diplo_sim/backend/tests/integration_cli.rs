use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

fn bin_path() -> &'static str {
    env!("CARGO_BIN_EXE_diplo_sim_backend")
}

fn unique_tmp_dir() -> PathBuf {
    let mut dir = std::env::temp_dir();
    let counter = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
    dir.push(format!("diplo_sim_test_{}_{}", std::process::id(), counter));
    fs::create_dir_all(&dir).expect("create tmp dir");
    dir
}

#[test]
fn run_then_replay_produces_identical_report_bytes() {
    let map = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/tests/fixtures/maps/tiny_triangle_map.json"
    );

    let dir = unique_tmp_dir();
    let run_out = dir.join("run_report.json");
    let replay_file = dir.join("match.replay.json");
    let replay_out = dir.join("replay_report.json");

    let run_status = Command::new(bin_path())
        .args([
            "run",
            "--turns",
            "5",
            "--seed",
            "42",
            "--map",
            map,
            "--players",
            "2",
            "--out",
            run_out.to_str().expect("utf8 path"),
            "--replay-out",
            replay_file.to_str().expect("utf8 path"),
        ])
        .status()
        .expect("run command");
    assert!(run_status.success(), "run command should succeed");

    let replay_status = Command::new(bin_path())
        .args([
            "replay",
            "--replay",
            replay_file.to_str().expect("utf8 path"),
            "--out",
            replay_out.to_str().expect("utf8 path"),
        ])
        .status()
        .expect("replay command");
    assert!(replay_status.success(), "replay command should succeed");

    let run_bytes = fs::read(&run_out).expect("read run report");
    let replay_bytes = fs::read(&replay_out).expect("read replay report");
    assert_eq!(run_bytes, replay_bytes, "report bytes must match exactly");
}

#[test]
fn invalid_orders_exit_with_code_3() {
    let map = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/tests/fixtures/maps/tiny_triangle_map.json"
    );
    let orders = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/tests/fixtures/orders/invalid_orders.json"
    );

    let output = Command::new(bin_path())
        .args(["validate-orders", "--map", map, "--orders", orders])
        .output()
        .expect("validate-orders command");

    assert_eq!(output.status.code(), Some(3));
}

#[test]
fn run_with_map_id_then_replay_succeeds() {
    let dir = unique_tmp_dir();
    let run_out = dir.join("run_report_map_id.json");
    let replay_file = dir.join("match_map_id.replay.json");
    let replay_out = dir.join("replay_report_map_id.json");

    let run_status = Command::new(bin_path())
        .args([
            "run",
            "--turns",
            "3",
            "--seed",
            "7",
            "--map-id",
            "tiny_triangle",
            "--players",
            "2",
            "--out",
            run_out.to_str().expect("utf8 path"),
            "--replay-out",
            replay_file.to_str().expect("utf8 path"),
        ])
        .status()
        .expect("run map-id command");
    assert!(run_status.success(), "run with --map-id should succeed");

    let replay_status = Command::new(bin_path())
        .args([
            "replay",
            "--replay",
            replay_file.to_str().expect("utf8 path"),
            "--out",
            replay_out.to_str().expect("utf8 path"),
        ])
        .status()
        .expect("replay command");
    assert!(replay_status.success(), "replay command should succeed");

    let run_bytes = fs::read(&run_out).expect("read run report");
    let replay_bytes = fs::read(&replay_out).expect("read replay report");
    assert_eq!(run_bytes, replay_bytes);
}

#[test]
fn list_maps_writes_known_map_ids() {
    let dir = unique_tmp_dir();
    let maps_out = dir.join("maps.json");

    let status = Command::new(bin_path())
        .args(["list-maps", "--out", maps_out.to_str().expect("utf8 path")])
        .status()
        .expect("list-maps command");
    assert!(status.success(), "list-maps command should succeed");

    let maps_json = fs::read_to_string(&maps_out).expect("read maps json");
    let map_ids: Vec<String> = common_json::from_json_str(&maps_json).expect("parse maps json");
    assert_eq!(map_ids, vec!["tiny_triangle".to_string()]);
}

#[test]
fn map_info_writes_expected_shape() {
    let dir = unique_tmp_dir();
    let map_info_out = dir.join("map_info.json");

    let status = Command::new(bin_path())
        .args([
            "map-info",
            "--map-id",
            "tiny_triangle",
            "--out",
            map_info_out.to_str().expect("utf8 path"),
        ])
        .status()
        .expect("map-info command");
    assert!(status.success(), "map-info command should succeed");

    let json = fs::read_to_string(&map_info_out).expect("read map-info json");
    assert!(json.contains("\"map_id\":\"tiny_triangle\""));
    assert!(json.contains("\"territory_count\":3"));
    assert!(json.contains("\"adjacency_count\":3"));
    assert!(json.contains("\"starting_unit_count\":2"));
}
