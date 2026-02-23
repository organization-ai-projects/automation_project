// projects/products/unstable/platform_ide/src/app.rs
use crate::auth::Session;
use crate::changes::ChangeSet;
use crate::client::PlatformClient;
use crate::diff::LocalDiff;
use crate::editor::FileBuffer;
use crate::errors::IdeError;
use crate::issues::IssueSummary;
use crate::offline::OfflinePolicy;
use crate::slices::{AllowedPath, SliceManifest};
use crate::verification::VerificationResultView;

/// Configuration for the platform IDE.
pub struct IdeConfig {
    /// URL of the platform-versioning backend (e.g. `"http://127.0.0.1:8080"`).
    pub platform_url: String,
}

impl IdeConfig {
    /// Reads configuration from environment variables with sensible defaults.
    pub fn from_env() -> Self {
        let platform_url = std::env::var("PLATFORM_IDE_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
        Self { platform_url }
    }
}

/// The platform IDE application.
///
/// `IdeApp` orchestrates the full IDE workflow: authentication, issue listing,
/// slice loading, editing, diffing, submitting changes, and viewing verification
/// results. All operations enforce least-privilege visibility through the slice
/// manifest.
pub struct IdeApp {
    client: PlatformClient,
    session: Session,
    /// The active issue (repository) identifier, if one has been opened.
    pub active_issue: Option<String>,
    /// The slice manifest for the active issue.
    pub manifest: Option<SliceManifest>,
    /// The offline mode policy as reported by the platform.
    pub offline_policy: OfflinePolicy,
}

impl IdeApp {
    /// Creates a new `IdeApp` with an authenticated session.
    pub fn new(platform_url: impl Into<String>, session: Session) -> Self {
        Self {
            client: PlatformClient::new(platform_url),
            session,
            active_issue: None,
            manifest: None,
            offline_policy: OfflinePolicy::disabled(),
        }
    }

    /// Returns the authenticated subject for the current session.
    pub fn current_user(&self) -> &str {
        &self.session.subject
    }

    /// Lists the issues visible to the current user.
    pub async fn list_issues(&self) -> Result<Vec<IssueSummary>, IdeError> {
        self.client.list_issues(&self.session).await
    }

    /// Opens an issue and loads its slice manifest. After this call, the IDE
    /// is scoped to the files allowed by that issue's manifest.
    pub async fn open_issue(&mut self, issue_id: impl Into<String>) -> Result<(), IdeError> {
        let id = issue_id.into();
        let manifest = self.client.load_slice(&self.session, &id).await?;
        self.active_issue = Some(id);
        self.manifest = Some(manifest);
        self.offline_policy = self.client.get_offline_policy(&self.session).await;
        Ok(())
    }

    /// Returns the slice manifest for the active issue, if one is loaded.
    pub fn slice_manifest(&self) -> Option<&SliceManifest> {
        self.manifest.as_ref()
    }

    /// Validates a path against the active slice manifest and returns an
    /// `AllowedPath` if the path is permitted.
    pub fn allow_path(&self, raw: &str) -> Result<AllowedPath, IdeError> {
        self.manifest
            .as_ref()
            .ok_or(IdeError::NoSliceLoaded)?
            .allow(raw)
    }

    /// Opens a file for editing. The path is validated against the active
    /// slice manifest before any content is fetched from the platform.
    pub async fn open_file(&self, raw_path: &str) -> Result<FileBuffer, IdeError> {
        let issue_id = self.active_issue.as_deref().ok_or(IdeError::NoSliceLoaded)?;
        let manifest = self.manifest.as_ref().ok_or(IdeError::NoSliceLoaded)?;
        let content = self
            .client
            .read_file(&self.session, issue_id, manifest, raw_path)
            .await?;
        let allowed = manifest.allow(raw_path)?;
        Ok(FileBuffer::open(allowed, content))
    }

    /// Computes a local diff for a file buffer.
    pub fn local_diff(buf: &FileBuffer) -> LocalDiff {
        LocalDiff::from_buffer(buf)
    }

    /// Submits a change set for the active issue.
    pub async fn submit_changes(
        &self,
        change_set: &ChangeSet,
        message: impl Into<String>,
    ) -> Result<String, IdeError> {
        let issue_id = self.active_issue.as_deref().ok_or(IdeError::NoSliceLoaded)?;
        let result = self
            .client
            .submit_changes(&self.session, issue_id, change_set, message)
            .await?;
        Ok(result.commit_id)
    }

    /// Requests a verification run for the active issue and returns a filtered
    /// result view.
    pub async fn request_verification(&self) -> Result<VerificationResultView, IdeError> {
        let issue_id = self.active_issue.as_deref().ok_or(IdeError::NoSliceLoaded)?;
        let manifest = self.manifest.as_ref().ok_or(IdeError::NoSliceLoaded)?;
        self.client
            .request_verification(&self.session, issue_id, manifest)
            .await
    }

    /// Returns `true` if the offline mode controls should be displayed.
    ///
    /// Controls are shown only if the platform has admin-approved offline mode.
    pub fn show_offline_controls(&self) -> bool {
        self.offline_policy.is_allowed()
    }
}
