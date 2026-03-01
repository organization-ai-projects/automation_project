use crate::model::block_id::BlockId;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum LayoutNode {
    Block { block_id: BlockId, kind: String, children: Vec<LayoutNode> },
    Inline { kind: String, text: String },
}
