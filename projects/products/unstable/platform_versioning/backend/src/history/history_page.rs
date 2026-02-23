// projects/products/unstable/platform_versioning/backend/src/history/history_page.rs
use serde::{Deserialize, Serialize};

use crate::history::HistoryEntry;
use crate::ids::CommitId;

/// A paginated history result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryPage {
    /// The entries in this page, in reverse-chronological order.
    pub entries: Vec<HistoryEntry>,
    /// The cursor to pass to the next call to get the subsequent page, or `None`
    /// if this is the last page.
    pub next_cursor: Option<CommitId>,
}
