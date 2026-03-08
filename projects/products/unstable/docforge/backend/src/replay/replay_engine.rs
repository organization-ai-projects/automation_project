use crate::diagnostics::error::Error;
use crate::edit::edit_tx::EditTx;
use crate::model::document::Document;

use super::doc_event::DocEvent;

pub struct ReplayEngine;

impl ReplayEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn replay(&self, doc: &mut Document, events: &[DocEvent]) -> Result<(), Error> {
        let mut sorted: Vec<&DocEvent> = events.iter().collect();
        sorted.sort_by_key(|e| e.sequence);
        let mut previous_sequence: Option<u64> = None;
        for event in sorted {
            if event.doc_id != doc.id {
                return Err(Error::Replay(
                    "event doc_id does not match target document".to_string(),
                ));
            }
            if let Some(previous) = previous_sequence
                && event.sequence <= previous
            {
                return Err(Error::Replay(
                    "event sequence must be strictly increasing".to_string(),
                ));
            }
            let tx = EditTx::from_ops(event.ops.clone());
            tx.apply(doc)?;
            previous_sequence = Some(event.sequence);
        }
        Ok(())
    }
}

impl Default for ReplayEngine {
    fn default() -> Self {
        Self::new()
    }
}
