use super::block_id::BlockId;
use super::inline::Inline;
use super::style_id::StyleId;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Block {
    Heading {
        id: BlockId,
        level: u8,
        content: Vec<Inline>,
        style: Option<StyleId>,
    },
    Paragraph {
        id: BlockId,
        content: Vec<Inline>,
        style: Option<StyleId>,
    },
    List {
        id: BlockId,
        items: Vec<Vec<Inline>>,
        ordered: bool,
        style: Option<StyleId>,
    },
    CodeBlock {
        id: BlockId,
        language: Option<String>,
        code: String,
        style: Option<StyleId>,
    },
    Quote {
        id: BlockId,
        content: Vec<Inline>,
        style: Option<StyleId>,
    },
}

impl Block {
    pub fn id(&self) -> &BlockId {
        match self {
            Block::Heading { id, .. } => id,
            Block::Paragraph { id, .. } => id,
            Block::List { id, .. } => id,
            Block::CodeBlock { id, .. } => id,
            Block::Quote { id, .. } => id,
        }
    }
}
