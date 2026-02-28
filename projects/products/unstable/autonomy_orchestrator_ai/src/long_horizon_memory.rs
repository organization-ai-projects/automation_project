// projects/products/unstable/autonomy_orchestrator_ai/src/long_horizon_memory.rs
use crate::domain::{
    DecisionReliabilityInput, MemoryEntry, MemoryPolicy, RunReport, StageExecutionStatus,
    TerminalState,
};
use common_binary::{BinaryOptions, read_binary, write_binary};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const LONG_HORIZON_MEMORY_MAGIC: [u8; 4] = *b"AOLM";
const LONG_HORIZON_MEMORY_SCHEMA_ID: u64 = 1;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LongHorizonMemoryStore {
    pub entries: Vec<MemoryEntry>,
    pub next_run_index: u64,
    pub updated_at_unix_secs: u64,
}

fn memory_bin_options() -> BinaryOptions {
    BinaryOptions {
        magic: LONG_HORIZON_MEMORY_MAGIC,
        container_version: 1,
        schema_id: LONG_HORIZON_MEMORY_SCHEMA_ID,
        verify_checksum: true,
    }
}

pub fn load_memory(path: &Path) -> Result<LongHorizonMemoryStore, String> {
    read_binary(path, &memory_bin_options()).map_err(|e| {
        format!(
            "Failed to load long-horizon memory '{}': {e}",
            path.display()
        )
    })
}

pub fn save_memory(path: &Path, store: &LongHorizonMemoryStore) -> Result<(), String> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent).map_err(|e| {
            format!(
                "Failed to create memory parent dir '{}': {e}",
                parent.display()
            )
        })?;
    }
    write_binary(store, path, &memory_bin_options()).map_err(|e| {
        format!(
            "Failed to save long-horizon memory '{}': {e}",
            path.display()
        )
    })
}

/// Append a new entry to the store from the given run report.
pub fn record_run(
    store: &mut LongHorizonMemoryStore,
    report: &RunReport,
    timestamp: u64,
) -> MemoryEntry {
    let run_index = store.next_run_index;
    store.next_run_index += 1;

    let failure_signature = build_failure_signature(report);
    let terminal_state_code = report
        .terminal_state
        .map(|s| format!("{s:?}"))
        .unwrap_or_else(|| "Unknown".to_string());

    let entry = MemoryEntry {
        run_id: report.run_id.clone(),
        run_index,
        failure_signature,
        terminal_state_code,
        blocked_reason_codes: report.blocked_reason_codes.clone(),
        reliability_updates: report.decision_reliability_updates.clone(),
        recorded_at_unix_secs: timestamp,
    };
    store.entries.push(entry.clone());
    store.updated_at_unix_secs = timestamp;
    entry
}

fn build_failure_signature(report: &RunReport) -> Option<String> {
    if report.terminal_state == Some(TerminalState::Done) {
        return None;
    }
    let mut parts: Vec<String> = report.blocked_reason_codes.clone();
    for exec in &report.stage_executions {
        if matches!(
            exec.status,
            StageExecutionStatus::Failed
                | StageExecutionStatus::SpawnFailed
                | StageExecutionStatus::ArtifactMissing
        ) {
            parts.push(exec.command.clone());
        }
    }
    if parts.is_empty() {
        Some("unknown_failure".to_string())
    } else {
        Some(parts.join("|"))
    }
}

