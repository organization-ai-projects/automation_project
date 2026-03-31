pub struct ArtifactWriter;

impl ArtifactWriter {
    pub fn write(path: &str, content: &str) -> Result<(), String> {
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory for {path}: {e}"))?;
        }
        std::fs::write(path, content)
            .map_err(|e| format!("Failed to write artifact to {path}: {e}"))
    }
}
