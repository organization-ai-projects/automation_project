use crate::model::block::Block;
use crate::model::document::Document;
use crate::model::inline::Inline;

use super::layout_node::LayoutNode;

pub struct LayoutEngine;

impl LayoutEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn layout(&self, doc: &Document) -> Vec<LayoutNode> {
        doc.blocks.iter().map(|block| Self::layout_block(block)).collect()
    }

    fn layout_block(block: &Block) -> LayoutNode {
        match block {
            Block::Heading { id, level, content, .. } => LayoutNode::Block {
                block_id: id.clone(),
                kind: format!("heading-{level}"),
                children: Self::layout_inlines(content),
            },
            Block::Paragraph { id, content, .. } => LayoutNode::Block {
                block_id: id.clone(),
                kind: "paragraph".into(),
                children: Self::layout_inlines(content),
            },
            Block::List { id, items, ordered, .. } => {
                let kind = if *ordered { "ordered-list" } else { "unordered-list" };
                let children = items
                    .iter()
                    .map(|item| LayoutNode::Block {
                        block_id: id.clone(),
                        kind: "list-item".into(),
                        children: Self::layout_inlines(item),
                    })
                    .collect();
                LayoutNode::Block { block_id: id.clone(), kind: kind.into(), children }
            }
            Block::CodeBlock { id, language, code, .. } => {
                let kind = language
                    .as_deref()
                    .map(|l| format!("code-{l}"))
                    .unwrap_or_else(|| "code".into());
                LayoutNode::Block {
                    block_id: id.clone(),
                    kind,
                    children: vec![LayoutNode::Inline { kind: "code".into(), text: code.clone() }],
                }
            }
            Block::Quote { id, content, .. } => LayoutNode::Block {
                block_id: id.clone(),
                kind: "quote".into(),
                children: Self::layout_inlines(content),
            },
        }
    }

    fn layout_inlines(inlines: &[Inline]) -> Vec<LayoutNode> {
        inlines.iter().map(|inline| Self::layout_inline(inline)).collect()
    }

    fn layout_inline(inline: &Inline) -> LayoutNode {
        match inline {
            Inline::Text(s) => LayoutNode::Inline { kind: "text".into(), text: s.clone() },
            Inline::Bold(children) => LayoutNode::Inline {
                kind: "bold".into(),
                text: children
                    .iter()
                    .map(|c| Self::inline_text(c))
                    .collect::<Vec<_>>()
                    .join(""),
            },
            Inline::Italic(children) => LayoutNode::Inline {
                kind: "italic".into(),
                text: children
                    .iter()
                    .map(|c| Self::inline_text(c))
                    .collect::<Vec<_>>()
                    .join(""),
            },
            Inline::CodeSpan(s) => LayoutNode::Inline { kind: "code-span".into(), text: s.clone() },
            Inline::Link { href, text } => {
                LayoutNode::Inline { kind: format!("link:{href}"), text: text.clone() }
            }
        }
    }

    fn inline_text(inline: &Inline) -> String {
        match inline {
            Inline::Text(s) => s.clone(),
            Inline::Bold(children) | Inline::Italic(children) => {
                children.iter().map(|c| Self::inline_text(c)).collect::<Vec<_>>().join("")
            }
            Inline::CodeSpan(s) => s.clone(),
            Inline::Link { text, .. } => text.clone(),
        }
    }
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}
