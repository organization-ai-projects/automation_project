#![cfg(test)]

use crate::model::block::Block;
use crate::model::block_id::BlockId;
use crate::model::doc_id::DocId;
use crate::model::document::Document;
use crate::model::inline::Inline;
use crate::render::html_renderer::HtmlRenderer;

#[test]
fn test_html_render_stable() {
    let mut doc = Document::new(DocId::new("doc1"), "Test");
    doc.blocks.push(Block::Paragraph {
        id: BlockId::new("p1"),
        content: vec![Inline::Text("Hello world".to_string())],
        style: None,
    });
    let renderer = HtmlRenderer::new();
    let first = renderer.render(&doc);
    let second = renderer.render(&doc);
    assert_eq!(first, second);
}
