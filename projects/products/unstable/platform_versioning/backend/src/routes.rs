// projects/products/unstable/platform_versioning/backend/src/routes.rs
use std::sync::Arc;

use axum::Router;

use crate::auth::TokenVerifier;
use crate::objects::ObjectStore;
use crate::refs_store::RefStore;
use crate::repos::RepoStore;

/// Builds the Axum router for the platform-versioning API.
///
/// All routes are mounted under `/v1/`.
pub fn build_router(
    _repo_store: Arc<RepoStore>,
    _object_store: Arc<ObjectStore>,
    _ref_store: Arc<RefStore>,
    _token_verifier: Arc<TokenVerifier>,
) -> Router {
    Router::new().nest("/v1", v1_routes())
}

fn v1_routes() -> Router {
    use axum::routing::{get, post};
    Router::new()
        .route(
            "/repos",
            get(handlers::list_repos).post(handlers::create_repo),
        )
        .route("/repos/{repo_id}", get(handlers::get_repo))
        .route("/repos/{repo_id}/refs", get(handlers::list_refs))
        .route(
            "/repos/{repo_id}/commits/{commit_id}",
            get(handlers::get_commit),
        )
        .route(
            "/repos/{repo_id}/history/{commit_id}",
            get(handlers::get_history),
        )
        .route("/repos/{repo_id}/diff", post(handlers::compute_diff))
        .route("/repos/{repo_id}/merge", post(handlers::merge))
        .route("/repos/{repo_id}/upload", post(handlers::upload))
        .route("/repos/{repo_id}/fetch", post(handlers::fetch))
        .route("/verify/{repo_id}", post(handlers::verify_repo))
}

mod handlers {
    use axum::Json;
    use axum::http::StatusCode;

    use crate::http::ResponseEnvelope;

    pub async fn list_repos() -> (StatusCode, Json<ResponseEnvelope<Vec<String>>>) {
        (StatusCode::OK, Json(ResponseEnvelope::ok(vec![])))
    }

    pub async fn create_repo() -> (StatusCode, Json<ResponseEnvelope<String>>) {
        (
            StatusCode::CREATED,
            Json(ResponseEnvelope::ok("created".to_string())),
        )
    }

    pub async fn get_repo() -> (StatusCode, Json<ResponseEnvelope<String>>) {
        (
            StatusCode::OK,
            Json(ResponseEnvelope::ok("repo".to_string())),
        )
    }

    pub async fn list_refs() -> (StatusCode, Json<ResponseEnvelope<Vec<String>>>) {
        (StatusCode::OK, Json(ResponseEnvelope::ok(vec![])))
    }

    pub async fn get_commit() -> (StatusCode, Json<ResponseEnvelope<String>>) {
        (
            StatusCode::OK,
            Json(ResponseEnvelope::ok("commit".to_string())),
        )
    }

    pub async fn get_history() -> (StatusCode, Json<ResponseEnvelope<Vec<String>>>) {
        (StatusCode::OK, Json(ResponseEnvelope::ok(vec![])))
    }

    pub async fn compute_diff() -> (StatusCode, Json<ResponseEnvelope<Vec<String>>>) {
        (StatusCode::OK, Json(ResponseEnvelope::ok(vec![])))
    }

    pub async fn merge() -> (StatusCode, Json<ResponseEnvelope<String>>) {
        (
            StatusCode::OK,
            Json(ResponseEnvelope::ok("merged".to_string())),
        )
    }

    pub async fn upload() -> (StatusCode, Json<ResponseEnvelope<String>>) {
        (
            StatusCode::OK,
            Json(ResponseEnvelope::ok("uploaded".to_string())),
        )
    }

    pub async fn fetch() -> (StatusCode, Json<ResponseEnvelope<Vec<String>>>) {
        (StatusCode::OK, Json(ResponseEnvelope::ok(vec![])))
    }

    pub async fn verify_repo() -> (StatusCode, Json<ResponseEnvelope<String>>) {
        (StatusCode::OK, Json(ResponseEnvelope::ok("ok".to_string())))
    }
}
