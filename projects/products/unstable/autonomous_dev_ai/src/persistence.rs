// projects/products/unstable/autonomous_dev_ai/src/persistence.rs
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use common_ron::{read_ron, write_ron};
use serde::{Deserialize, Serialize};

use crate::agent_config::AgentConfig;
use crate::error::{AgentError, AgentResult};
use crate::memory::{DecisionEntry, FailureEntry};
use crate::memory_graph::MemoryGraph;
// Load configuration from .bin file
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStateIndex {
    pub generated_at_secs: u64,
    pub explored_files_count: usize,
    pub plans_count: usize,
    pub decisions_count: usize,
    pub failures_count: usize,
    pub objective_evaluations_count: usize,
    pub metadata_keys_count: usize,
    pub max_iteration_seen: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureInvertedIndex {
    pub generated_at_secs: u64,
    pub by_kind: std::collections::HashMap<String, usize>,
    pub by_tool: std::collections::HashMap<String, usize>,
    pub by_iteration: std::collections::HashMap<usize, usize>,
    pub latest_failure_iteration: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionInvertedIndex {
    pub generated_at_secs: u64,
    pub by_action: std::collections::HashMap<String, usize>,
    pub by_iteration: std::collections::HashMap<usize, usize>,
    pub latest_decision_iteration: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MemoryTransactionJournal {
    state: String,
    started_at_secs: u64,
    completed_at_secs: Option<u64>,
    files: Vec<String>,
}

impl MemoryStateIndex {
    pub fn from_memory(memory: &MemoryGraph) -> Self {
        let max_iteration_seen = memory
            .plans
            .iter()
            .map(|p| p.iteration)
            .chain(memory.decisions.iter().map(|d| d.iteration))
            .chain(memory.failures.iter().map(|f| f.iteration))
            .chain(memory.objective_evaluations.iter().map(|o| o.iteration))
            .max()
            .unwrap_or(0);

        Self {
            generated_at_secs: now_secs(),
            explored_files_count: memory.explored_files.len(),
            plans_count: memory.plans.len(),
            decisions_count: memory.decisions.len(),
            failures_count: memory.failures.len(),
            objective_evaluations_count: memory.objective_evaluations.len(),
            metadata_keys_count: memory.metadata.len(),
            max_iteration_seen,
        }
    }
}

impl FailureInvertedIndex {
    pub fn from_failures(failures: &[FailureEntry]) -> Self {
        let mut by_kind = std::collections::HashMap::new();
        let mut by_tool = std::collections::HashMap::new();
        let mut by_iteration = std::collections::HashMap::new();
        let mut latest_failure_iteration = None;

        for failure in failures {
            let kind = infer_failure_kind(failure);
            *by_kind.entry(kind).or_insert(0) += 1;

            let tool = infer_failure_tool(failure);
            *by_tool.entry(tool).or_insert(0) += 1;

            *by_iteration.entry(failure.iteration).or_insert(0) += 1;
            latest_failure_iteration = Some(
                latest_failure_iteration
                    .map(|v: usize| v.max(failure.iteration))
                    .unwrap_or(failure.iteration),
            );
        }

        Self {
            generated_at_secs: now_secs(),
            by_kind,
            by_tool,
            by_iteration,
            latest_failure_iteration,
        }
    }
}

impl DecisionInvertedIndex {
    pub fn from_decisions(decisions: &[DecisionEntry]) -> Self {
        let mut by_action = std::collections::HashMap::new();
        let mut by_iteration = std::collections::HashMap::new();
        let mut latest_decision_iteration = None;

        for decision in decisions {
            let action = infer_decision_action(decision);
            *by_action.entry(action).or_insert(0) += 1;
            *by_iteration.entry(decision.iteration).or_insert(0) += 1;
            latest_decision_iteration = Some(
                latest_decision_iteration
                    .map(|v: usize| v.max(decision.iteration))
                    .unwrap_or(decision.iteration),
            );
        }

        Self {
            generated_at_secs: now_secs(),
            by_action,
            by_iteration,
            latest_decision_iteration,
        }
    }
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
    let txn_path = base.with_extension("txn.json");

    let files = vec![
        ron_path.display().to_string(),
        bin_path.display().to_string(),
        idx_path.display().to_string(),
        fail_idx_path.display().to_string(),
        decision_idx_path.display().to_string(),
    ];
    let start_journal = MemoryTransactionJournal {
        state: "started".to_string(),
        started_at_secs: now_secs(),
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

    let done_journal = MemoryTransactionJournal {
        state: "completed".to_string(),
        started_at_secs: start_journal.started_at_secs,
        completed_at_secs: Some(now_secs()),
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

fn write_json_atomic<P: AsRef<Path>, T: Serialize>(path: P, value: &T) -> AgentResult<()> {
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

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn infer_failure_kind(entry: &FailureEntry) -> String {
    let text = format!(
        "{} {}",
        entry.description.to_ascii_lowercase(),
        entry.error.to_ascii_lowercase()
    );

    if text.contains("policy") || text.contains("authorization") {
        return "policy".to_string();
    }
    if text.contains("timeout") {
        return "timeout".to_string();
    }
    if text.contains("circuit") {
        return "circuit_breaker".to_string();
    }
    if text.contains("resource") || text.contains("budget") {
        return "resource".to_string();
    }
    if text.contains("test") || text.contains("validation") {
        return "validation".to_string();
    }
    if text.contains("tool") {
        return "tool".to_string();
    }
    "other".to_string()
}

fn infer_failure_tool(entry: &FailureEntry) -> String {
    let text = format!(
        "{} {}",
        entry.description.to_ascii_lowercase(),
        entry.error.to_ascii_lowercase()
    );
    for tool in [
        "run_tests",
        "read_file",
        "git_commit",
        "generate_pr_description",
        "apply_patch",
        "format_code",
        "git_branch",
        "create_pr",
    ] {
        if text.contains(tool) {
            return tool.to_string();
        }
    }
    "unknown".to_string()
}

fn infer_decision_action(entry: &DecisionEntry) -> String {
    let text = format!(
        "{} {}",
        entry.description.to_ascii_lowercase(),
        entry.symbolic_decision.to_ascii_lowercase()
    );
    for action in [
        "run_tests",
        "read_file",
        "git_commit",
        "generate_pr_description",
        "apply_patch",
        "format_code",
        "git_branch",
        "create_pr",
    ] {
        if text.contains(action) {
            return action.to_string();
        }
    }
    "other".to_string()
}
