use crate::report::ReportGenerator;
use crate::scanner::WorkspaceScanner;

fn fixture_path() -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    format!("{manifest_dir}/src/tests/fixtures/sample_workspace")
}

#[test]
fn scan_is_deterministic() {
    let snap1 = WorkspaceScanner::scan(&fixture_path()).unwrap();
    let snap2 = WorkspaceScanner::scan(&fixture_path()).unwrap();
    assert_eq!(snap1.snapshot_hash, snap2.snapshot_hash);
}

#[test]
fn snapshot_hash_is_stable() {
    let snap1 = WorkspaceScanner::scan(&fixture_path()).unwrap();
    let snap2 = WorkspaceScanner::scan(&fixture_path()).unwrap();

    let report1 = ReportGenerator::generate_snapshot_report(&snap1).unwrap();
    let report2 = ReportGenerator::generate_snapshot_report(&snap2).unwrap();
    assert_eq!(report1, report2);
}

#[test]
fn golden_snapshot_report() {
    let snapshot = WorkspaceScanner::scan(&fixture_path()).unwrap();
    let report = ReportGenerator::generate_snapshot_report(&snapshot).unwrap();

    // Normalize path-dependent and hash-dependent values
    let normalized = report
        .replace(&fixture_path(), "<ROOT>")
        .replace(&snapshot.snapshot_hash, "<HASH>");

    let expected = include_str!("fixtures/golden/snapshot_report.json");
    assert_eq!(normalized, expected);
}

#[test]
fn snapshot_contains_expected_crates() {
    let snapshot = WorkspaceScanner::scan(&fixture_path()).unwrap();
    let crate_names: Vec<&str> = snapshot
        .crate_graph
        .crates
        .iter()
        .map(|c| c.name.as_str())
        .collect();
    assert_eq!(crate_names, vec!["crate_a", "crate_b", "crate_c"]);
}

#[test]
fn snapshot_contains_expected_public_items() {
    let snapshot = WorkspaceScanner::scan(&fixture_path()).unwrap();
    let item_names: Vec<&str> = snapshot
        .public_items
        .iter()
        .map(|i| i.name.as_str())
        .collect();
    assert!(item_names.contains(&"greet"));
    assert!(item_names.contains(&"Config"));
    assert!(item_names.contains(&"Processor"));
    assert!(item_names.contains(&"transform"));
    assert!(item_names.contains(&"Status"));
    assert!(item_names.contains(&"VERSION"));
    assert!(item_names.contains(&"Result"));
    assert!(item_names.contains(&"helper"));
    assert!(item_names.contains(&"MAX_SIZE"));
    assert!(item_names.contains(&"utils"));
}
