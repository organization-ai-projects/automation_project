//! projects/products/unstable/autonomous_dev_ai/src/persistence/memory_store.rs
use std::fs;
use std::path::Path;

use common_binary::{BinaryOptions, read_binary, write_binary};
use common_json::JsonDeserializable;
use common_ron::{read_ron, write_ron};

use crate::error::{AgentError, AgentResult};
use crate::memory_graph::MemoryGraph;

use super::action_outcome_index::ActionOutcomeIndex;
use super::decision_inverted_index::DecisionInvertedIndex;
use super::failure_inverted_index::FailureInvertedIndex;
use super::io_atomic::write_json_atomic;
use super::learning_snapshot::LearningSnapshot;
use super::memory_state_index::MemoryStateIndex;
use super::memory_transaction_journal::MemoryTransactionJournal;
use super::utils;

// ---------------------------------------------------------------------------
// Binary options — shared across all memory persistence calls.
// ---------------------------------------------------------------------------

const MEMORY_BINARY_OPTIONS: BinaryOptions = BinaryOptions {
    magic: *b"MEMO",
    container_version: 1,
    schema_id: 1,
    verify_checksum: true,
};

// ---------------------------------------------------------------------------
// Transactional save
// ---------------------------------------------------------------------------

/// Saves the full memory state atomically.
///
/// All files are written with atomic rename semantics. A transaction journal
/// (`txn.json`) is written before and after the operation so that interrupted
/// writes can be detected on the next load.
pub fn save_memory_state_transactional<P: AsRef<Path>>(
    base_path: P,
    memory: &MemoryGraph,
) -> AgentResult<MemoryStateIndex> {
    let base = base_path.as_ref();
    let ron_path = base.with_extension("ron");
    let bin_path = base.with_extension("bin");
    let idx_path = base.with_extension("idx.json");
    let fail_idx_path = base.with_extension("fail_idx.json");
    let decision_idx_path = base.with_extension("decision_idx.json");
    let action_outcome_idx_path = base.with_extension("action_outcome_idx.json");
    let txn_path = base.with_extension("txn.json");

    let files = vec![
        ron_path.display().to_string(),
        bin_path.display().to_string(),
        idx_path.display().to_string(),
        fail_idx_path.display().to_string(),
        decision_idx_path.display().to_string(),
        action_outcome_idx_path.display().to_string(),
    ];

    // Mark transaction as started.
    let start_journal = MemoryTransactionJournal {
        state: "started".to_string(),
        started_at_secs: utils::now_secs(),
        completed_at_secs: None,
        files: files.clone(),
    };
    write_json_atomic(&txn_path, &start_journal)?;

    // Write primary state — both formats are atomic internally.
    write_ron(&ron_path, memory).map_err(|e| AgentError::Ron(e.to_string()))?;
    write_binary(memory, &bin_path, &MEMORY_BINARY_OPTIONS)
        .map_err(|e| AgentError::Binary(e.to_string()))?;

    // Write derived indexes.
    let index = MemoryStateIndex::from_memory(memory);
    write_json_atomic(&idx_path, &index)?;

    let failure_index = FailureInvertedIndex::from_failures(&memory.failures);
    write_json_atomic(&fail_idx_path, &failure_index)?;

    let decision_index = DecisionInvertedIndex::from_decisions(&memory.decisions);
    write_json_atomic(&decision_idx_path, &decision_index)?;

    let action_outcome_index = ActionOutcomeIndex::from_memory(memory);
    write_json_atomic(&action_outcome_idx_path, &action_outcome_index)?;

    // Mark transaction as completed.
    let done_journal = MemoryTransactionJournal {
        state: "completed".to_string(),
        started_at_secs: start_journal.started_at_secs,
        completed_at_secs: Some(utils::now_secs()),
        files,
    };
    write_json_atomic(&txn_path, &done_journal)?;

    Ok(index)
}

// ---------------------------------------------------------------------------
// Load with fallback
// ---------------------------------------------------------------------------

/// Loads memory state, preferring the binary format for performance.
/// Falls back to RON if the binary file is absent.
/// Returns an error if neither file exists.
pub fn load_memory_state_with_fallback<P: AsRef<Path>>(base_path: P) -> AgentResult<MemoryGraph> {
    let base = base_path.as_ref();
    let bin_path = base.with_extension("bin");
    let ron_path = base.with_extension("ron");

    if bin_path.exists() {
        return read_binary(&bin_path, &MEMORY_BINARY_OPTIONS)
            .map_err(|e| AgentError::Binary(e.to_string()));
    }

    if ron_path.exists() {
        let memory: MemoryGraph =
            read_ron(&ron_path).map_err(|e| AgentError::Ron(e.to_string()))?;
        return Ok(memory);
    }

    Err(AgentError::State("no saved memory state found".to_string()))
}

