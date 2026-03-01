use crate::model::block::Block;
use crate::model::block_id::BlockId;
use crate::model::inline::Inline;
use crate::model::style::Style;
use crate::model::style_id::StyleId;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum EditOp {
    InsertBlock { position: usize, block: Block },
    DeleteBlock { block_id: BlockId },
    ReplaceBlockContent { block_id: BlockId, new_content: Vec<Inline> },
    SetTitle { title: String },
    AddStyle { style: Style },
    RemoveStyle { style_id: StyleId },
    ApplyStyleToBlock { block_id: BlockId, style_id: StyleId },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::edit::edit_tx::EditTx;
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
            content: vec![Inline::Text("Hello".into())],
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
        tx.apply(&mut doc).unwrap();
        assert_eq!(doc.blocks.len(), 1);
        assert_eq!(doc.blocks[0].id(), &BlockId::new("b1"));
    }

    #[test]
    fn test_delete_block_op() {
        let mut doc = make_doc();
        let mut tx = EditTx::new();
        tx.add_op(EditOp::InsertBlock { position: 0, block: make_paragraph("b1") });
        tx.apply(&mut doc).unwrap();

        let tx2 = EditTx::from_ops(vec![EditOp::DeleteBlock { block_id: BlockId::new("b1") }]);
        tx2.apply(&mut doc).unwrap();
        assert!(doc.blocks.is_empty());
    }

    #[test]
    fn test_set_title_op() {
        let mut doc = make_doc();
        let tx = EditTx::from_ops(vec![EditOp::SetTitle { title: "New Title".into() }]);
        tx.apply(&mut doc).unwrap();
        assert_eq!(doc.title, "New Title");
    }
}
