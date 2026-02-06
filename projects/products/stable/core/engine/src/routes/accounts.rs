// projects/products/core/engine/src/routes/accounts.rs
use common_json::pjson;
use warp::{Reply, http::StatusCode};

use super::auth::{normalize_user_id, validate_user_id};
use super::helpers::{event_to_http, http_error, require_admin};
use crate::EngineState;
use crate::routes::http_forwarder::{accounts_product_id, forward_to_backend, payload_from};
use protocol::accounts::{
    CreateAccountRequest, ResetPasswordRequest, UpdateAccountRequest, UpdateStatusRequest,
};

/// List all user accounts
pub(crate) async fn list_accounts(
    headers: warp::http::HeaderMap,
    state: EngineState,
) -> Result<impl Reply, warp::Rejection> {
    let _token = match require_admin(&headers, &state) {
        Ok(token) => token,
        Err(err) => return Ok(err),
    };

    let product_id = match accounts_product_id() {
        Ok(id) => id,
        Err(msg) => return Ok(http_error(StatusCode::BAD_GATEWAY, msg)),
    };
    let payload = payload_from(pjson!({}), None);
    let event = match forward_to_backend(&product_id, "accounts.list", payload, &state).await {
        Ok(ev) => ev,
        Err(msg) => return Ok(http_error(StatusCode::BAD_GATEWAY, msg)),
    };

    Ok(event_to_http(event, StatusCode::OK))
}

/// Get a specific user account by user_id
pub(crate) async fn get_account(
    user_id: String,
    headers: warp::http::HeaderMap,
    state: EngineState,
) -> Result<impl Reply, warp::Rejection> {
    let _token = match require_admin(&headers, &state) {
        Ok(token) => token,
        Err(err) => return Ok(err),
    };

    let user_id = match normalize_user_id(&user_id) {
        Ok(id) => id,
        Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
    };

    let product_id = match accounts_product_id() {
        Ok(id) => id,
        Err(msg) => return Ok(http_error(StatusCode::BAD_GATEWAY, msg)),
    };
    let payload = payload_from(pjson!({ "user_id": user_id.to_string() }), None);
    let event = match forward_to_backend(&product_id, "accounts.get", payload, &state).await {
        Ok(ev) => ev,
        Err(msg) => return Ok(http_error(StatusCode::BAD_GATEWAY, msg)),
    };

    Ok(event_to_http(event, StatusCode::OK))
}

/// Create a new user account
pub(crate) async fn create_account(
    req: CreateAccountRequest,
    headers: warp::http::HeaderMap,
    state: EngineState,
) -> Result<impl Reply, warp::Rejection> {
    let _token = match require_admin(&headers, &state) {
        Ok(token) => token,
        Err(err) => return Ok(err),
    };

    let user_id = match validate_user_id(req.user_id) {
        Ok(id) => id,
        Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
    };

    let product_id = match accounts_product_id() {
        Ok(id) => id,
        Err(msg) => return Ok(http_error(StatusCode::BAD_GATEWAY, msg)),
    };
    let payload = payload_from(
        CreateAccountRequest {
            user_id,
            password: req.password,
            role: req.role,
            permissions: req.permissions,
        },
        None,
    );
    let event = match forward_to_backend(&product_id, "accounts.create", payload, &state).await {
        Ok(ev) => ev,
        Err(msg) => return Ok(http_error(StatusCode::BAD_GATEWAY, msg)),
    };

    Ok(event_to_http(event, StatusCode::CREATED))
}

/// Update user role and permissions
pub(crate) async fn update_account(
    user_id: String,
    req: UpdateAccountRequest,
    headers: warp::http::HeaderMap,
    state: EngineState,
) -> Result<impl Reply, warp::Rejection> {
    let _token = match require_admin(&headers, &state) {
        Ok(token) => token,
        Err(err) => return Ok(err),
    };

    let user_id = match normalize_user_id(&user_id) {
        Ok(id) => id,
        Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
    };

    let product_id = match accounts_product_id() {
        Ok(id) => id,
        Err(msg) => return Ok(http_error(StatusCode::BAD_GATEWAY, msg)),
    };
    let payload = payload_from(
        pjson!({
            "user_id": user_id.to_string(),
            "role": req.role,
            "permissions": req.permissions
        }),
        None,
    );
    let event = match forward_to_backend(&product_id, "accounts.update", payload, &state).await {
        Ok(ev) => ev,
        Err(msg) => return Ok(http_error(StatusCode::BAD_GATEWAY, msg)),
    };

    Ok(event_to_http(event, StatusCode::OK))
}

/// Update user account status
pub(crate) async fn update_status(
    user_id: String,
    req: UpdateStatusRequest,
    headers: warp::http::HeaderMap,
    state: EngineState,
) -> Result<impl Reply, warp::Rejection> {
    let _token = match require_admin(&headers, &state) {
        Ok(token) => token,
        Err(err) => return Ok(err),
    };

    let user_id = match normalize_user_id(&user_id) {
        Ok(id) => id,
        Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
    };

    let product_id = match accounts_product_id() {
        Ok(id) => id,
        Err(msg) => return Ok(http_error(StatusCode::BAD_GATEWAY, msg)),
    };
    let payload = payload_from(
        pjson!({
            "user_id": user_id.to_string(),
            "status": req.status
        }),
        None,
    );
    let event =
        match forward_to_backend(&product_id, "accounts.update_status", payload, &state).await {
            Ok(ev) => ev,
            Err(msg) => return Ok(http_error(StatusCode::BAD_GATEWAY, msg)),
        };

    Ok(event_to_http(event, StatusCode::OK))
}

/// Reset user password
pub(crate) async fn reset_password(
    user_id: String,
    req: ResetPasswordRequest,
    headers: warp::http::HeaderMap,
    state: EngineState,
) -> Result<impl Reply, warp::Rejection> {
    let _token = match require_admin(&headers, &state) {
        Ok(token) => token,
        Err(err) => return Ok(err),
    };

    let user_id = match normalize_user_id(&user_id) {
        Ok(id) => id,
        Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
    };

    let product_id = match accounts_product_id() {
        Ok(id) => id,
        Err(msg) => return Ok(http_error(StatusCode::BAD_GATEWAY, msg)),
    };
    let payload = payload_from(
        pjson!({
            "user_id": user_id.to_string(),
            "password": req.password
        }),
        None,
    );
    let event =
        match forward_to_backend(&product_id, "accounts.reset_password", payload, &state).await {
            Ok(ev) => ev,
            Err(msg) => return Ok(http_error(StatusCode::BAD_GATEWAY, msg)),
        };

    Ok(event_to_http(event, StatusCode::OK))
}
