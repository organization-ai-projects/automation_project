pub struct ArtifactReader;

impl ArtifactReader {
    pub fn read(path: &str) -> Result<String, String> {
        std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read artifact from {path}: {e}"))
    }
}
