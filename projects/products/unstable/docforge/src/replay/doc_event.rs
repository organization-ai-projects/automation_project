use crate::edit::edit_op::EditOp;
use crate::model::doc_id::DocId;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DocEvent {
    pub sequence: u64,
    pub doc_id: DocId,
    pub ops: Vec<EditOp>,
}

impl DocEvent {
    pub fn new(sequence: u64, doc_id: DocId, ops: Vec<EditOp>) -> Self {
        Self {
            sequence,
            doc_id,
            ops,
        }
    }
}
