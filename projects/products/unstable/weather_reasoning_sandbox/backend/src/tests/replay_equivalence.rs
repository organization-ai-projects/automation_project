use crate::application::run_simulation::RunSimulation;
use crate::infrastructure::dataset_loader::DatasetLoader;
use crate::infrastructure::dataset_parser::DatasetParser;
use crate::replay::replay_runner::ReplayRunner;
use crate::reporting::canonical_report_builder::CanonicalReportBuilder;
use crate::reporting::canonical_snapshot_builder::CanonicalSnapshotBuilder;
use crate::simulation::simulation_engine::SimulationEngine;

#[test]
fn run_and_replay_produce_identical_checksums() {
    let output1 = RunSimulation::execute(42, 5, None).unwrap();

    let (id, obs) = DatasetLoader::load_default(42, 5);
    let cs = DatasetParser::compute_checksum(&obs);
    let sim_output = SimulationEngine::run(42, 5, id, cs, obs);

    let snapshot2 = CanonicalSnapshotBuilder::build(&sim_output).unwrap();
    let report2 =
        CanonicalReportBuilder::build(&sim_output, Some(snapshot2.snapshot_checksum.clone()), None)
            .unwrap();

    assert_eq!(output1.report.report_checksum, report2.report_checksum);
    assert_eq!(
        output1.snapshot.snapshot_checksum,
        snapshot2.snapshot_checksum
    );
}

#[test]
fn replay_from_journal_yields_identical_output() {
    let (id, obs) = DatasetLoader::load_default(42, 5);
    let cs = DatasetParser::compute_checksum(&obs);
    let output1 = SimulationEngine::run(42, 5, id, cs, obs);

    let snapshot1 = CanonicalSnapshotBuilder::build(&output1).unwrap();
    let report1 =
        CanonicalReportBuilder::build(&output1, Some(snapshot1.snapshot_checksum.clone()), None)
            .unwrap();

    let (report2, snapshot2, replay_result) =
        ReplayRunner::replay_from_journal(&output1.journal).unwrap();

    assert_eq!(report1.report_checksum, report2.report_checksum);
    assert_eq!(
        snapshot1.snapshot_checksum,
        snapshot2.snapshot_checksum
    );
    assert!(replay_result.is_equivalent);
}

#[test]
fn identical_seed_and_dataset_yield_identical_outputs() {
    let output1 = RunSimulation::execute(123, 8, None).unwrap();
    let output2 = RunSimulation::execute(123, 8, None).unwrap();

    assert_eq!(
        output1.report.report_checksum,
        output2.report.report_checksum
    );
    assert_eq!(
        output1.snapshot.snapshot_checksum,
        output2.snapshot.snapshot_checksum
    );
}

#[test]
fn different_seeds_yield_deterministic_but_different_outputs() {
    let output1 = RunSimulation::execute(1, 5, None).unwrap();
    let output2 = RunSimulation::execute(2, 5, None).unwrap();

    assert_ne!(
        output1.report.report_checksum,
        output2.report.report_checksum
    );

    // But each is internally deterministic
    let output1b = RunSimulation::execute(1, 5, None).unwrap();
    assert_eq!(
        output1.report.report_checksum,
        output1b.report.report_checksum
    );
}
