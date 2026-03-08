use crate::diagnostics::error::Error;
use crate::model::block::Block;
use crate::model::doc_id::DocId;
use crate::model::document::Document;
use crate::model::inline::Inline;
use crate::model::style::Style;
use crate::replay::doc_event::DocEvent;
use sha2::Digest;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DocSnapshot {
    pub doc_id: DocId,
    pub version: u64,
    pub document: Document,
    pub events: Vec<DocEvent>,
    pub checksum: String,
}

impl DocSnapshot {
    pub fn create(doc: &Document, version: u64, events: Vec<DocEvent>) -> Result<Self, Error> {
        let canonical = canonical_document_bytes(doc)?;
        let checksum = compute_sha256(&canonical);
        Ok(Self {
            doc_id: doc.id.clone(),
            version,
            document: doc.clone(),
            events,
            checksum,
        })
    }

    pub fn verify(&self) -> Result<(), Error> {
        let canonical = canonical_document_bytes(&self.document)?;
        let expected = compute_sha256(&canonical);
        if expected == self.checksum {
            Ok(())
        } else {
            Err(Error::ChecksumMismatch)
        }
    }
}

fn canonical_document_bytes(doc: &Document) -> Result<Vec<u8>, Error> {
    Ok(canonical_document_string(doc).into_bytes())
}

fn compute_sha256(data: &[u8]) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

fn canonical_document_string(doc: &Document) -> String {
    let mut out = String::new();
    out.push_str("doc|id=");
    append_escaped(&mut out, &doc.id.0);
    out.push_str("|title=");
    append_escaped(&mut out, &doc.title);

    out.push_str("|styles=");
    out.push_str(&doc.styles.len().to_string());
    for (style_id, style) in &doc.styles {
        out.push('|');
        append_escaped(&mut out, &style_id.0);
        out.push(':');
        append_style(&mut out, style);
    }

    out.push_str("|blocks=");
    out.push_str(&doc.blocks.len().to_string());
    for block in &doc.blocks {
        out.push('|');
        append_block(&mut out, block);
    }

    out
}

fn append_style(out: &mut String, style: &Style) {
    out.push_str("id=");
    append_escaped(out, &style.id.0);
    out.push_str(",font_family=");
    append_option_str(out, style.font_family.as_deref());
    out.push_str(",font_size_pt=");
    append_option_u32(out, style.font_size_pt);
    out.push_str(",bold=");
    out.push_str(if style.bold { "1" } else { "0" });
    out.push_str(",italic=");
    out.push_str(if style.italic { "1" } else { "0" });
    out.push_str(",color_hex=");
    append_option_str(out, style.color_hex.as_deref());
}

fn append_block(out: &mut String, block: &Block) {
    match block {
        Block::Heading {
            id,
            level,
            content,
            style,
        } => {
            out.push_str("Heading(");
            append_escaped(out, &id.0);
            out.push(',');
            out.push_str(&level.to_string());
            out.push(',');
            append_inlines(out, content);
            out.push(',');
            append_option_style_id(out, style.as_ref());
            out.push(')');
        }
        Block::Paragraph { id, content, style } => {
            out.push_str("Paragraph(");
            append_escaped(out, &id.0);
            out.push(',');
            append_inlines(out, content);
            out.push(',');
            append_option_style_id(out, style.as_ref());
            out.push(')');
        }
        Block::List {
            id,
            items,
            ordered,
            style,
        } => {
            out.push_str("List(");
            append_escaped(out, &id.0);
            out.push(',');
            out.push_str(if *ordered { "1" } else { "0" });
            out.push(',');
            out.push('[');
            for item in items {
                out.push('{');
                append_inlines(out, item);
                out.push('}');
            }
            out.push(']');
            out.push(',');
            append_option_style_id(out, style.as_ref());
            out.push(')');
        }
        Block::CodeBlock {
            id,
            language,
            code,
            style,
        } => {
            out.push_str("CodeBlock(");
            append_escaped(out, &id.0);
            out.push(',');
            append_option_str(out, language.as_deref());
            out.push(',');
            append_escaped(out, code);
            out.push(',');
            append_option_style_id(out, style.as_ref());
            out.push(')');
        }
        Block::Quote { id, content, style } => {
            out.push_str("Quote(");
            append_escaped(out, &id.0);
            out.push(',');
            append_inlines(out, content);
            out.push(',');
            append_option_style_id(out, style.as_ref());
            out.push(')');
        }
    }
}

fn append_inlines(out: &mut String, inlines: &[Inline]) {
    out.push('[');
    for inline in inlines {
        append_inline(out, inline);
        out.push(';');
    }
    out.push(']');
}

fn append_inline(out: &mut String, inline: &Inline) {
    match inline {
        Inline::Text(value) => {
            out.push_str("Text(");
            append_escaped(out, value);
            out.push(')');
        }
        Inline::Bold(children) => {
            out.push_str("Bold(");
            append_inlines(out, children);
            out.push(')');
        }
        Inline::Italic(children) => {
            out.push_str("Italic(");
            append_inlines(out, children);
            out.push(')');
        }
        Inline::CodeSpan(value) => {
            out.push_str("CodeSpan(");
            append_escaped(out, value);
            out.push(')');
        }
        Inline::Link { href, text } => {
            out.push_str("Link(");
            append_escaped(out, href);
            out.push(',');
            append_escaped(out, text);
            out.push(')');
        }
    }
}

fn append_option_str(out: &mut String, value: Option<&str>) {
    match value {
        Some(txt) => append_escaped(out, txt),
        None => out.push('-'),
    }
}

fn append_option_u32(out: &mut String, value: Option<u32>) {
    match value {
        Some(v) => out.push_str(&v.to_string()),
        None => out.push('-'),
    }
}

fn append_option_style_id(out: &mut String, value: Option<&crate::model::style_id::StyleId>) {
    match value {
        Some(style_id) => append_escaped(out, &style_id.0),
        None => out.push('-'),
    }
}

fn append_escaped(out: &mut String, value: &str) {
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '|' => out.push_str("\\|"),
            ',' => out.push_str("\\,"),
            ';' => out.push_str("\\;"),
            '[' => out.push_str("\\["),
            ']' => out.push_str("\\]"),
            '{' => out.push_str("\\{"),
            '}' => out.push_str("\\}"),
            '(' => out.push_str("\\("),
            ')' => out.push_str("\\)"),
            _ => out.push(ch),
        }
    }
}
