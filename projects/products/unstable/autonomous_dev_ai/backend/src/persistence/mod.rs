// projects/products/unstable/autonomous_dev_ai/src/persistence/mod.rs
mod action_outcome_index;
mod action_outcome_stats;
mod artifacts;
mod config_io;
mod decision_inverted_index;
mod failure_inverted_index;
mod io_atomic;
mod learning_snapshot;
mod memory_state_index;
mod memory_store;
mod memory_transaction_journal;
mod utils;

#[cfg(test)]
mod tests;

pub(crate) use action_outcome_index::ActionOutcomeIndex;
pub(crate) use action_outcome_stats::ActionOutcomeStats;
pub(crate) use artifacts::Artifacts;
pub(crate) use config_io::{load_bin, load_ron, save_bin, save_ron};
pub(crate) use decision_inverted_index::DecisionInvertedIndex;
pub(crate) use failure_inverted_index::FailureInvertedIndex;
pub(crate) use learning_snapshot::LearningSnapshot;
pub(crate) use memory_state_index::MemoryStateIndex;
pub(crate) use memory_store::{
    append_learning_snapshot, load_action_outcome_index, load_decision_inverted_index,
    load_failure_inverted_index, load_memory_state_index, load_memory_state_with_fallback,
    load_recent_learning_snapshots, memory_transaction_completed, save_memory_state_transactional,
};
pub(crate) use utils::{ensure_parent_dir_exists, now_secs};
