// projects/products/unstable/platform_ide/src/client/platform_client.rs
use serde::{Deserialize, Serialize};

use crate::auth::Session;
use crate::errors::IdeError;
use crate::issues::IssueSummary;
use crate::offline::OfflinePolicy;
use crate::slices::SliceManifest;
use crate::verification::{FindingSeverity, RawFinding, VerificationResultView};

/// The response envelope returned by all platform API endpoints.
#[derive(Debug, Deserialize)]
struct ApiEnvelope<T> {
    ok: bool,
    data: Option<T>,
    error: Option<ApiErrorPayload>,
}

#[derive(Debug, Deserialize)]
struct ApiErrorPayload {
    code: String,
    #[allow(dead_code)]
    message: String,
}

/// Request body for the platform token issuance endpoint.
#[derive(Debug, Serialize)]
struct IssueTokenRequest {
    subject: String,
    repo_id: Option<String>,
    permission: String,
    expires_at: Option<u64>,
}

/// Response body from the token issuance endpoint.
#[derive(Debug, Deserialize)]
struct IssueTokenResponse {
    token: String,
}

/// Response body from the list-repos endpoint.
#[derive(Debug, Deserialize)]
struct RepoSummary {
    id: String,
    name: String,
    description: Option<String>,
    created_at: u64,
}

/// Response body from the verify endpoint.
#[derive(Debug, Deserialize)]
struct VerifySummary {
    healthy: bool,
    #[serde(default)]
    report: RawReport,
}

#[derive(Debug, Default, Deserialize)]
struct RawReport {
    #[serde(default)]
    issues: Vec<RawIssueItem>,
}

#[derive(Debug, Deserialize)]
struct RawIssueItem {
    #[serde(default)]
    description: String,
    #[serde(default)]
    path: Option<String>,
}

/// Request body for creating a commit (submitting a change set).
#[derive(Debug, Serialize)]
struct CreateCommitRequest {
    author: String,
    message: String,
    timestamp: u64,
    files: Vec<CommitFile>,
}

#[derive(Debug, Serialize)]
struct CommitFile {
    path: String,
    #[serde(rename = "content_hex")]
    content: String,
}

/// Response body from the create-commit endpoint.
#[derive(Debug, Deserialize)]
struct CommitResponse {
    #[allow(dead_code)]
    commit_id: String,
}

/// The result of submitting a change set.
#[derive(Debug)]
pub struct SubmitResult {
    /// The commit identifier assigned by the platform.
    pub commit_id: String,
}

/// An HTTP client for the platform-versioning backend.
///
/// All methods communicate solely via the platform's public HTTP API
/// (`/v1/…`). No direct access to the platform's internal data structures
/// is performed.
pub struct PlatformClient {
    base_url: String,
    http: reqwest::Client,
}

