use crate::diagnostics::error::FactoryError;
use crate::input::artifact_input::{ArtifactInput, ArtifactKind};

pub struct InputLoader;

impl InputLoader {
    pub fn load_from_paths(paths: &[String]) -> Result<Vec<ArtifactInput>, FactoryError> {
        let mut inputs = Vec::new();
        for path in paths {
            let content =
                std::fs::read_to_string(path).map_err(|e| FactoryError::Io(e.to_string()))?;
            let kind = classify_path(path);
            inputs.push(ArtifactInput {
                path: path.clone(),
                content,
                kind,
            });
        }
        Ok(inputs)
    }
}

fn classify_path(path: &str) -> ArtifactKind {
    if path.ends_with("_report.json") || path.contains("/reports/") {
        ArtifactKind::Report
    } else if path.ends_with("_replay.json") || path.contains("/replays/") {
        ArtifactKind::Replay
    } else if path.ends_with("manifest.json") || path.contains("/manifests/") {
        ArtifactKind::Manifest
    } else if path.ends_with(".proto") || path.ends_with("_schema.json") {
        ArtifactKind::ProtocolSchema
    } else {
        ArtifactKind::Unknown
    }
}
