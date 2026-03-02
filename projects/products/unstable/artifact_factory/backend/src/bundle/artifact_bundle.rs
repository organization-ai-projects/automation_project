use std::collections::BTreeMap;

/// A canonical, deterministic artifact bundle.
/// All fields use BTreeMap for stable iteration order.
#[derive(Debug, Clone, Default)]
pub struct ArtifactBundle {
    /// file name → content bytes (sorted by name for determinism)
    pub files: BTreeMap<String, Vec<u8>>,
    /// ordered list of file names (for manifest)
    pub manifest: Vec<String>,
}

impl ArtifactBundle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_file(&mut self, name: impl Into<String>, content: impl Into<Vec<u8>>) {
        let name = name.into();
        self.files.insert(name.clone(), content.into());
        // Rebuild manifest in sorted order for determinism
        self.manifest = self.files.keys().cloned().collect();
        self.manifest.sort();
    }

    pub fn file_names(&self) -> &[String] {
        &self.manifest
    }
}