impl PlatformClient {
    /// Creates a new client targeting `base_url` (e.g. `"http://127.0.0.1:8080"`).
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            http: reqwest::Client::new(),
        }
    }

    /// Authenticates with the platform using the bootstrap secret and returns
    /// a [`Session`] for the given `subject`.
    ///
    /// This is used for the initial IDE setup. In a production flow a user
    /// would obtain a token through a web-based auth flow or existing SSO.
    pub async fn authenticate(
        &self,
        subject: impl Into<String>,
        bootstrap_secret: impl Into<String>,
    ) -> Result<Session, IdeError> {
        let subject = subject.into();
        let url = format!("{}/v1/auth/issue", self.base_url);
        let body = IssueTokenRequest {
            subject: subject.clone(),
            repo_id: None,
            permission: "read".to_string(),
            expires_at: None,
        };

        let resp = self
            .http
            .post(&url)
            .header("X-Bootstrap-Secret", bootstrap_secret.into())
            .json(&body)
            .send()
            .await?;

        let envelope: ApiEnvelope<IssueTokenResponse> = resp.json().await?;

        if !envelope.ok {
            let code = envelope
                .error
                .map(|e| e.code)
                .unwrap_or_else(|| "UNKNOWN".to_string());
            // Do not log the bootstrap secret or any credential material.
            return Err(IdeError::ApiError { code });
        }

        let data = envelope.data.ok_or(IdeError::UnexpectedResponse)?;
        Ok(Session::new(data.token, subject))
    }

    /// Lists the issues (repositories) visible to the authenticated session.
    pub async fn list_issues(&self, session: &Session) -> Result<Vec<IssueSummary>, IdeError> {
        let url = format!("{}/v1/repos", self.base_url);
        let resp = self
            .http
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", session.bearer_token()),
            )
            .send()
            .await?;

        let envelope: ApiEnvelope<Vec<RepoSummary>> = resp.json().await?;

        if !envelope.ok {
            let code = envelope
                .error
                .map(|e| e.code)
                .unwrap_or_else(|| "UNKNOWN".to_string());
            return Err(IdeError::ApiError { code });
        }

        let repos = envelope.data.ok_or(IdeError::UnexpectedResponse)?;
        Ok(repos
            .into_iter()
            .map(|r| IssueSummary {
                id: r.id,
                name: r.name,
                description: r.description,
                created_at: r.created_at,
            })
            .collect())
    }

    /// Loads a [`SliceManifest`] for a given issue by checking out the
    /// latest commit's file tree from the platform.
    ///
    /// The manifest is the authoritative set of paths the user may access.
    pub async fn load_slice(
        &self,
        session: &Session,
        issue_id: &str,
    ) -> Result<SliceManifest, IdeError> {
        // Retrieve the list of refs to find the HEAD commit.
        let url = format!("{}/v1/repos/{}/refs", self.base_url, issue_id);
        let resp = self
            .http
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", session.bearer_token()),
            )
            .send()
            .await?;

        let envelope: ApiEnvelope<Vec<serde_json::Value>> = resp.json().await?;

        if !envelope.ok {
            let code = envelope
                .error
                .map(|e| e.code)
                .unwrap_or_else(|| "UNKNOWN".to_string());
            return Err(IdeError::ApiError { code });
        }

        let refs = envelope.data.ok_or(IdeError::UnexpectedResponse)?;
        // Find the HEAD/main commit id from the refs list.
        let head_commit = refs
            .iter()
            .find_map(|r| r.get("target").and_then(|t| t.as_str()).map(str::to_string))
            .ok_or(IdeError::NoSliceLoaded)?;

        // Fetch the commit to get the tree of allowed files.
        let commit_url = format!(
            "{}/v1/repos/{}/commits/{}",
            self.base_url, issue_id, head_commit
        );
        let commit_resp = self
            .http
            .get(&commit_url)
            .header(
                "Authorization",
                format!("Bearer {}", session.bearer_token()),
            )
            .send()
            .await?;

        let commit_envelope: ApiEnvelope<serde_json::Value> = commit_resp.json().await?;

        if !commit_envelope.ok {
            let code = commit_envelope
                .error
                .map(|e| e.code)
                .unwrap_or_else(|| "UNKNOWN".to_string());
            return Err(IdeError::ApiError { code });
        }

        let commit = commit_envelope.data.ok_or(IdeError::UnexpectedResponse)?;

        // Extract the list of file paths from the commit's index.
        let paths: Vec<String> = commit
            .get("index")
            .and_then(|i| i.get("entries"))
            .and_then(|e| e.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|e| e.get("path").and_then(|p| p.as_str()).map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();

        Ok(SliceManifest::new(issue_id, head_commit, paths))
    }

    /// Reads the raw content of an allowed file from the platform.
    pub async fn read_file(
        &self,
        session: &Session,
        issue_id: &str,
        manifest: &SliceManifest,
        raw_path: &str,
    ) -> Result<Vec<u8>, IdeError> {
        // Validate path against the manifest before making any request.
        let allowed = manifest.allow(raw_path)?;

        let url = format!(
            "{}/v1/repos/{}/commits/{}/file/{}",
            self.base_url,
            issue_id,
            manifest.base_commit,
            allowed.as_str()
        );

        let resp = self
            .http
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", session.bearer_token()),
            )
            .send()
            .await?;

        if resp.status() == 403 || resp.status() == 401 {
            return Err(IdeError::PermissionDenied);
        }

        Ok(resp.bytes().await?.to_vec())
    }

    /// Submits a change set for `issue_id` by creating a new commit on the
    /// platform.
    pub async fn submit_changes(
        &self,
        session: &Session,
        issue_id: &str,
        change_set: &crate::changes::ChangeSet,
        message: impl Into<String>,
    ) -> Result<SubmitResult, IdeError> {
        change_set.validate()?;

        let files: Vec<CommitFile> = change_set
            .entries()
            .iter()
            .map(|e| CommitFile {
                path: e.path.as_str().to_string(),
                content: e.content.iter().map(|b| format!("{b:02x}")).collect(),
            })
            .collect();

        let url = format!("{}/v1/repos/{}/commits", self.base_url, issue_id);
        let body = CreateCommitRequest {
            author: session.subject.clone(),
            message: message.into(),
            timestamp: now_secs(),
            files,
        };

        let resp = self
            .http
            .post(&url)
            .header(
                "Authorization",
                format!("Bearer {}", session.bearer_token()),
            )
            .json(&body)
            .send()
            .await?;

        let envelope: ApiEnvelope<serde_json::Value> = resp.json().await?;

        if !envelope.ok {
            let code = envelope
                .error
                .map(|e| e.code)
                .unwrap_or_else(|| "UNKNOWN".to_string());
            return Err(IdeError::ApiError { code });
        }

        let data = envelope.data.ok_or(IdeError::UnexpectedResponse)?;
        let commit_id = data
            .get("commit_id")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        Ok(SubmitResult { commit_id })
    }

    /// Requests a verification run for `issue_id` and returns a filtered
    /// [`VerificationResultView`].
    pub async fn request_verification(
        &self,
        session: &Session,
        issue_id: &str,
        manifest: &SliceManifest,
    ) -> Result<VerificationResultView, IdeError> {
        let url = format!("{}/v1/verify/{}", self.base_url, issue_id);
        let resp = self
            .http
            .post(&url)
            .header(
                "Authorization",
                format!("Bearer {}", session.bearer_token()),
            )
            .send()
            .await?;

        let envelope: ApiEnvelope<VerifySummary> = resp.json().await?;

        if !envelope.ok {
            let code = envelope
                .error
                .map(|e| e.code)
                .unwrap_or_else(|| "UNKNOWN".to_string());
            return Err(IdeError::ApiError { code });
        }

        let summary = envelope.data.ok_or(IdeError::UnexpectedResponse)?;

        // Convert raw platform issues to RawFindings and filter through manifest.
        let raw_findings: Vec<RawFinding> = summary
            .report
            .issues
            .into_iter()
            .map(|i| RawFinding {
                severity: FindingSeverity::Error,
                summary: i.description,
                path: i.path,
                line: None,
            })
            .collect();

        Ok(VerificationResultView::from_raw(
            summary.healthy,
            raw_findings,
            manifest,
        ))
    }

    /// Retrieves the offline mode policy from the platform.
    ///
    /// Returns a disabled policy if the platform does not advertise one,
    /// preserving the secure default.
    pub async fn get_offline_policy(&self, session: &Session) -> OfflinePolicy {
        // The platform does not yet have a dedicated offline-policy endpoint.
        // This is policy plumbing for the MVP; offline mode is disabled unless
        // the platform explicitly signals approval.
        tracing::debug!(
            subject = %session.subject,
            "offline policy requested; defaulting to disabled (no platform endpoint yet)"
        );
        OfflinePolicy::disabled()
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