/// Derive reliability inputs from memory history within the decay window.
/// Returns `(inputs, reason_codes)`. Deterministic: sorted by (contributor_id, capability).
pub fn derive_reliability_inputs(
    store: &LongHorizonMemoryStore,
    policy: &MemoryPolicy,
) -> (Vec<DecisionReliabilityInput>, Vec<String>) {
    let window_start = store
        .next_run_index
        .saturating_sub(u64::from(policy.decay_window_runs));

    // Accumulate scores per (contributor_id, capability) using a sorted Vec for determinism.
    let mut acc: Vec<((String, String), (i32, u32))> = Vec::new();

    for entry in &store.entries {
        if entry.run_index < window_start {
            continue;
        }
        for update in &entry.reliability_updates {
            let key = (update.contributor_id.clone(), update.capability.clone());
            match acc.iter_mut().find(|(k, _)| k == &key) {
                Some((_, (sum, count))) => {
                    *sum += i32::from(update.new_score);
                    *count += 1;
                }
                None => {
                    acc.push((key, (i32::from(update.new_score), 1)));
                }
            }
        }
    }

    if acc.is_empty() {
        return (Vec::new(), Vec::new());
    }

    acc.sort_by(|(a, _), (b, _)| a.cmp(b));

    let inputs = acc
        .into_iter()
        .map(|((contributor_id, capability), (sum, count))| {
            let avg = (sum / i32::try_from(count).unwrap_or(1)).clamp(0, 100) as u8;
            DecisionReliabilityInput {
                contributor_id,
                capability,
                score: avg,
            }
        })
        .collect::<Vec<_>>();

    let reason_codes = vec!["MEMORY_SIGNAL_APPLIED".to_string()];
    (inputs, reason_codes)
}

