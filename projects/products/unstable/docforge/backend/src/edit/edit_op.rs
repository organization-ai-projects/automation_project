use crate::model::block::Block;
use crate::model::block_id::BlockId;
use crate::model::inline::Inline;
use crate::model::style::Style;
use crate::model::style_id::StyleId;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum EditOp {
    InsertBlock {
        position: usize,
        block: Block,
    },
    DeleteBlock {
        block_id: BlockId,
    },
    ReplaceBlockContent {
        block_id: BlockId,
        new_content: Vec<Inline>,
    },
    SetTitle {
        title: String,
    },
    AddStyle {
        style: Style,
    },
    RemoveStyle {
        style_id: StyleId,
    },
    ApplyStyleToBlock {
        block_id: BlockId,
        style_id: StyleId,
    },
}
