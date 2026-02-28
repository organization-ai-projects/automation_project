use crate::analyze::dependency_graph::DependencyGraph;

/// Renders a deterministic SVG graph for the dependency graph.
/// Layout is deterministic: nodes sorted alphabetically, laid out in a grid.
pub struct SvgRenderer;

impl SvgRenderer {
    pub fn render(dep_graph: &DependencyGraph) -> String {
        let nodes: Vec<&String> = dep_graph.edges.keys().collect();
        let node_count = nodes.len();

        let cols = std::cmp::max(1, (node_count as f64).sqrt().ceil() as usize);
        let cell_w = 200usize;
        let cell_h = 80usize;
        let rows = (node_count + cols - 1) / cols;
        let width = cols * cell_w + 40;
        let height = rows * cell_h + 40;

        let mut svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}">"#
        );
        svg.push('\n');

        // Node positions (deterministic: sorted alphabetical, row-major grid)
        let mut positions: std::collections::BTreeMap<&str, (usize, usize)> =
            std::collections::BTreeMap::new();
        for (i, node) in nodes.iter().enumerate() {
            let col = i % cols;
            let row = i / cols;
            let cx = col * cell_w + 20 + cell_w / 2;
            let cy = row * cell_h + 20 + cell_h / 2;
            positions.insert(node.as_str(), (cx, cy));
        }

        // Draw edges
        for (from, deps) in &dep_graph.edges {
            if let Some(&(fx, fy)) = positions.get(from.as_str()) {
                for dep in deps {
                    if let Some(&(tx, ty)) = positions.get(dep.as_str()) {
                        svg.push_str(&format!(
                            "  <line x1=\"{fx}\" y1=\"{fy}\" x2=\"{tx}\" y2=\"{ty}\" stroke=\"#999\" stroke-width=\"1\"/>"
                        ));
                        svg.push('\n');
                    }
                }
            }
        }

        // Draw nodes
        for node in &nodes {
            let (cx, cy) = positions[node.as_str()];
            let label = if node.len() > 20 { &node[..20] } else { node };
            svg.push_str(&format!(
                "  <rect x=\"{}\" y=\"{}\" width=\"160\" height=\"40\" rx=\"6\" fill=\"#dbeafe\" stroke=\"#3b82f6\"/>",
                cx - 80, cy - 20
            ));
            svg.push('\n');
            svg.push_str(&format!(
                "  <text x=\"{cx}\" y=\"{}\" text-anchor=\"middle\" font-size=\"11\" fill=\"#1e40af\">{label}</text>",
                cy + 5
            ));
            svg.push('\n');
        }

        svg.push_str("</svg>\n");
        svg
    }
}
