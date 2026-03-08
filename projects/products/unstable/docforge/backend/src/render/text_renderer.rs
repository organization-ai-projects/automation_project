use crate::model::block::Block;
use crate::model::document::Document;
use crate::model::inline::Inline;

pub struct TextRenderer;

impl TextRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, doc: &Document) -> String {
        let mut lines: Vec<String> = Vec::new();
        lines.push(doc.title.clone());
        lines.push("=".repeat(doc.title.len()));
        for block in &doc.blocks {
            lines.push(self.render_block(block));
        }
        lines.join("\n")
    }

    fn render_block(&self, block: &Block) -> String {
        match block {
            Block::Heading { level, content, .. } => {
                let prefix = "#".repeat(*level as usize);
                format!("{prefix} {}", self.render_inlines(content))
            }
            Block::Paragraph { content, .. } => self.render_inlines(content),
            Block::List { items, ordered, .. } => items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    if *ordered {
                        format!("{}. {}", i + 1, self.render_inlines(item))
                    } else {
                        format!("- {}", self.render_inlines(item))
                    }
                })
                .collect::<Vec<_>>()
                .join("\n"),
            Block::Code { language, code, .. } => {
                let fence = language.as_deref().unwrap_or("");
                format!("```{fence}\n{code}\n```")
            }
            Block::Quote { content, .. } => {
                format!("> {}", self.render_inlines(content))
            }
        }
    }

    fn render_inlines(&self, inlines: &[Inline]) -> String {
        inlines
            .iter()
            .map(|i| self.render_inline(i))
            .collect::<Vec<_>>()
            .join("")
    }

    fn render_inline(&self, inline: &Inline) -> String {
        match inline {
            Inline::Text(s) => s.clone(),
            Inline::Bold(children) => format!("**{}**", self.render_inlines(children)),
            Inline::Italic(children) => format!("_{}_", self.render_inlines(children)),
            Inline::CodeSpan(s) => format!("`{s}`"),
            Inline::Link { href, text } => format!("[{text}]({href})"),
        }
    }
}

impl Default for TextRenderer {
    fn default() -> Self {
        Self::new()
    }
}
