use crate::graph::GraphDefinition;
use crate::layout::NodePosition;
use crate::render::svg_renderer::SvgRenderer;

/// Renders a complete HTML document wrapping the SVG output.
pub struct HtmlRenderer;

impl HtmlRenderer {
    pub fn render(graph: &GraphDefinition, positions: &[NodePosition]) -> String {
        let svg = SvgRenderer::render(graph, positions);
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("  <meta charset=\"utf-8\">\n");
        html.push_str("  <title>graphvizor</title>\n");
        html.push_str("</head>\n<body>\n");
        html.push_str(&svg);
        html.push_str("</body>\n</html>\n");
        html
    }
}
