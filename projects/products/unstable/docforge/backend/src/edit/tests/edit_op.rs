#![cfg(test)]

use crate::edit::edit_op::EditOp;
use crate::edit::edit_tx::EditTx;
use crate::model::block::Block;
use crate::model::block_id::BlockId;
use crate::model::doc_id::DocId;
use crate::model::document::Document;
use crate::model::inline::Inline;

fn make_doc() -> Document {
    Document::new(DocId::new("doc1"), "Test Doc")
}

fn make_paragraph(id: &str) -> Block {
    Block::Paragraph {
        id: BlockId::new(id),
        content: vec![Inline::Text("Hello".to_string())],
        style: None,
    }
}

#[test]
fn test_insert_block_op() {
    let mut doc = make_doc();
    let tx = EditTx::from_ops(vec![EditOp::InsertBlock {
        position: 0,
        block: make_paragraph("b1"),
    }]);
    let applied = tx.apply(&mut doc);
    assert!(applied.is_ok());
    assert_eq!(doc.blocks.len(), 1);
    assert_eq!(doc.blocks[0].id(), &BlockId::new("b1"));
}
