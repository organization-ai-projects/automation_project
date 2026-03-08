use crate::model::block::Block;
use crate::model::document::Document;
use crate::model::inline::Inline;

pub struct HtmlRenderer;

impl HtmlRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, doc: &Document) -> String {
        let mut out = String::from("<div class=\"docforge-doc\">\n");
        for block in &doc.blocks {
            out.push_str(&self.render_block(block));
            out.push('\n');
        }
        out.push_str("</div>");
        out
    }

    fn render_block(&self, block: &Block) -> String {
        match block {
            Block::Heading { level, content, .. } => {
                let tag = format!("h{level}");
                format!("<{tag}>{}</{tag}>", self.render_inlines(content))
            }
            Block::Paragraph { content, .. } => {
                format!("<p>{}</p>", self.render_inlines(content))
            }
            Block::List { items, ordered, .. } => {
                let tag = if *ordered { "ol" } else { "ul" };
                let items_html: String = items
                    .iter()
                    .map(|item| format!("<li>{}</li>", self.render_inlines(item)))
                    .collect::<Vec<_>>()
                    .join("");
                format!("<{tag}>{}</{tag}>", items_html)
            }
            Block::Code { language, code, .. } => {
                let lang_attr = language
                    .as_deref()
                    .map(|l| format!(" class=\"language-{}\"", Self::escape_html(l)))
                    .unwrap_or_default();
                format!(
                    "<pre><code{lang_attr}>{}</code></pre>",
                    Self::escape_html(code)
                )
            }
            Block::Quote { content, .. } => {
                format!("<blockquote>{}</blockquote>", self.render_inlines(content))
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
            Inline::Text(s) => Self::escape_html(s),
            Inline::Bold(children) => format!("<strong>{}</strong>", self.render_inlines(children)),
            Inline::Italic(children) => format!("<em>{}</em>", self.render_inlines(children)),
            Inline::CodeSpan(s) => format!("<code>{}</code>", Self::escape_html(s)),
            Inline::Link { href, text } => {
                format!(
                    "<a href=\"{}\">{}</a>",
                    Self::escape_html(href),
                    Self::escape_html(text)
                )
            }
        }
    }

    fn escape_html(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }
}

impl Default for HtmlRenderer {
    fn default() -> Self {
        Self::new()
    }
}
