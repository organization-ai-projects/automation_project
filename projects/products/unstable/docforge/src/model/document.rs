use std::collections::BTreeMap;

use super::block::Block;
use super::block_id::BlockId;
use super::doc_id::DocId;
use super::style::Style;
use super::style_id::StyleId;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Document {
    pub id: DocId,
    pub title: String,
    pub blocks: Vec<Block>,
    pub styles: BTreeMap<StyleId, Style>,
}

impl Document {
    pub fn new(id: DocId, title: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            blocks: Vec::new(),
            styles: BTreeMap::new(),
        }
    }

    pub fn block_by_id(&self, id: &BlockId) -> Option<&Block> {
        self.blocks.iter().find(|b| b.id() == id)
    }

    pub fn block_index(&self, id: &BlockId) -> Option<usize> {
        self.blocks.iter().position(|b| b.id() == id)
    }
}
