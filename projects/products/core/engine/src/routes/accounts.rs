// projects/products/core/engine/src/routes/accounts.rs
use common_json::pjson;
use warp::{Reply, http::StatusCode};

use super::auth::normalize_user_id;
use super::helpers::{http_error, map_summary, parse_permissions, parse_role, require_admin};
use crate::EngineState;
use accounts_backend::AccountStoreError;
use protocol::accounts::{
    AccountsListResponse, CreateAccountRequest, ResetPasswordRequest, UpdateAccountRequest,
    UpdateStatusRequest,
};

/// List all user accounts
pub async fn list_accounts(
    headers: warp::http::HeaderMap,
    state: EngineState,
) -> Result<impl Reply, warp::Rejection> {
    let _token = match require_admin(&headers, &state) {
        Ok(token) => token,
        Err(err) => return Ok(err),
    };

    let users = state
        .account_manager
        .list()
        .await
        .into_iter()
        .map(map_summary)
        .collect::<Vec<_>>();

    Ok(warp::reply::with_status(
        warp::reply::json(&AccountsListResponse { users }),
        StatusCode::OK,
    ))
}

/// Get a specific user account by user_id
pub async fn get_account(
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

    match state.account_manager.get(&user_id).await {
        Ok(summary) => Ok(warp::reply::with_status(
            warp::reply::json(&map_summary(summary)),
            StatusCode::OK,
        )),
        Err(AccountStoreError::NotFound) => Ok(http_error(StatusCode::NOT_FOUND, "User not found")),
        Err(_) => Ok(http_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to load user",
        )),
    }
}

/// Create a new user account
pub async fn create_account(
    req: CreateAccountRequest,
    headers: warp::http::HeaderMap,
    state: EngineState,
) -> Result<impl Reply, warp::Rejection> {
    let token = match require_admin(&headers, &state) {
        Ok(token) => token,
        Err(err) => return Ok(err),
    };

    let user_id = match normalize_user_id(&req.user_id) {
        Ok(id) => id,
        Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
    };

    let role = match parse_role(&req.role) {
        Ok(role) => role,
        Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
    };

    let perms = match parse_permissions(&req.permissions) {
        Ok(perms) => perms,
        Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
    };

    match state
        .account_manager
        .create(
            user_id,
            &req.password,
            role,
            perms,
            &token.subject_id.to_string(),
        )
        .await
    {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&pjson!({ "ok": true })),
            StatusCode::CREATED,
        )),
        Err(AccountStoreError::AlreadyExists) => {
            Ok(http_error(StatusCode::CONFLICT, "User already exists"))
        }
        Err(AccountStoreError::InvalidPassword) => {
            Ok(http_error(StatusCode::BAD_REQUEST, "Invalid password"))
        }
        Err(_) => Ok(http_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Create failed",
        )),
    }
}

/// Update user role and permissions
pub async fn update_account(
    user_id: String,
    req: UpdateAccountRequest,
    headers: warp::http::HeaderMap,
    state: EngineState,
) -> Result<impl Reply, warp::Rejection> {
    let token = match require_admin(&headers, &state) {
        Ok(token) => token,
        Err(err) => return Ok(err),
    };

    let role = match req.role {
        Some(role) => match parse_role(&role) {
            Ok(role) => Some(role),
            Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
        },
        None => None,
    };

    let perms = match req.permissions {
        Some(perms) => match parse_permissions(&perms) {
            Ok(perms) => Some(perms),
            Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
        },
        None => None,
    };

    let user_id = match normalize_user_id(&user_id) {
        Ok(id) => id,
        Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
    };

    match state
        .account_manager
        .update_role_permissions(&user_id, role, perms, &token.subject_id.to_string())
        .await
    {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&pjson!({ "ok": true })),
            StatusCode::OK,
        )),
        Err(AccountStoreError::NotFound) => Ok(http_error(StatusCode::NOT_FOUND, "User not found")),
        Err(_) => Ok(http_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Update failed",
        )),
    }
}

/// Update user account status
pub async fn update_status(
    user_id: String,
    req: UpdateStatusRequest,
    headers: warp::http::HeaderMap,
    state: EngineState,
) -> Result<impl Reply, warp::Rejection> {
    let token = match require_admin(&headers, &state) {
        Ok(token) => token,
        Err(err) => return Ok(err),
    };

    let user_id = match normalize_user_id(&user_id) {
        Ok(id) => id,
        Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
    };

    let status = match req.status.parse::<accounts_backend::AccountStatus>() {
        Ok(status) => status,
        Err(_) => return Ok(http_error(StatusCode::BAD_REQUEST, "Invalid status")),
    };

    match state
        .account_manager
        .update_status(&user_id, status, &token.subject_id.to_string())
        .await
    {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&pjson!({ "ok": true })),
            StatusCode::OK,
        )),
        Err(AccountStoreError::NotFound) => Ok(http_error(StatusCode::NOT_FOUND, "User not found")),
        Err(_) => Ok(http_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Update failed",
        )),
    }
}

/// Reset user password
pub async fn reset_password(
    user_id: String,
    req: ResetPasswordRequest,
    headers: warp::http::HeaderMap,
    state: EngineState,
) -> Result<impl Reply, warp::Rejection> {
    let token = match require_admin(&headers, &state) {
        Ok(token) => token,
        Err(err) => return Ok(err),
    };

    let user_id = match normalize_user_id(&user_id) {
        Ok(id) => id,
        Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
    };

    match state
        .account_manager
        .reset_password(&user_id, &req.password, &token.subject_id.to_string())
        .await
    {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&pjson!({ "ok": true })),
            StatusCode::OK,
        )),
        Err(AccountStoreError::NotFound) => Ok(http_error(StatusCode::NOT_FOUND, "User not found")),
        Err(AccountStoreError::InvalidPassword) => {
            Ok(http_error(StatusCode::BAD_REQUEST, "Invalid password"))
        }
        Err(_) => Ok(http_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Reset failed",
        )),
    }
}
