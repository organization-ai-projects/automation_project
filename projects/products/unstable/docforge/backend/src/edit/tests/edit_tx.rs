#![cfg(test)]

use crate::edit::edit_op::EditOp;
use crate::edit::edit_tx::EditTx;
use crate::model::block::Block;
use crate::model::block_id::BlockId;
use crate::model::doc_id::DocId;
use crate::model::document::Document;
use crate::model::inline::Inline;

#[test]
fn test_edit_tx_applies_insert_and_title() {
    let mut doc = Document::new(DocId::new("doc"), "Initial");
    let tx = EditTx::from_ops(vec![
        EditOp::InsertBlock {
            position: 0,
            block: Block::Paragraph {
                id: BlockId::new("p1"),
                content: vec![Inline::Text("Hello".to_string())],
                style: None,
            },
        },
        EditOp::SetTitle {
            title: "Updated".to_string(),
        },
    ]);

    let applied = tx.apply(&mut doc);
    assert!(applied.is_ok());
    assert_eq!(doc.title, "Updated");
    assert_eq!(doc.blocks.len(), 1);
}

#[test]
fn test_edit_tx_rolls_back_on_failure() {
    let mut doc = Document::new(DocId::new("doc"), "Initial");
    let tx = EditTx::from_ops(vec![
        EditOp::SetTitle {
            title: "ShouldNotPersist".to_string(),
        },
        EditOp::DeleteBlock {
            block_id: BlockId::new("missing"),
        },
    ]);

    let applied = tx.apply(&mut doc);
    assert!(applied.is_err());
    assert_eq!(doc.title, "Initial");
    assert!(doc.blocks.is_empty());
}