/// Enforce retention and decay policy.
/// Entries outside the decay window are removed (MEMORY_ENTRY_DECAYED).
/// If still over `max_entries`, oldest entries are evicted (MEMORY_ENTRY_EVICTED).
/// Returns reason codes for actions taken.
pub fn enforce_policy(
    store: &mut LongHorizonMemoryStore,
    policy: &MemoryPolicy,
    timestamp: u64,
) -> Vec<String> {
    let mut reason_codes = Vec::new();
    let window_start = store
        .next_run_index
        .saturating_sub(u64::from(policy.decay_window_runs));

    let before_len = store.entries.len();
    store.entries.retain(|e| e.run_index >= window_start);
    if store.entries.len() < before_len {
        reason_codes.push("MEMORY_ENTRY_DECAYED".to_string());
    }

    let max = usize::try_from(policy.max_entries).unwrap_or(usize::MAX);
    if store.entries.len() > max {
        store.entries.drain(..store.entries.len() - max);
        reason_codes.push("MEMORY_ENTRY_EVICTED".to_string());
    }

    store.updated_at_unix_secs = timestamp;
    reason_codes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{DecisionReliabilityUpdate, MemoryPolicy, RunReport, TerminalState};

    fn make_store_with_runs(n: u64) -> LongHorizonMemoryStore {
        let mut store = LongHorizonMemoryStore::default();
        for i in 0..n {
            let mut report = RunReport::new(format!("run_{i}"));
            report.terminal_state = Some(TerminalState::Failed);
            report.blocked_reason_codes = vec!["GATE_CI_NOT_SUCCESS".to_string()];
            report.decision_reliability_updates = vec![DecisionReliabilityUpdate {
                contributor_id: "agent_a".to_string(),
                capability: "test".to_string(),
                previous_score: 50,
                new_score: 52,
                reason_code: "RELIABILITY_REWARD_ALIGNMENT".to_string(),
            }];
            record_run(&mut store, &report, 1000 + i);
        }
        store
    }

    #[test]
    fn record_run_increments_index_deterministically() {
        let mut store = LongHorizonMemoryStore::default();
        let mut report = RunReport::new("r1".to_string());
        report.terminal_state = Some(TerminalState::Done);
        let entry = record_run(&mut store, &report, 100);
        assert_eq!(entry.run_index, 0);
        assert_eq!(store.next_run_index, 1);

        let mut report2 = RunReport::new("r2".to_string());
        report2.terminal_state = Some(TerminalState::Failed);
        let entry2 = record_run(&mut store, &report2, 200);
        assert_eq!(entry2.run_index, 1);
        assert_eq!(store.next_run_index, 2);
    }

    #[test]
    fn done_run_has_no_failure_signature() {
        let mut store = LongHorizonMemoryStore::default();
        let mut report = RunReport::new("run_done".to_string());
        report.terminal_state = Some(TerminalState::Done);
        let entry = record_run(&mut store, &report, 0);
        assert!(entry.failure_signature.is_none());
    }

    #[test]
    fn failed_run_has_failure_signature() {
        let mut store = LongHorizonMemoryStore::default();
        let mut report = RunReport::new("run_fail".to_string());
        report.terminal_state = Some(TerminalState::Failed);
        report.blocked_reason_codes = vec!["GATE_CI_NOT_SUCCESS".to_string()];
        let entry = record_run(&mut store, &report, 0);
        assert!(entry.failure_signature.is_some());
        assert!(entry
            .failure_signature
            .unwrap()
            .contains("GATE_CI_NOT_SUCCESS"));
    }

    #[test]
    fn decay_removes_entries_outside_window() {
        let mut store = make_store_with_runs(10);
        let policy = MemoryPolicy {
            max_entries: 500,
            decay_window_runs: 5,
        };
        let codes = enforce_policy(&mut store, &policy, 9999);
        assert!(codes.contains(&"MEMORY_ENTRY_DECAYED".to_string()));
        assert_eq!(store.entries.len(), 5);
    }

    #[test]
    fn eviction_enforces_max_entries() {
        let mut store = make_store_with_runs(10);
        let policy = MemoryPolicy {
            max_entries: 3,
            decay_window_runs: 1000,
        };
        let codes = enforce_policy(&mut store, &policy, 9999);
        assert!(codes.contains(&"MEMORY_ENTRY_EVICTED".to_string()));
        assert_eq!(store.entries.len(), 3);
    }

    #[test]
    fn no_reason_codes_when_policy_is_not_triggered() {
        let mut store = make_store_with_runs(5);
        let policy = MemoryPolicy {
            max_entries: 500,
            decay_window_runs: 1000,
        };
        let codes = enforce_policy(&mut store, &policy, 0);
        assert!(codes.is_empty());
        assert_eq!(store.entries.len(), 5);
    }

    #[test]
    fn derive_reliability_inputs_is_deterministic() {
        let store = make_store_with_runs(3);
        let policy = MemoryPolicy::default();
        let (inputs1, codes1) = derive_reliability_inputs(&store, &policy);
        let (inputs2, codes2) = derive_reliability_inputs(&store, &policy);
        assert_eq!(inputs1, inputs2);
        assert_eq!(codes1, codes2);
        assert!(codes1.contains(&"MEMORY_SIGNAL_APPLIED".to_string()));
    }

    #[test]
    fn derive_reliability_inputs_ignores_entries_outside_window() {
        let mut store = make_store_with_runs(10);
        let policy = MemoryPolicy {
            max_entries: 500,
            decay_window_runs: 3,
        };
        // Entries 0-6 are outside the window (window_start = 10 - 3 = 7)
        let (inputs, _) = derive_reliability_inputs(&store, &policy);
        // Only runs 7, 8, 9 contribute
        assert!(!inputs.is_empty());
        // Average of 3 entries each contributing score=52 -> avg=52
        assert_eq!(inputs[0].score, 52);

        // Compare with full window
        let full_policy = MemoryPolicy {
            max_entries: 500,
            decay_window_runs: 1000,
        };
        let (full_inputs, _) = derive_reliability_inputs(&store, &full_policy);
        assert_eq!(full_inputs[0].score, 52); // all have same score so average is same

        // Add some entries with different score and verify windowing
        let mut report = RunReport::new("extra".to_string());
        report.terminal_state = Some(TerminalState::Done);
        report.decision_reliability_updates = vec![DecisionReliabilityUpdate {
            contributor_id: "agent_a".to_string(),
            capability: "test".to_string(),
            previous_score: 52,
            new_score: 80,
            reason_code: "RELIABILITY_REWARD_ALIGNMENT".to_string(),
        }];
        record_run(&mut store, &report, 9999);

        let (windowed, _) = derive_reliability_inputs(&store, &policy);
        // Window is now run_index >= 11 - 3 = 8
        // Runs 8, 9 have score 52, run 10 has score 80 -> avg = (52+52+80)/3 = 61
        assert_eq!(windowed[0].score, 61);
    }

    #[test]
    fn empty_store_returns_empty_inputs() {
        let store = LongHorizonMemoryStore::default();
        let policy = MemoryPolicy::default();
        let (inputs, codes) = derive_reliability_inputs(&store, &policy);
        assert!(inputs.is_empty());
        assert!(codes.is_empty());
    }
}
