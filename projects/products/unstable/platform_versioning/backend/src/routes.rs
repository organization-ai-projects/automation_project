use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use crate::auth::{
    AuditEntry, AuditOutcome, AuthorizationService, Permission, PermissionGrant, TokenClaims,
};
use crate::checkouts::{Checkout, CheckoutPolicy, Materialized};
use crate::diffs::Diff;
use crate::errors::PvError;
use crate::history::HistoryWalker;
use crate::http::{ApiError, ApiVersion, ResponseEnvelope};
use crate::ids::{CommitId, RefId, RepoId};
use crate::indexes::Index;
use crate::issues::{Issue, IssueId, IssueStore, IssueVisibility};
use crate::merges::{Merge, MergeResult};
use crate::objects::{Blob, HashDigest, Object, ObjectKind};
use crate::pipeline::CommitBuilder;
use crate::refs_store::{HeadState, RefKind, RefTarget};
use crate::repos::{RepoMetadata, RepoStore};
use crate::slices::{SliceDefinition, SliceManifest, SliceProjection};
use crate::sync::{FetchRequest, Negotiation, RefUpdatePolicy, UploadRequest};
use crate::verify::{SliceFeedback, Verification};

#[derive(Clone)]
struct AppState {
    repo_store: Arc<RepoStore>,
    auth_svc: Arc<AuthorizationService>,
    issue_store: Arc<IssueStore>,
    bootstrap_secret: Option<String>,
}

/// Builds the Axum router for the platform-versioning API.
///
/// All routes are mounted under `/v1/`.
pub fn build_router(
    repo_store: Arc<RepoStore>,
    auth_svc: Arc<AuthorizationService>,
    issue_store: Arc<IssueStore>,
    bootstrap_secret: Option<String>,
) -> Router {
    let state = AppState {
        repo_store,
        auth_svc,
        issue_store,
        bootstrap_secret,
    };

    Router::new()
        .nest(ApiVersion::V1.path_prefix(), v1_routes())
        .with_state(state)
}

fn v1_routes() -> Router<AppState> {
    Router::new()
        .route("/auth/issue", post(issue_token))
        .route("/repos", get(list_repos).post(create_repo))
        .route("/repos/{repo_id}", get(get_repo))
        .route("/repos/{repo_id}/metadata", post(update_repo_metadata))
        .route("/repos/{repo_id}/refs", get(list_refs))
        .route("/repos/{repo_id}/commits", post(create_commit))
        .route("/repos/{repo_id}/commits/{commit_id}", get(get_commit))
        .route("/repos/{repo_id}/history/{commit_id}", get(get_history))
        .route("/repos/{repo_id}/diff", post(compute_diff))
        .route("/repos/{repo_id}/merge", post(merge))
        .route("/repos/{repo_id}/upload", post(upload))
        .route("/repos/{repo_id}/fetch", post(fetch))
        .route(
            "/repos/{repo_id}/checkout/{commit_id}",
            post(checkout_commit),
        )
        .route("/verify/{repo_id}", post(verify_repo))
        .route("/issues", get(list_issues).post(create_issue))
        .route("/issues/{issue_id}", get(get_issue))
        .route("/issues/{issue_id}/assign", post(assign_issue))
        .route("/issues/{issue_id}/share", post(share_issue))
        .route(
            "/issues/{issue_id}/slice",
            get(get_slice_manifest).post(set_slice_definition),
        )
        .route(
            "/verify/{repo_id}/slice/{issue_id}",
            post(verify_with_slice),
        )
}

#[derive(Debug, Deserialize)]
struct CreateRepoRequest {
    id: String,
    name: String,
    description: Option<String>,
}

#[derive(Debug, Serialize)]
struct RepoSummary {
    id: String,
    name: String,
    description: Option<String>,
    created_at: u64,
    updated_at: u64,
}

#[derive(Debug, Deserialize)]
struct UpdateRepoMetadataRequest {
    name: Option<String>,
    description: Option<Option<String>>,
}

