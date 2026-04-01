use crate::query::Query;
use crate::query_engine::QueryEngine;
use crate::report::ReportGenerator;
use crate::scanner::WorkspaceScanner;

fn fixture_path() -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    format!("{manifest_dir}/src/tests/fixtures/sample_workspace")
}

fn scan_fixture() -> crate::snapshot::Snapshot {
    WorkspaceScanner::scan(&fixture_path()).unwrap()
}

#[test]
fn reverse_deps_query() {
    let snapshot = scan_fixture();
    let query = Query::ReverseDeps {
        crate_name: "crate_a".to_string(),
    };
    let result = QueryEngine::execute(&snapshot, &query).unwrap();
    assert_eq!(result.matches, vec!["crate_b", "crate_c"]);
}

#[test]
fn public_items_query() {
    let snapshot = scan_fixture();
    let query = Query::PublicItems {
        crate_name: "crate_b".to_string(),
    };
    let result = QueryEngine::execute(&snapshot, &query).unwrap();
    assert_eq!(result.matches, vec!["Status", "transform"]);
}

#[test]
fn find_symbol_query() {
    let snapshot = scan_fixture();
    let query = Query::FindSymbol {
        substring: "helper".to_string(),
    };
    let result = QueryEngine::execute(&snapshot, &query).unwrap();
    assert_eq!(result.matches.len(), 1);
    assert!(result.matches[0].contains("helper"));
}

#[test]
fn query_results_are_sorted() {
    let snapshot = scan_fixture();

    let query = Query::ReverseDeps {
        crate_name: "crate_a".to_string(),
    };
    let result = QueryEngine::execute(&snapshot, &query).unwrap();

    let mut sorted = result.matches.clone();
    sorted.sort();
    assert_eq!(result.matches, sorted);
}

#[test]
fn query_results_are_deterministic() {
    let snapshot = scan_fixture();

    let query = Query::PublicItems {
        crate_name: "crate_c".to_string(),
    };
    let r1 = QueryEngine::execute(&snapshot, &query).unwrap();
    let r2 = QueryEngine::execute(&snapshot, &query).unwrap();
    assert_eq!(r1.matches, r2.matches);
}

#[test]
fn golden_query_result() {
    let snapshot = scan_fixture();
    let query = Query::ReverseDeps {
        crate_name: "crate_a".to_string(),
    };
    let result = QueryEngine::execute(&snapshot, &query).unwrap();
    let json = ReportGenerator::generate_query_report(&result).unwrap();

    let expected = include_str!("fixtures/golden/query_result.json");
    assert_eq!(json, expected);
}
