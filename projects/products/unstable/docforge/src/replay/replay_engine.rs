use crate::diagnostics::error::DocError;
use crate::edit::edit_tx::EditTx;
use crate::model::document::Document;

use super::doc_event::DocEvent;

pub struct ReplayEngine;

impl ReplayEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn replay(&self, doc: &mut Document, events: &[DocEvent]) -> Result<(), DocError> {
        let mut sorted: Vec<&DocEvent> = events.iter().collect();
        sorted.sort_by_key(|e| e.sequence);
        for event in sorted {
            let tx = EditTx::from_ops(event.ops.clone());
            tx.apply(doc)?;
        }
        Ok(())
    }
}

impl Default for ReplayEngine {
    fn default() -> Self {
        Self::new()
    }
}
