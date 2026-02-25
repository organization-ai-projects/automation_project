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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::issues::Issue;
    use crate::slices::SliceDefinition;

    fn make_issue_with_def(paths: &[&str]) -> Issue {
        let def =
            SliceDefinition::from_paths(paths.iter().map(|p| p.to_string()).collect()).unwrap();
        Issue {
            id: "iss-1".parse().unwrap(),
            repo_id: None,
            title: "Test".to_string(),
            description: None,
            assignees: vec!["alice".to_string()],
            shared_with: vec![],
            slice_definition: Some(def),
        }
    }

    fn make_issue_without_def() -> Issue {
        Issue {
            id: "iss-no-def".parse().unwrap(),
            repo_id: None,
            title: "No Def".to_string(),
            description: None,
            assignees: vec![],
            shared_with: vec![],
            slice_definition: None,
        }
    }

    #[test]
    fn projection_includes_allowed_paths() {
        let issue = make_issue_with_def(&["src", "docs"]);
        let manifest = SliceProjection::project("alice", &issue).unwrap();
        assert!(manifest.allowed_paths.contains(&"src".to_string()));
        assert!(manifest.allowed_paths.contains(&"docs".to_string()));
    }

    #[test]
    fn projection_excludes_forbidden_paths() {
        let issue = make_issue_with_def(&["src"]);
        let manifest = SliceProjection::project("alice", &issue).unwrap();
        assert!(!manifest.allows("tests/integration.rs"));
        assert!(!manifest.allowed_paths.contains(&"tests".to_string()));
    }

    #[test]
    fn projection_output_is_sorted() {
        let issue = make_issue_with_def(&["z_module", "a_module", "m_module"]);
        let manifest = SliceProjection::project("alice", &issue).unwrap();
        let mut expected = manifest.allowed_paths.clone();
        expected.sort();
        assert_eq!(manifest.allowed_paths, expected);
    }

    #[test]
    fn projection_fails_without_slice_definition() {
        let issue = make_issue_without_def();
        assert!(SliceProjection::project("alice", &issue).is_err());
    }

    #[test]
    fn manifest_does_not_leak_forbidden_names_in_metadata() {
        let issue = make_issue_with_def(&["src"]);
        let manifest = SliceProjection::project("alice", &issue).unwrap();
        // Forbidden paths must not appear in the allowed_paths list.
        assert!(manifest.allowed_paths.iter().all(|p| !p.contains("secret")));
        assert!(
            manifest
                .allowed_paths
                .iter()
                .all(|p| !p.contains("passwords"))
        );
        // Only "src" should be present.
        assert_eq!(manifest.allowed_paths, vec!["src".to_string()]);
    }
}
