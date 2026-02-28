use crate::analyze::dependency_graph::DependencyGraph;
use crate::analyze::event_map::EventMap;
use crate::analyze::protocol_map::ProtocolMap;
use crate::input::artifact_input::ArtifactInput;

/// Renders a deterministic Markdown documentation bundle.
pub struct MarkdownRenderer;

impl MarkdownRenderer {
    pub fn render(
        inputs: &[ArtifactInput],
        event_map: &EventMap,
        protocol_map: &ProtocolMap,
        dep_graph: &DependencyGraph,
    ) -> String {
        let mut out = String::new();
        out.push_str("# Artifact Factory — Documentation Bundle\n\n");

        out.push_str("## Inputs\n\n");
        for input in inputs {
            out.push_str(&format!("- `{}` ({:?})\n", input.path, input.kind));
        }
        out.push('\n');

        out.push_str("## Event Map\n\n");
        if event_map.is_empty() {
            out.push_str("_No events detected._\n\n");
        } else {
            for (event, paths) in &event_map.entries {
                out.push_str(&format!("- **{}**: {}\n", event, paths.join(", ")));
            }
            out.push('\n');
        }

        out.push_str("## Protocol Map\n\n");
        if protocol_map.is_empty() {
            out.push_str("_No protocols detected._\n\n");
        } else {
            for (proto, paths) in &protocol_map.entries {
                out.push_str(&format!("- **{}**: {}\n", proto, paths.join(", ")));
            }
            out.push('\n');
        }

        out.push_str("## Dependency Graph\n\n");
        for (node, deps) in &dep_graph.edges {
            if deps.is_empty() {
                out.push_str(&format!("- `{}` (no dependencies)\n", node));
            } else {
                out.push_str(&format!("- `{}` → {}\n", node, deps.iter().map(|d| format!("`{d}`")).collect::<Vec<_>>().join(", ")));
            }
        }
        out.push('\n');

        out
    }
}
