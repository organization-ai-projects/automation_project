use crate::input::artifact_input::ArtifactInput;
use std::collections::BTreeMap;

/// A deterministic adjacency map of dependencies between artifact paths.
#[derive(Debug, Clone, Default)]
pub struct DependencyGraph {
    pub edges: BTreeMap<String, Vec<String>>,
}

impl DependencyGraph {
    pub fn build(inputs: &[ArtifactInput]) -> Self {
        let mut edges: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for input in inputs {
            for line in input.content.lines() {
                let trimmed = line.trim();
                if let Some(rest) = trimmed
                    .strip_prefix("depends_on:")
                    .or_else(|| trimmed.strip_prefix("DependsOn:"))
                {
                    let dep = rest.trim().to_string();
                    if !dep.is_empty() {
                        edges.entry(input.path.clone()).or_default().push(dep);
                    }
                }
            }
            // Ensure node always present even with no edges
            edges.entry(input.path.clone()).or_default();
        }
        // Sort all adjacency lists for determinism
        for deps in edges.values_mut() {
            deps.sort();
            deps.dedup();
        }
        Self { edges }
    }

    pub fn node_count(&self) -> usize {
        self.edges.len()
    }
}
