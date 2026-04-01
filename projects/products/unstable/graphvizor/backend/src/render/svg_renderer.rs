use crate::graph::GraphDefinition;
use crate::layout::NodePosition;

/// Renders a deterministic SVG from graph definition and positioned nodes.
pub struct SvgRenderer;

/// Escapes XML special characters in text content.
fn escape_xml(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(ch),
        }
    }
    out
}

/// Truncates a string to at most `max_chars` Unicode characters.
fn truncate_chars(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        Some((byte_idx, _)) => &s[..byte_idx],
        None => s,
    }
}

impl SvgRenderer {
    pub fn render(graph: &GraphDefinition, positions: &[NodePosition]) -> String {
        let canonical = graph.canonicalize();

        // Build position lookup (sorted by id via BTreeMap)
        let pos_map: std::collections::BTreeMap<&str, (i64, i64)> = positions
            .iter()
            .map(|p| (p.id.as_str(), (p.x, p.y)))
            .collect();

        // Compute bounding box
        let min_x = positions.iter().map(|p| p.x).min().unwrap_or(0) - 100;
        let min_y = positions.iter().map(|p| p.y).min().unwrap_or(0) - 50;
        let max_x = positions.iter().map(|p| p.x).max().unwrap_or(0) + 100;
        let max_y = positions.iter().map(|p| p.y).max().unwrap_or(0) + 50;

        let width = max_x - min_x;
        let height = max_y - min_y;
        let offset_x = -min_x;
        let offset_y = -min_y;

        let mut svg = String::new();
        svg.push_str(&format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\">\n"
        ));

        // Arrow marker definition
        svg.push_str("  <defs>\n");
        svg.push_str("    <marker id=\"arrow\" viewBox=\"0 0 10 10\" refX=\"10\" refY=\"5\" markerWidth=\"6\" markerHeight=\"6\" orient=\"auto-start-reverse\">\n");
        svg.push_str("      <path d=\"M 0 0 L 10 5 L 0 10 z\" fill=\"#999\"/>\n");
        svg.push_str("    </marker>\n");
        svg.push_str("  </defs>\n");

        // Draw edges first (sorted canonically: by from, then to)
        for edge in &canonical.edges {
            if let (Some(&(fx, fy)), Some(&(tx, ty))) = (
                pos_map.get(edge.from.as_str()),
                pos_map.get(edge.to.as_str()),
            ) {
                let x1 = fx + offset_x;
                let y1 = fy + offset_y;
                let x2 = tx + offset_x;
                let y2 = ty + offset_y;
                svg.push_str(&format!(
                    "  <line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\" stroke=\"#999\" stroke-width=\"1\" marker-end=\"url(#arrow)\"/>\n"
                ));
            }
        }

        // Draw nodes (sorted canonically by id)
        for node in &canonical.nodes {
            if let Some(&(nx, ny)) = pos_map.get(node.id.as_str()) {
                let cx = nx + offset_x;
                let cy = ny + offset_y;
                let label = node.label.as_deref().unwrap_or(node.id.as_str());
                let display_label = escape_xml(truncate_chars(label, 20));
                svg.push_str(&format!(
                    "  <rect x=\"{}\" y=\"{}\" width=\"120\" height=\"40\" rx=\"6\" fill=\"#dbeafe\" stroke=\"#3b82f6\"/>\n",
                    cx - 60, cy - 20
                ));
                svg.push_str(&format!(
                    "  <text x=\"{cx}\" y=\"{}\" text-anchor=\"middle\" font-size=\"12\" fill=\"#1e40af\">{display_label}</text>\n",
                    cy + 5
                ));
            }
        }

        svg.push_str("</svg>\n");
        svg
    }
}
