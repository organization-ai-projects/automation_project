// projects/libraries/common_json/src/tests/merge_strategy.rs
use crate::merge_strategy::*;

#[test]
fn test_merge_strategy_default() {
    let strategy = MergeStrategy::default();
    assert_eq!(strategy, MergeStrategy::Replace);
}

#[test]
fn test_merge_strategy_equality() {
    let strategy1 = MergeStrategy::Replace;
    let strategy2 = MergeStrategy::Replace;
    let strategy3 = MergeStrategy::DeepMerge;
    assert_eq!(strategy1, strategy2);
    assert_ne!(strategy1, strategy3);
}

#[test]
fn test_merge_strategy_debug() {
    let strategy = MergeStrategy::Concat;
    assert_eq!(format!("{:?}", strategy), "Concat");
}
