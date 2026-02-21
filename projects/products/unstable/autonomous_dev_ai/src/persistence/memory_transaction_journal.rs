// projects/products/unstable/autonomous_dev_ai/src/persistence/memory_transaction_journal.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct MemoryTransactionJournal {
    pub(super) state: String,
    pub(super) started_at_secs: u64,
    pub(super) completed_at_secs: Option<u64>,
    pub(super) files: Vec<String>,
}