// ---------------------------------------------------------------------------
// Index loaders
// ---------------------------------------------------------------------------

pub fn load_memory_state_index<P: AsRef<Path>>(
    base_path: P,
) -> AgentResult<Option<MemoryStateIndex>> {
    let idx_path = base_path.as_ref().with_extension("idx.json");
    if !idx_path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(&idx_path)?;
    let index: MemoryStateIndex =
        common_json::from_str(&content).map_err(|e| AgentError::Serialization(e.to_string()))?;
    Ok(Some(index))
}

pub fn load_failure_inverted_index<P: AsRef<Path>>(
    base_path: P,
) -> AgentResult<Option<FailureInvertedIndex>> {
    let idx_path = base_path.as_ref().with_extension("fail_idx.json");
    if !idx_path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(&idx_path)?;
    let index: FailureInvertedIndex = common_json::parse_str(&content)
        .and_then(|json| FailureInvertedIndex::from_json(&json))
        .map_err(|e| AgentError::Serialization(e.to_string()))?;
    Ok(Some(index))
}

pub fn load_decision_inverted_index<P: AsRef<Path>>(
    base_path: P,
) -> AgentResult<Option<DecisionInvertedIndex>> {
    let idx_path = base_path.as_ref().with_extension("decision_idx.json");
    if !idx_path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(&idx_path)?;
    let index: DecisionInvertedIndex = common_json::parse_str(&content)
        .and_then(|json| DecisionInvertedIndex::from_json(&json))
        .map_err(|e| AgentError::Serialization(e.to_string()))?;
    Ok(Some(index))
}

pub fn load_action_outcome_index<P: AsRef<Path>>(
    base_path: P,
) -> AgentResult<Option<ActionOutcomeIndex>> {
    let idx_path = base_path.as_ref().with_extension("action_outcome_idx.json");
    if !idx_path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(&idx_path)?;
    let index: ActionOutcomeIndex = common_json::parse_str(&content)
        .and_then(|json| ActionOutcomeIndex::from_json(&json))
        .map_err(|e| AgentError::Serialization(e.to_string()))?;
    Ok(Some(index))
}

// ---------------------------------------------------------------------------
// Transaction journal check
// ---------------------------------------------------------------------------

pub fn memory_transaction_completed<P: AsRef<Path>>(base_path: P) -> AgentResult<bool> {
    let txn_path = base_path.as_ref().with_extension("txn.json");
    if !txn_path.exists() {
        return Ok(true);
    }
    let content = fs::read_to_string(txn_path)?;
    let journal: MemoryTransactionJournal = common_json::parse_str(&content)
        .and_then(|json| MemoryTransactionJournal::from_json(&json))
        .map_err(|e| AgentError::Serialization(e.to_string()))?;
    Ok(journal.state == "completed" && journal.completed_at_secs.is_some())
}

// ---------------------------------------------------------------------------
// Learning snapshots
// ---------------------------------------------------------------------------

pub fn append_learning_snapshot<P: AsRef<Path>>(
    base_path: P,
    memory: &MemoryGraph,
    window_size: usize,
) -> AgentResult<LearningSnapshot> {
    let learning_path = base_path.as_ref().with_extension("learning.json");
    let mut snapshots = load_learning_snapshots_internal(&learning_path)?;

    let snapshot = LearningSnapshot::from_memory(memory);
    snapshots.push(snapshot.clone());

    let keep = window_size.max(1);
    if snapshots.len() > keep {
        let drop_count = snapshots.len() - keep;
        snapshots.drain(0..drop_count);
    }

    write_json_atomic(&learning_path, &snapshots)?;
    Ok(snapshot)
}

pub fn load_recent_learning_snapshots<P: AsRef<Path>>(
    base_path: P,
    window_size: usize,
) -> AgentResult<Vec<LearningSnapshot>> {
    let learning_path = base_path.as_ref().with_extension("learning.json");
    let snapshots = load_learning_snapshots_internal(&learning_path)?;

    let keep = window_size.max(1);
    if snapshots.len() <= keep {
        return Ok(snapshots);
    }
    let start = snapshots.len() - keep;
    Ok(snapshots[start..].to_vec())
}

fn load_learning_snapshots_internal(path: &Path) -> AgentResult<Vec<LearningSnapshot>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(path)?;
    let snapshots: Vec<LearningSnapshot> =
        common_json::from_str(&content).map_err(|e| AgentError::Serialization(e.to_string()))?;
    Ok(snapshots)
}
