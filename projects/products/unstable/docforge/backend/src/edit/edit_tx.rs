use crate::diagnostics::error::Error;
use crate::model::block::Block;
use crate::model::document::Document;

use super::edit_op::EditOp;

pub struct EditTx {
    ops: Vec<EditOp>,
}

impl EditTx {
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }

    pub fn from_ops(ops: Vec<EditOp>) -> Self {
        Self { ops }
    }

    pub fn apply(&self, doc: &mut Document) -> Result<(), Error> {
        let mut working = doc.clone();
        for op in &self.ops {
            apply_op(op, &mut working)?;
        }
        *doc = working;
        Ok(())
    }
}

impl Default for EditTx {
    fn default() -> Self {
        Self::new()
    }
}

fn apply_op(op: &EditOp, doc: &mut Document) -> Result<(), Error> {
    match op {
        EditOp::InsertBlock { position, block } => {
            let pos = (*position).min(doc.blocks.len());
            doc.blocks.insert(pos, block.clone());
        }
        EditOp::DeleteBlock { block_id } => {
            if let Some(idx) = doc.block_index(block_id) {
                doc.blocks.remove(idx);
            } else {
                return Err(Error::BlockNotFound(block_id.clone()));
            }
        }
        EditOp::ReplaceBlockContent {
            block_id,
            new_content,
        } => {
            if let Some(idx) = doc.block_index(block_id) {
                match &mut doc.blocks[idx] {
                    Block::Heading { content, .. } => *content = new_content.clone(),
                    Block::Paragraph { content, .. } => *content = new_content.clone(),
                    Block::Quote { content, .. } => *content = new_content.clone(),
                    _ => {
                        return Err(Error::InvalidOperation(
                            "Cannot replace content of this block type".into(),
                        ));
                    }
                }
            } else {
                return Err(Error::BlockNotFound(block_id.clone()));
            }
        }
        EditOp::SetTitle { title } => {
            doc.title = title.clone();
        }
        EditOp::AddStyle { style } => {
            doc.styles.insert(style.id.clone(), style.clone());
        }
        EditOp::RemoveStyle { style_id } => {
            doc.styles.remove(style_id);
        }
        EditOp::ApplyStyleToBlock { block_id, style_id } => {
            if let Some(idx) = doc.block_index(block_id) {
                match &mut doc.blocks[idx] {
                    Block::Heading { style, .. } => *style = Some(style_id.clone()),
                    Block::Paragraph { style, .. } => *style = Some(style_id.clone()),
                    Block::List { style, .. } => *style = Some(style_id.clone()),
                    Block::Code { style, .. } => *style = Some(style_id.clone()),
                    Block::Quote { style, .. } => *style = Some(style_id.clone()),
                }
            } else {
                return Err(Error::BlockNotFound(block_id.clone()));
            }
        }
    }
    Ok(())
}
