use std::fs;
use std::path::Path;

use common_ron::{read_ron, write_ron};

use crate::agent_config::AgentConfig;
use crate::error::{AgentError, AgentResult};
use crate::memory_graph::MemoryGraph;

mod action_outcome_index;
mod action_outcome_stats;
mod decision_inverted_index;
mod failure_inverted_index;
mod learning_snapshot;
mod memory_state_index;
mod memory_transaction_journal;
mod utils;

pub use action_outcome_index::ActionOutcomeIndex;
pub use action_outcome_stats::ActionOutcomeStats;
pub use decision_inverted_index::DecisionInvertedIndex;
pub use failure_inverted_index::FailureInvertedIndex;
pub use learning_snapshot::LearningSnapshot;
pub use memory_state_index::MemoryStateIndex;

use memory_transaction_journal::MemoryTransactionJournal;

/// Load configuration from .bin file
pub fn load_bin<P: AsRef<Path>>(path: P) -> AgentResult<AgentConfig> {
    let bytes = fs::read(path)?;
    bincode::decode_from_slice(&bytes, bincode::config::standard())
        .map(|(config, _)| config)
        .map_err(|e| AgentError::Bincode(format!("{:?}", e)))
}

/// Load configuration from .ron file
pub fn load_ron<P: AsRef<Path>>(path: P) -> AgentResult<AgentConfig> {
    read_ron(path).map_err(|e| AgentError::Ron(e.to_string()))
}

/// Save configuration to .bin file
pub fn save_bin<P: AsRef<Path>>(path: P, config: &AgentConfig) -> AgentResult<()> {
    let bytes = bincode::encode_to_vec(config, bincode::config::standard())
        .map_err(|e| AgentError::Bincode(format!("{:?}", e)))?;
    fs::write(path, bytes)?;
    Ok(())
}

/// Save configuration to .ron file
pub fn save_ron<P: AsRef<Path>>(path: P, config: &AgentConfig) -> AgentResult<()> {
    write_ron(path, config).map_err(|e| AgentError::Ron(e.to_string()))
}

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
    let start_journal = MemoryTransactionJournal {
        state: "started".to_string(),
        started_at_secs: utils::now_secs(),
        completed_at_secs: None,
        files: files.clone(),
    };
    write_json_atomic(&txn_path, &start_journal)?;

    let ron_content = ron::ser::to_string_pretty(memory, ron::ser::PrettyConfig::default())
        .map_err(|e| AgentError::Ron(e.to_string()))?;
    write_string_atomic(&ron_path, &ron_content)?;

    let bin_content = bincode::encode_to_vec(memory, bincode::config::standard())
        .map_err(|e| AgentError::Bincode(format!("{:?}", e)))?;
    write_bytes_atomic(&bin_path, &bin_content)?;

    let index = MemoryStateIndex::from_memory(memory);
    write_json_atomic(&idx_path, &index)?;
    let failure_index = FailureInvertedIndex::from_failures(&memory.failures);
    write_json_atomic(&fail_idx_path, &failure_index)?;
    let decision_index = DecisionInvertedIndex::from_decisions(&memory.decisions);
    write_json_atomic(&decision_idx_path, &decision_index)?;
    let action_outcome_index = ActionOutcomeIndex::from_memory(memory);
    write_json_atomic(&action_outcome_idx_path, &action_outcome_index)?;

    let done_journal = MemoryTransactionJournal {
        state: "completed".to_string(),
        started_at_secs: start_journal.started_at_secs,
        completed_at_secs: Some(utils::now_secs()),
        files,
    };
    write_json_atomic(&txn_path, &done_journal)?;

    Ok(index)
}

pub fn load_memory_state_with_fallback<P: AsRef<Path>>(base_path: P) -> AgentResult<MemoryGraph> {
    let base = base_path.as_ref();
    let bin_path = base.with_extension("bin");
    let ron_path = base.with_extension("ron");

    if bin_path.exists() {
        let bytes = fs::read(&bin_path)?;
        let (memory, _) = bincode::decode_from_slice(&bytes, bincode::config::standard())
            .map_err(|e| AgentError::Bincode(format!("{:?}", e)))?;
        return Ok(memory);
    }

    if ron_path.exists() {
        let content = fs::read_to_string(&ron_path)?;
        let memory = ron::from_str(&content).map_err(|e| AgentError::Ron(e.to_string()))?;
        let rebuilt = bincode::encode_to_vec(&memory, bincode::config::standard())
            .map_err(|e| AgentError::Bincode(format!("{:?}", e)))?;
        write_bytes_atomic(&bin_path, &rebuilt)?;
        return Ok(memory);
    }

    Err(AgentError::State("No saved state found".to_string()))
}

pub fn load_memory_state_index<P: AsRef<Path>>(
    base_path: P,
) -> AgentResult<Option<MemoryStateIndex>> {
    let idx_path = base_path.as_ref().with_extension("idx.json");
    if !idx_path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(&idx_path)?;
    let index: MemoryStateIndex =
        serde_json::from_str(&content).map_err(|e| AgentError::Serialization(e.to_string()))?;
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
    let index: FailureInvertedIndex =
        serde_json::from_str(&content).map_err(|e| AgentError::Serialization(e.to_string()))?;
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
    let index: DecisionInvertedIndex =
        serde_json::from_str(&content).map_err(|e| AgentError::Serialization(e.to_string()))?;
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
    let index: ActionOutcomeIndex =
        serde_json::from_str(&content).map_err(|e| AgentError::Serialization(e.to_string()))?;
    Ok(Some(index))
}

pub fn memory_transaction_completed<P: AsRef<Path>>(base_path: P) -> AgentResult<bool> {
    let txn_path = base_path.as_ref().with_extension("txn.json");
    if !txn_path.exists() {
        return Ok(true);
    }
    let content = fs::read_to_string(txn_path)?;
    let journal: MemoryTransactionJournal =
        serde_json::from_str(&content).map_err(|e| AgentError::Serialization(e.to_string()))?;
    Ok(journal.state == "completed" && journal.completed_at_secs.is_some())
}

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
        serde_json::from_str(&content).map_err(|e| AgentError::Serialization(e.to_string()))?;
    Ok(snapshots)
}

fn write_json_atomic<P: AsRef<Path>, T: serde::Serialize>(path: P, value: &T) -> AgentResult<()> {
    let content = serde_json::to_string_pretty(value)
        .map_err(|e| AgentError::Serialization(e.to_string()))?;
    write_string_atomic(path, &content)
}

fn write_string_atomic<P: AsRef<Path>>(path: P, content: &str) -> AgentResult<()> {
    write_bytes_atomic(path, content.as_bytes())
}

fn write_bytes_atomic<P: AsRef<Path>>(path: P, bytes: &[u8]) -> AgentResult<()> {
    let path = path.as_ref();
    let tmp_path = path.with_extension(format!(
        "{}.tmp",
        path.extension().and_then(|e| e.to_str()).unwrap_or("data")
    ));
    fs::write(&tmp_path, bytes)?;
    fs::rename(&tmp_path, path)?;
    Ok(())
}
