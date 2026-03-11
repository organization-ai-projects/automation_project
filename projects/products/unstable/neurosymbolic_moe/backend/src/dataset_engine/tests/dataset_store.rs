use crate::dataset_engine::{DatasetEntry, DatasetStore, Outcome};
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
