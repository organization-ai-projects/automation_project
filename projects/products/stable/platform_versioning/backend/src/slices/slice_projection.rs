// projects/products/stable/platform_versioning/backend/src/slices/slice_projection.rs
use crate::errors::PvError;
use crate::issues::Issue;
use crate::slices::{SliceDefinition, SliceManifest};

/// Projects a [`SliceDefinition`] into a [`SliceManifest`] for a specific user.
///
/// The projection takes the issue's path allowlist and produces a deterministic
/// manifest that can be used by all downstream filtering. Forbidden paths are
/// never included: the manifest contains only what is explicitly allowed.
pub struct SliceProjection;

impl SliceProjection {
    /// Generates a [`SliceManifest`] for `subject` working on `issue`.
    ///
    /// When the issue has no [`SliceDefinition`] attached, returns an error
    /// because there is no scope to project.
    pub fn project(subject: &str, issue: &Issue) -> Result<SliceManifest, PvError> {
        let definition = issue.slice_definition.as_ref().ok_or_else(|| {
            PvError::SliceBuildFailed(format!("issue '{}' has no slice definition", issue.id))
        })?;

        Self::project_from_definition(subject, &issue.id.to_string(), definition)
    }

    /// Generates a [`SliceManifest`] directly from a [`SliceDefinition`].
    pub fn project_from_definition(
        subject: &str,
        issue_id: &str,
        definition: &SliceDefinition,
    ) -> Result<SliceManifest, PvError> {
        let mut allowed_paths: Vec<String> = definition
            .paths()
            .iter()
            .map(|p| p.as_str().to_string())
            .collect();

        // Ensure deterministic output by sorting.
        allowed_paths.sort();

        Ok(SliceManifest {
            subject: subject.to_string(),
            issue_id: issue_id.to_string(),
            allowed_paths,
        })
    }
}
