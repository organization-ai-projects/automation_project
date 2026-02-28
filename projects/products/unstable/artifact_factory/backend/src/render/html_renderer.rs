use crate::analyze::dependency_graph::DependencyGraph;
use crate::analyze::event_map::EventMap;
use crate::analyze::protocol_map::ProtocolMap;
use crate::input::artifact_input::ArtifactInput;
use crate::render::markdown_renderer::MarkdownRenderer;
use crate::render::svg_renderer::SvgRenderer;

/// Renders a deterministic HTML documentation page embedding markdown + SVG.
pub struct HtmlRenderer;

impl HtmlRenderer {
    pub fn render(
        inputs: &[ArtifactInput],
        event_map: &EventMap,
        protocol_map: &ProtocolMap,
        dep_graph: &DependencyGraph,
    ) -> String {
        let markdown_body = MarkdownRenderer::render(inputs, event_map, protocol_map, dep_graph);
        let svg_graph = SvgRenderer::render(dep_graph);
        // Convert minimal markdown to HTML (just wrap in <pre> for simplicity — deterministic)
        let html_body = markdown_body.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8"/>
<title>Artifact Factory Documentation</title>
<style>body{{font-family:sans-serif;max-width:900px;margin:2rem auto;}} pre{{background:#f3f4f6;padding:1rem;border-radius:4px;}}</style>
</head>
<body>
<h1>Artifact Factory</h1>
<section id="dependency-graph">
{svg_graph}
</section>
<section id="docs">
<pre>{html_body}</pre>
</section>
</body>
</html>
"#
        )
    }
}
