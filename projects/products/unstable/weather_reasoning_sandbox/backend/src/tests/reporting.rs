use crate::application::export_report::ExportReport;
use crate::application::export_snapshot::ExportSnapshot;
use crate::application::run_simulation::RunSimulation;
use crate::reporting::checksum_generator::ChecksumGenerator;

#[test]
fn report_json_is_stable() {
    let output = RunSimulation::execute(42, 5, None).unwrap();
    let json1 = ExportReport::to_json(&output.report).unwrap();
    let json2 = ExportReport::to_json(&output.report).unwrap();
    assert_eq!(json1, json2);
}

#[test]
fn snapshot_json_is_stable() {
    let output = RunSimulation::execute(42, 5, None).unwrap();
    let json1 = ExportSnapshot::to_json(&output.snapshot).unwrap();
    let json2 = ExportSnapshot::to_json(&output.snapshot).unwrap();
    assert_eq!(json1, json2);
}

#[test]
fn report_checksum_is_stable_for_identical_inputs() {
    let output1 = RunSimulation::execute(42, 5, None).unwrap();
    let output2 = RunSimulation::execute(42, 5, None).unwrap();
    assert_eq!(
        output1.report.report_checksum,
        output2.report.report_checksum
    );
}

#[test]
fn snapshot_checksum_is_stable_for_identical_inputs() {
    let output1 = RunSimulation::execute(42, 5, None).unwrap();
    let output2 = RunSimulation::execute(42, 5, None).unwrap();
    assert_eq!(
        output1.snapshot.snapshot_checksum,
        output2.snapshot.snapshot_checksum
    );
}

#[test]
fn checksum_generator_produces_deterministic_output() {
    let data = "test data for checksum";
    let cs1 = ChecksumGenerator::compute(data);
    let cs2 = ChecksumGenerator::compute(data);
    assert_eq!(cs1, cs2);
    assert!(!cs1.0.is_empty());
}

#[test]
fn report_contains_all_tick_reports() {
    let output = RunSimulation::execute(42, 5, None).unwrap();
    assert_eq!(output.report.tick_reports.len(), 5);
    assert_eq!(output.report.metadata.tick_count, 5);
}

#[test]
fn report_contains_contradiction_summary() {
    let output = RunSimulation::execute(42, 10, None).unwrap();
    assert!(output.report.contradiction_count <= 10);
}

#[test]
fn canonical_report_json_is_byte_stable() {
    let output1 = RunSimulation::execute(77, 7, None).unwrap();
    let output2 = RunSimulation::execute(77, 7, None).unwrap();

    let json1 = ExportReport::to_json(&output1.report).unwrap();
    let json2 = ExportReport::to_json(&output2.report).unwrap();

    let cs1 = ChecksumGenerator::compute(&json1);
    let cs2 = ChecksumGenerator::compute(&json2);
    assert_eq!(cs1, cs2);
}
