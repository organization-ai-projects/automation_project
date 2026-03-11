use crate::dataset_engine::{DatasetEntry, DatasetQualityReport, DatasetStore, Outcome};
use crate::moe_core::{ExpertId, TaskId};
use std::collections::HashMap;

fn make_entry(id: &str, task: &str, expert: &str, outcome: Outcome) -> DatasetEntry {
    DatasetEntry {
        id: id.to_string(),
        task_id: TaskId::new(task),
        expert_id: ExpertId::new(expert),
        input: "input".to_string(),
        output: "output".to_string(),
        outcome,
        score: Some(0.9),
        tags: vec!["tag1".to_string()],
        created_at: 1,
        metadata: HashMap::new(),
    }
}

#[test]
fn add_entry_and_count() {
    let mut store = DatasetStore::new();
    store.add_entry(make_entry("d1", "t1", "e1", Outcome::Success));
    store.add_entry(make_entry("d2", "t1", "e2", Outcome::Failure));
    assert_eq!(store.count(), 2);
}

#[test]
fn get_by_task() {
    let mut store = DatasetStore::new();
    store.add_entry(make_entry("d1", "t1", "e1", Outcome::Success));
    store.add_entry(make_entry("d2", "t2", "e1", Outcome::Success));
    let results = store.get_by_task(&TaskId::new("t1"));
    assert_eq!(results.len(), 1);
}

#[test]
fn get_by_expert() {
    let mut store = DatasetStore::new();
    store.add_entry(make_entry("d1", "t1", "e1", Outcome::Success));
    store.add_entry(make_entry("d2", "t2", "e2", Outcome::Success));
    let results = store.get_by_expert(&ExpertId::new("e1"));
    assert_eq!(results.len(), 1);
}

#[test]
fn get_by_outcome_and_counts() {
    let mut store = DatasetStore::new();
    store.add_entry(make_entry("d1", "t1", "e1", Outcome::Success));
    store.add_entry(make_entry("d2", "t2", "e2", Outcome::Failure));
    assert_eq!(store.successful_count(), 1);
    assert_eq!(store.failed_count(), 1);
    let successes = store.get_by_outcome(&Outcome::Success);
    assert_eq!(successes.len(), 1);
}

#[test]
fn quality_report_tracks_score_and_correction_signals() {
    let mut store = DatasetStore::new();
    store.add_entry(make_entry("d1", "t1", "e1", Outcome::Success));
    let mut low_score = make_entry("d2", "t2", "e2", Outcome::Failure);
    low_score.score = Some(0.2);
    store.add_entry(low_score);

    store.add_correction(crate::dataset_engine::Correction {
        entry_id: "d2".to_string(),
        corrected_output: "fixed".to_string(),
        reason: "human".to_string(),
        corrected_at: 2,
    });

    let report: DatasetQualityReport = store.quality_report(0.5);
    assert_eq!(report.total_entries, 2);
    assert_eq!(report.scored_entries, 2);
    assert_eq!(report.low_score_entries, 1);
    assert_eq!(report.corrected_entries, 1);
    assert!((report.correction_ratio - 0.5).abs() < f64::EPSILON);
    assert!((report.success_ratio - 0.5).abs() < f64::EPSILON);
    assert!(report.average_score.is_some());
}