#[derive(Debug, Serialize)]
struct RefView {
    full_name: String,
    short_name: String,
    kind: RefKind,
    commit_id: String,
    stable_ref_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CreateCommitFile {
    path: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct CreateCommitRequest {
    author: String,
    message: String,
    timestamp_secs: Option<u64>,
    extra_parent: Option<String>,
    files: Vec<CreateCommitFile>,
}

#[derive(Debug, Deserialize)]
struct DiffRequest {
    from: String,
    to: String,
}

#[derive(Debug, Deserialize)]
struct MergeRequest {
    ours: String,
    theirs: String,
    author: String,
    message: String,
    timestamp_secs: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct CheckoutRequest {
    destination: Option<String>,
    policy: Option<String>,
}

#[derive(Debug, Serialize)]
struct CommitView {
    kind: ObjectKind,
    commit: crate::objects::Commit,
    id_raw_len: usize,
}

#[derive(Debug, Serialize)]
struct UploadSummary {
    objects_written: usize,
    refs_updated: usize,
}

#[derive(Debug, Serialize)]
struct FetchSummary {
    object_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct IssueTokenRequest {
    subject: String,
    repo_id: Option<String>,
    permission: Permission,
    expires_at: Option<u64>,
}

#[derive(Debug, Serialize)]
struct IssueTokenResponse {
    token: String,
}

#[derive(Debug, Serialize)]
struct VerifySummary {
    healthy: bool,
    report: crate::verify::IntegrityReport,
}

// ── Issue request/response types ────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct CreateIssueRequest {
    id: String,
    repo_id: Option<String>,
    title: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AssignUserRequest {
    subject: String,
}

#[derive(Debug, Deserialize)]
struct SetSliceDefinitionRequest {
    paths: Vec<String>,
}

// ── Shared helpers ───────────────────────────────────────────────────────────

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn bearer_token(headers: &HeaderMap) -> Option<String> {
    let raw = headers.get("authorization")?.to_str().ok()?;
    raw.strip_prefix("Bearer ").map(|s| s.trim().to_string())
}

fn bootstrap_secret(headers: &HeaderMap) -> Option<String> {
    headers
        .get("x-pv-bootstrap-secret")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn error_response<T>(err: PvError) -> (StatusCode, Json<ResponseEnvelope<T>>) {
    let status = StatusCode::from_u16(crate::http::api_error::http_status_for(&err))
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    (status, Json(ResponseEnvelope::err(ApiError::from(err))))
}

fn sanitize_checkout_subdir(input: &str) -> Result<PathBuf, PvError> {
    let path = PathBuf::from(input);
    let mut clean = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::Normal(part) => clean.push(part),
            _ => {
                return Err(PvError::UnsafePath(format!(
                    "invalid checkout destination component in '{input}'"
                )));
            }
        }
    }
    if clean.as_os_str().is_empty() {
        return Err(PvError::UnsafePath(
            "checkout destination must not be empty".to_string(),
        ));
    }
    Ok(clean)
}

/// Delegates permission checking to the centralized [`AuthorizationService`].
fn require_permission(
    state: &AppState,
    headers: &HeaderMap,
    repo_id: Option<&RepoId>,
    permission: Permission,
    action: &str,
) -> Result<TokenClaims, PvError> {
    state
        .auth_svc
        .require_permission(headers, repo_id, permission, action)
}

fn stage_tree_into_index(
    index: &mut Index,
    store: &crate::objects::ObjectStore,
    tree_id: &crate::ids::ObjectId,
    prefix: &str,
) -> Result<(), PvError> {
    let tree = match store.read(tree_id)? {
        Object::Tree(t) => t,
        _ => {
            return Err(PvError::CorruptObject(format!(
                "object {tree_id} expected to be tree"
            )));
        }
    };

    for entry in tree.entries {
        let path = if prefix.is_empty() {
            entry.name.clone()
        } else {
            format!("{prefix}/{}", entry.name)
        };
        match entry.kind {
            crate::objects::TreeEntryKind::Blob => {
                let safe = path.parse()?;
                let _ = index.add(safe, crate::ids::BlobId::from(entry.id));
            }
            crate::objects::TreeEntryKind::Tree => {
                stage_tree_into_index(index, store, &entry.id, &path)?
            }
        }
    }
    Ok(())
}

fn seed_index_from_head(index: &mut Index, repo: &crate::repos::Repo) -> Result<(), PvError> {
    let head = repo.refs.read_head()?;
    let commit_id = match head {
        HeadState::Branch(branch) | HeadState::Unborn(branch) => {
            if let Ok(target) = repo.refs.read_ref(&branch) {
                Some(target.commit_id().clone())
            } else {
                None
            }
        }
        HeadState::Detached(raw) => raw.parse::<CommitId>().ok(),
    };

    if let Some(commit_id) = commit_id {
        let commit = match repo.objects.read(commit_id.as_object_id())? {
            Object::Commit(c) => c,
            _ => {
                return Err(PvError::CorruptObject(format!(
                    "object {commit_id} is not a commit"
                )));
            }
        };
        stage_tree_into_index(index, &repo.objects, commit.tree_id.as_object_id(), "")?;
    }

    Ok(())
}

async fn list_repos(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> (StatusCode, Json<ResponseEnvelope<Vec<RepoSummary>>>) {
    if let Err(err) = require_permission(&state, &headers, None, Permission::Read, "repo.list") {
        return error_response(err);
    }

    let out = (|| -> Result<Vec<RepoSummary>, PvError> {
        let ids = state.repo_store.list()?;
        let mut repos = Vec::with_capacity(ids.len());
        for id in ids {
            let repo = state.repo_store.get(&id)?;
            repos.push(RepoSummary {
                id: repo.metadata.id.to_string(),
                name: repo.metadata.name,
                description: repo.metadata.description,
                created_at: repo.metadata.created_at,
                updated_at: repo.metadata.updated_at,
            });
        }

        Ok(repos)
    })();

    match out {
        Ok(repos) => (StatusCode::OK, Json(ResponseEnvelope::ok(repos))),
        Err(err) => error_response(err),
    }
}

async fn issue_token(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<IssueTokenRequest>,
) -> (StatusCode, Json<ResponseEnvelope<IssueTokenResponse>>) {
    if bearer_token(&headers).is_some() {
        if let Err(err) =
            require_permission(&state, &headers, None, Permission::Admin, "auth.issue")
        {
            return error_response(err);
        }
    } else {
        let provided = bootstrap_secret(&headers);
        let configured = state.bootstrap_secret.clone();
        match (provided, configured) {
            (Some(p), Some(c)) if p == c => {}
            _ => {
                return error_response(PvError::AuthRequired(
                    "missing admin bearer token or valid bootstrap secret".to_string(),
                ));
            }
        }
    }

    let repo_scope = match req.repo_id {
        Some(raw) => match raw.parse::<RepoId>() {
            Ok(id) => Some(id),
            Err(err) => return error_response(err),
        },
        None => None,
    };

    let claims = TokenClaims {
        subject: req.subject,
        grants: vec![PermissionGrant {
            repo_id: repo_scope,
            permission: req.permission,
        }],
        expires_at: req.expires_at,
        path_grants: vec![],
    };

    match state.auth_svc.issue_token(&claims) {
        Ok(token) => (
            StatusCode::OK,
            Json(ResponseEnvelope::ok(IssueTokenResponse {
                token: token.as_str().to_string(),
            })),
        ),
        Err(err) => error_response(err),
    }
}

async fn create_repo(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<CreateRepoRequest>,
) -> (StatusCode, Json<ResponseEnvelope<RepoMetadata>>) {
    let repo_id = match req.id.parse::<RepoId>() {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };

    if let Err(err) = require_permission(
        &state,
        &headers,
        Some(&repo_id),
        Permission::Admin,
        "repo.create",
    ) {
        return error_response(err);
    }

    let out = state
        .repo_store
        .create(repo_id, req.name, req.description, now_secs())
        .map(|r| r.metadata);

    match out {
        Ok(meta) => (StatusCode::CREATED, Json(ResponseEnvelope::ok(meta))),
        Err(err) => error_response(err),
    }
}

async fn get_repo(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(repo_id_raw): Path<String>,
) -> (StatusCode, Json<ResponseEnvelope<RepoSummary>>) {
    let repo_id = match repo_id_raw.parse::<RepoId>() {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };

    if let Err(err) = require_permission(
        &state,
        &headers,
        Some(&repo_id),
        Permission::Read,
        "repo.get",
    ) {
        return error_response(err);
    }

    match state.repo_store.get(&repo_id) {
        Ok(repo) => {
            let _ = repo.id();
            (
                StatusCode::OK,
                Json(ResponseEnvelope::ok(RepoSummary {
                    id: repo.metadata.id.to_string(),
                    name: repo.metadata.name,
                    description: repo.metadata.description,
                    created_at: repo.metadata.created_at,
                    updated_at: repo.metadata.updated_at,
                })),
            )
        }
        Err(err) => error_response(err),
    }
}

async fn update_repo_metadata(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(repo_id_raw): Path<String>,
    Json(req): Json<UpdateRepoMetadataRequest>,
) -> (StatusCode, Json<ResponseEnvelope<RepoMetadata>>) {
    let repo_id = match repo_id_raw.parse::<RepoId>() {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };

    if let Err(err) = require_permission(
        &state,
        &headers,
        Some(&repo_id),
        Permission::Admin,
        "repo.update_metadata",
    ) {
        return error_response(err);
    }

    match state
        .repo_store
        .update_metadata(&repo_id, req.name, req.description, now_secs())
    {
        Ok(meta) => (StatusCode::OK, Json(ResponseEnvelope::ok(meta))),
        Err(err) => error_response(err),
    }
}

async fn list_refs(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(repo_id_raw): Path<String>,
) -> (StatusCode, Json<ResponseEnvelope<Vec<RefView>>>) {
    let repo_id = match repo_id_raw.parse::<RepoId>() {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };

    if let Err(err) = require_permission(
        &state,
        &headers,
        Some(&repo_id),
        Permission::Read,
        "ref.list",
    ) {
        return error_response(err);
    }

    let out = (|| -> Result<Vec<RefView>, PvError> {
        let repo = state.repo_store.get(&repo_id)?;
        let refs = repo.refs.list_refs()?;
        let mut out = Vec::with_capacity(refs.len());
        for (name, target) in refs {
            let stable_ref_id = name
                .as_str()
                .parse::<RefId>()
                .ok()
                .map(|r| r.as_str().to_string());
            out.push(RefView {
                full_name: name.as_str().to_string(),
                short_name: name.short_name().to_string(),
                kind: name.kind(),
                commit_id: target.commit_id().to_string(),
                stable_ref_id,
            });
        }
        Ok(out)
    })();

    match out {
        Ok(data) => (StatusCode::OK, Json(ResponseEnvelope::ok(data))),
        Err(err) => error_response(err),
    }
}

async fn create_commit(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(repo_id_raw): Path<String>,
    Json(req): Json<CreateCommitRequest>,
) -> (
    StatusCode,
    Json<ResponseEnvelope<crate::pipeline::CommitResult>>,
) {
    let repo_id = match repo_id_raw.parse::<RepoId>() {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };

    if let Err(err) = require_permission(
        &state,
        &headers,
        Some(&repo_id),
        Permission::Write,
        "commit.create",
    ) {
        return error_response(err);
    }

    let out = (|| -> Result<crate::pipeline::CommitResult, PvError> {
        let repo = state.repo_store.get(&repo_id)?;
        let mut index = Index::new();
        seed_index_from_head(&mut index, &repo)?;

        for file in req.files {
            let path: crate::indexes::SafePath = file.path.parse()?;
            let _ = index.remove(&path);
            let blob = Blob::from_bytes(file.content.into_bytes());
            let _ = index.add(path.clone(), blob.id.clone());
            let _ = index.get(&path);
            repo.objects.write(Object::Blob(blob))?;
        }
        let _staged_files = index.len();

        let mut builder = CommitBuilder::new(
            req.author,
            req.message,
            req.timestamp_secs.unwrap_or_else(now_secs),
        );
        if let Some(extra_parent) = req.extra_parent {
            builder = builder.with_parent(extra_parent.parse()?);
        }

        builder.commit(&index, &repo.objects, &repo.refs)
    })();

    match out {
        Ok(commit) => (StatusCode::CREATED, Json(ResponseEnvelope::ok(commit))),
        Err(err) => error_response(err),
    }
}

async fn get_commit(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((repo_id_raw, commit_id_raw)): Path<(String, String)>,
) -> (StatusCode, Json<ResponseEnvelope<CommitView>>) {
    let repo_id = match repo_id_raw.parse::<RepoId>() {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };
    let commit_id = match commit_id_raw.parse::<CommitId>() {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };

    if let Err(err) = require_permission(
        &state,
        &headers,
        Some(&repo_id),
        Permission::Read,
        "commit.get",
    ) {
        return error_response(err);
    }

    let out = (|| -> Result<CommitView, PvError> {
        let repo = state.repo_store.get(&repo_id)?;
        let obj = repo.objects.read(commit_id.as_object_id())?;
        let kind = obj.kind();
        match obj {
            Object::Commit(commit) => {
                let id_raw_len = commit.id.as_object_id().to_bytes().len();
                Ok(CommitView {
                    kind,
                    commit,
                    id_raw_len,
                })
            }
            _ => Err(PvError::CommitNotFound(commit_id.to_string())),
        }
    })();

    match out {
        Ok(view) => (StatusCode::OK, Json(ResponseEnvelope::ok(view))),
        Err(err) => error_response(err),
    }
}

async fn get_history(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((repo_id_raw, commit_id_raw)): Path<(String, String)>,
) -> (
    StatusCode,
    Json<ResponseEnvelope<crate::history::HistoryPage>>,
) {
    let repo_id = match repo_id_raw.parse::<RepoId>() {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };
    let commit_id = match commit_id_raw.parse::<CommitId>() {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };

    if let Err(err) = require_permission(
        &state,
        &headers,
        Some(&repo_id),
        Permission::Read,
        "history.get",
    ) {
        return error_response(err);
    }

    let out = (|| -> Result<crate::history::HistoryPage, PvError> {
        let repo = state.repo_store.get(&repo_id)?;
        let walker = HistoryWalker::new(&repo.objects);
        walker.page(&commit_id, 100)
    })();

    match out {
        Ok(history) => (StatusCode::OK, Json(ResponseEnvelope::ok(history))),
        Err(err) => error_response(err),
    }
}

async fn compute_diff(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(repo_id_raw): Path<String>,
    Json(req): Json<DiffRequest>,
) -> (StatusCode, Json<ResponseEnvelope<Diff>>) {
    let repo_id = match repo_id_raw.parse::<RepoId>() {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };
    let from = match req.from.parse::<CommitId>() {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };
    let to = match req.to.parse::<CommitId>() {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };

    if let Err(err) = require_permission(
        &state,
        &headers,
        Some(&repo_id),
        Permission::Read,
        "diff.compute",
    ) {
        return error_response(err);
    }

    let out = (|| -> Result<Diff, PvError> {
        let repo = state.repo_store.get(&repo_id)?;
        Diff::compute(&from, &to, &repo.objects)
    })();

    match out {
        Ok(diff) => (StatusCode::OK, Json(ResponseEnvelope::ok(diff))),
        Err(err) => error_response(err),
    }
}

async fn merge(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(repo_id_raw): Path<String>,
    Json(req): Json<MergeRequest>,
) -> (StatusCode, Json<ResponseEnvelope<MergeResult>>) {
    let repo_id = match repo_id_raw.parse::<RepoId>() {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };
    let ours = match req.ours.parse::<CommitId>() {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };
    let theirs = match req.theirs.parse::<CommitId>() {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };

    if let Err(err) = require_permission(
        &state,
        &headers,
        Some(&repo_id),
        Permission::Write,
        "merge.perform",
    ) {
        return error_response(err);
    }
    if ours == theirs {
        return error_response(PvError::MergeConflict(
            "cannot merge a commit into itself".to_string(),
        ));
    }

    let out = (|| -> Result<MergeResult, PvError> {
        let repo = state.repo_store.get(&repo_id)?;
        Merge::perform(
            &ours,
            &theirs,
            &req.author,
            &req.message,
            req.timestamp_secs.unwrap_or_else(now_secs),
            &repo.objects,
            &repo.refs,
        )
    })();

    match out {
        Ok(result) => {
            let status = if matches!(result, MergeResult::Conflicted { .. }) {
                StatusCode::CONFLICT
            } else {
                StatusCode::OK
            };
            (status, Json(ResponseEnvelope::ok(result)))
        }
        Err(err) => error_response(err),
    }
}

async fn upload(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(repo_id_raw): Path<String>,
    Json(req): Json<UploadRequest>,
) -> (StatusCode, Json<ResponseEnvelope<UploadSummary>>) {
    let repo_id = match repo_id_raw.parse::<RepoId>() {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };

    if let Err(err) = require_permission(
        &state,
        &headers,
        Some(&repo_id),
        Permission::Write,
        "upload",
    ) {
        return error_response(err);
    }

    let out = (|| -> Result<UploadSummary, PvError> {
        let repo = state.repo_store.get(&repo_id)?;
        let mut written = 0usize;
        for obj in req.objects {
            if !obj.verify() {
                let object_id = match &obj {
                    Object::Blob(b) => b.id.as_object_id().to_string(),
                    Object::Tree(t) => t.id.as_object_id().to_string(),
                    Object::Commit(c) => c.id.as_object_id().to_string(),
                };
                return Err(PvError::CorruptObject(format!(
                    "uploaded object {object_id} failed integrity verification"
                )));
            }
            if let Object::Blob(blob) = &obj {
                let digest = HashDigest::compute(&blob.content);
                if blob.id.as_object_id().to_bytes() != digest {
                    return Err(PvError::CorruptObject(format!(
                        "blob {} digest mismatch",
                        blob.id
                    )));
                }
            }
            let _ = repo.objects.write(obj)?;
            written += 1;
        }

        let mut refs_updated = 0usize;
        for update in req.ref_updates {
            if update.policy == RefUpdatePolicy::Force {
                let _ = require_permission(
                    &state,
                    &headers,
                    Some(&repo_id),
                    Permission::Admin,
                    "ref.force_update",
                )?;
            }
            let target = RefTarget::Commit(update.new_commit);
            repo.refs.write_ref(
                &update.ref_name,
                &target,
                update.policy == RefUpdatePolicy::Force,
                Some(&repo.objects),
            )?;
            refs_updated += 1;
        }

        Ok(UploadSummary {
            objects_written: written,
            refs_updated,
        })
    })();

    match out {
        Ok(summary) => (StatusCode::OK, Json(ResponseEnvelope::ok(summary))),
        Err(err) => error_response(err),
    }
}

async fn fetch(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(repo_id_raw): Path<String>,
    Json(req): Json<FetchRequest>,
) -> (StatusCode, Json<ResponseEnvelope<FetchSummary>>) {
    let repo_id = match repo_id_raw.parse::<RepoId>() {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };

    if let Err(err) =
        require_permission(&state, &headers, Some(&repo_id), Permission::Read, "fetch")
    {
        return error_response(err);
    }

    let out = (|| -> Result<FetchSummary, PvError> {
        let repo = state.repo_store.get(&repo_id)?;
        let objects = Negotiation::collect(&req, &repo.objects, &repo.refs)?;
        let object_ids = objects
            .iter()
            .map(|o| match o {
                Object::Blob(b) => b.id.as_str().to_string(),
                Object::Tree(t) => t.id.as_object_id().to_string(),
                Object::Commit(c) => c.id.as_object_id().to_string(),
            })
            .collect();
        Ok(FetchSummary { object_ids })
    })();

    match out {
        Ok(summary) => (StatusCode::OK, Json(ResponseEnvelope::ok(summary))),
        Err(err) => error_response(err),
    }
}

async fn checkout_commit(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((repo_id_raw, commit_id_raw)): Path<(String, String)>,
    Json(req): Json<CheckoutRequest>,
) -> (StatusCode, Json<ResponseEnvelope<Materialized>>) {
    let repo_id = match repo_id_raw.parse::<RepoId>() {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };
    let commit_id = match commit_id_raw.parse::<CommitId>() {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };

    if let Err(err) = require_permission(
        &state,
        &headers,
        Some(&repo_id),
        Permission::Write,
        "checkout",
    ) {
        return error_response(err);
    }

    let out = (|| -> Result<Materialized, PvError> {
        let repo = state.repo_store.get(&repo_id)?;
        let checkout_root = state.repo_store.checkout_root(&repo_id);
        std::fs::create_dir_all(&checkout_root)
            .map_err(|e| PvError::AtomicWriteFailed(format!("create checkout root: {e}")))?;
        let dest = match req.destination {
            Some(raw) => checkout_root.join(sanitize_checkout_subdir(&raw)?),
            None => checkout_root.join(format!("run-{}", now_secs())),
        };
        let policy = match req.policy.as_deref() {
            Some("clean") => CheckoutPolicy::clean(),
            Some("safe") => CheckoutPolicy::safe(),
            _ => CheckoutPolicy::overwrite(),
        };
        Checkout::materialize(&commit_id, &repo.objects, &dest, &policy)
    })();

    match out {
        Ok(done) => (StatusCode::OK, Json(ResponseEnvelope::ok(done))),
        Err(err) => error_response(err),
    }
}

async fn verify_repo(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(repo_id_raw): Path<String>,
) -> (StatusCode, Json<ResponseEnvelope<VerifySummary>>) {
    let repo_id = match repo_id_raw.parse::<RepoId>() {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };

    if let Err(err) =
        require_permission(&state, &headers, Some(&repo_id), Permission::Read, "verify")
    {
        return error_response(err);
    }

    let out = (|| -> Result<VerifySummary, PvError> {
        let repo = state.repo_store.get(&repo_id)?;
        let report = Verification::run(&repo.objects, &repo.refs)?;
        Ok(VerifySummary {
            healthy: report.is_healthy(),
            report,
        })
    })();

    match out {
        Ok(report) => (StatusCode::OK, Json(ResponseEnvelope::ok(report))),
        Err(err) => error_response(err),
    }
}

// ── Issue management endpoints ───────────────────────────────────────────────

async fn create_issue(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<CreateIssueRequest>,
) -> (StatusCode, Json<ResponseEnvelope<Issue>>) {
    if let Err(err) = require_permission(&state, &headers, None, Permission::Admin, "issue.create")
    {
        return error_response(err);
    }

    let out = (|| -> Result<Issue, PvError> {
        let id: IssueId = req
            .id
            .parse()
            .map_err(|_| PvError::InvalidId(format!("invalid issue id '{}'", req.id)))?;
        let repo_id = match req.repo_id {
            Some(raw) => Some(raw.parse::<RepoId>()?),
            None => None,
        };
        let issue = Issue {
            id,
            repo_id,
            title: req.title,
            description: req.description,
            assignees: vec![],
            shared_with: vec![],
            slice_definition: None,
        };
        state.issue_store.create(issue)
    })();

    match out {
        Ok(issue) => (StatusCode::CREATED, Json(ResponseEnvelope::ok(issue))),
        Err(err) => error_response(err),
    }
}

async fn list_issues(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> (StatusCode, Json<ResponseEnvelope<Vec<Issue>>>) {
    let claims = match state.auth_svc.authenticate(&headers, "issue.list") {
        Ok(c) => c,
        Err(err) => return error_response(err),
    };

    // Admin is determined by having Admin permission on the global scope.
    let is_admin = "global"
        .parse::<RepoId>()
        .map(|g| claims.has_permission(&g, Permission::Admin))
        .unwrap_or(false);
    let visibility = IssueVisibility::for_role(is_admin);

    match state.issue_store.list(&claims.subject, visibility) {
        Ok(issues) => (StatusCode::OK, Json(ResponseEnvelope::ok(issues))),
        Err(err) => error_response(err),
    }
}

async fn get_issue(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(issue_id_raw): Path<String>,
) -> (StatusCode, Json<ResponseEnvelope<Issue>>) {
    let claims = match state.auth_svc.authenticate(&headers, "issue.get") {
        Ok(c) => c,
        Err(err) => return error_response(err),
    };

    let out = (|| -> Result<Issue, PvError> {
        let id: IssueId = issue_id_raw
            .parse()
            .map_err(|_| PvError::InvalidId(format!("invalid issue id '{issue_id_raw}'")))?;
        let issue = state.issue_store.get(&id)?;

        let is_admin = issue
            .repo_id
            .as_ref()
            .map(|rid| claims.has_permission(rid, Permission::Admin))
            .unwrap_or_else(|| {
                "global"
                    .parse::<RepoId>()
                    .map(|g| claims.has_permission(&g, Permission::Admin))
                    .unwrap_or(false)
            });

        if !is_admin && !issue.is_visible_to(&claims.subject) {
            state.auth_svc.audit_log().record(AuditEntry {
                timestamp_secs: now_secs(),
                subject: claims.subject.clone(),
                action: "issue.get".to_string(),
                repo_id: issue.repo_id.clone(),
                outcome: AuditOutcome::Denied,
            });
            return Err(PvError::PermissionDenied(
                "issue not visible to user".to_string(),
            ));
        }
        Ok(issue)
    })();

    match out {
        Ok(issue) => (StatusCode::OK, Json(ResponseEnvelope::ok(issue))),
        Err(err) => error_response(err),
    }
}

async fn assign_issue(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(issue_id_raw): Path<String>,
    Json(req): Json<AssignUserRequest>,
) -> (StatusCode, Json<ResponseEnvelope<Issue>>) {
    let claims = match require_permission(&state, &headers, None, Permission::Admin, "issue.assign")
    {
        Ok(c) => c,
        Err(err) => return error_response(err),
    };

    let out = (|| -> Result<Issue, PvError> {
        let id: IssueId = issue_id_raw
            .parse()
            .map_err(|_| PvError::InvalidId(format!("invalid issue id '{issue_id_raw}'")))?;
        let issue = state.issue_store.assign_user(&id, req.subject.clone())?;
        state.auth_svc.record_permission_change(
            &claims.subject,
            &req.subject,
            issue.repo_id.clone(),
        );
        Ok(issue)
    })();

    match out {
        Ok(issue) => (StatusCode::OK, Json(ResponseEnvelope::ok(issue))),
        Err(err) => error_response(err),
    }
}

async fn share_issue(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(issue_id_raw): Path<String>,
    Json(req): Json<AssignUserRequest>,
) -> (StatusCode, Json<ResponseEnvelope<Issue>>) {
    let claims = match require_permission(&state, &headers, None, Permission::Admin, "issue.share")
    {
        Ok(c) => c,
        Err(err) => return error_response(err),
    };

    let out = (|| -> Result<Issue, PvError> {
        let id: IssueId = issue_id_raw
            .parse()
            .map_err(|_| PvError::InvalidId(format!("invalid issue id '{issue_id_raw}'")))?;
        let issue = state.issue_store.share_with(&id, req.subject.clone())?;
        state.auth_svc.record_permission_change(
            &claims.subject,
            &req.subject,
            issue.repo_id.clone(),
        );
        Ok(issue)
    })();

    match out {
        Ok(issue) => (StatusCode::OK, Json(ResponseEnvelope::ok(issue))),
        Err(err) => error_response(err),
    }
}

// ── Slice endpoints ──────────────────────────────────────────────────────────

async fn get_slice_manifest(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(issue_id_raw): Path<String>,
) -> (StatusCode, Json<ResponseEnvelope<SliceManifest>>) {
    let claims = match state.auth_svc.authenticate(&headers, "slice.get") {
        Ok(c) => c,
        Err(err) => return error_response(err),
    };

    let out = (|| -> Result<SliceManifest, PvError> {
        let id: IssueId = issue_id_raw
            .parse()
            .map_err(|_| PvError::InvalidId(format!("invalid issue id '{issue_id_raw}'")))?;
        let issue = state.issue_store.get(&id)?;

        let is_admin = issue
            .repo_id
            .as_ref()
            .map(|rid| claims.has_permission(rid, Permission::Admin))
            .unwrap_or(false);

        if !is_admin && !issue.is_visible_to(&claims.subject) {
            return Err(PvError::PermissionDenied(
                "issue not visible to user".to_string(),
            ));
        }

        let manifest = SliceProjection::project(&claims.subject, &issue)?;
        state
            .auth_svc
            .record_slice_access(&claims.subject, &id.to_string(), issue.repo_id.clone());
        Ok(manifest)
    })();

    match out {
        Ok(manifest) => (StatusCode::OK, Json(ResponseEnvelope::ok(manifest))),
        Err(err) => error_response(err),
    }
}

async fn set_slice_definition(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(issue_id_raw): Path<String>,
    Json(req): Json<SetSliceDefinitionRequest>,
) -> (StatusCode, Json<ResponseEnvelope<Issue>>) {
    let claims = match require_permission(&state, &headers, None, Permission::Admin, "slice.set") {
        Ok(c) => c,
        Err(err) => return error_response(err),
    };

    let out = (|| -> Result<Issue, PvError> {
        let id: IssueId = issue_id_raw
            .parse()
            .map_err(|_| PvError::InvalidId(format!("invalid issue id '{issue_id_raw}'")))?;
        let definition = SliceDefinition::from_paths(req.paths)?;
        let issue = state.issue_store.set_slice_definition(&id, definition)?;
        state.auth_svc.record_slice_created(
            &claims.subject,
            &id.to_string(),
            issue.repo_id.clone(),
        );
        Ok(issue)
    })();

    match out {
        Ok(issue) => (StatusCode::OK, Json(ResponseEnvelope::ok(issue))),
        Err(err) => error_response(err),
    }
}

async fn verify_with_slice(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((repo_id_raw, issue_id_raw)): Path<(String, String)>,
) -> (StatusCode, Json<ResponseEnvelope<SliceFeedback>>) {
    let repo_id = match repo_id_raw.parse::<RepoId>() {
        Ok(id) => id,
        Err(err) => return error_response(err),
    };

    let claims = match require_permission(
        &state,
        &headers,
        Some(&repo_id),
        Permission::Read,
        "verify.slice",
    ) {
        Ok(c) => c,
        Err(err) => return error_response(err),
    };

    let out = (|| -> Result<SliceFeedback, PvError> {
        let id: IssueId = issue_id_raw
            .parse()
            .map_err(|_| PvError::InvalidId(format!("invalid issue id '{issue_id_raw}'")))?;
        let issue = state.issue_store.get(&id)?;

        let is_admin = claims.has_permission(&repo_id, Permission::Admin);
        if !is_admin && !issue.is_visible_to(&claims.subject) {
            return Err(PvError::PermissionDenied(
                "issue not visible to user".to_string(),
            ));
        }

        let manifest = SliceProjection::project(&claims.subject, &issue)?;
        let repo = state.repo_store.get(&repo_id)?;
        let report = Verification::run(&repo.objects, &repo.refs)?;
        let feedback = SliceFeedback::for_manifest(&report, &manifest);
        state
            .auth_svc
            .record_slice_access(&claims.subject, &id.to_string(), Some(repo_id));
        Ok(feedback)
    })();

    match out {
        Ok(feedback) => (StatusCode::OK, Json(ResponseEnvelope::ok(feedback))),
        Err(err) => error_response(err),
    }
}
