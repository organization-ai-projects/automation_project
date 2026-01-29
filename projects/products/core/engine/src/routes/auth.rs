// projects/products/core/engine/src/routes/auth.rs
use common::Id128;
use identity::UserId;
use tracing::warn;
use warp::{Reply, http::StatusCode};

use super::helpers::http_error;
use crate::EngineState;
use crate::const_values::{DEFAULT_DURATION_MS, LOGIN_MAX_DURATION_MS};
use protocol::accounts::{LoginRequest, LoginResponse};

/// Parse user_id from hex string
pub fn parse_user_id(input: &str) -> Result<UserId, &'static str> {
    if let Ok(id) = Id128::from_hex(input) {
        return UserId::new(id).map_err(|_| "invalid user id");
    }

    Err("user_id must be 32 hex chars")
}

/// Normalize user_id string (validate and return canonical form)
pub fn normalize_user_id(input: &str) -> Result<String, &'static str> {
    parse_user_id(input).map(|id| id.to_string())
}

/// Login handler - authenticate user and issue JWT token
pub async fn login(req: LoginRequest, state: EngineState) -> Result<impl Reply, warp::Rejection> {
    if req.password.trim().is_empty() {
        warn!("Login failed: empty password for user_id={}", req.user_id);
        return Ok(http_error(StatusCode::UNAUTHORIZED, "Invalid credentials"));
    }

    let user_id = match parse_user_id(&req.user_id) {
        Ok(id) => id,
        Err(e) => {
            warn!("Login failed: invalid user_id={}: {}", req.user_id, e);
            return Ok(http_error(StatusCode::BAD_REQUEST, e));
        }
    };

    if req.role.is_some() {
        warn!(
            "Login request tried to set role from client for user_id={}",
            user_id
        );
    }

    let duration_ms = req
        .duration_ms
        .unwrap_or_else(|| {
            eprintln!(
                "Duration not provided, using default: {} ms",
                DEFAULT_DURATION_MS
            );
            DEFAULT_DURATION_MS
        })
        .min(LOGIN_MAX_DURATION_MS);

    let role = match state
        .account_manager
        .authenticate(&user_id.to_string(), &req.password)
        .await
    {
        Ok(r) => r,
        Err(_) => {
            warn!("Login failed: invalid credentials for user_id={}", user_id);
            return Ok(http_error(StatusCode::UNAUTHORIZED, "Invalid credentials"));
        }
    };

    let jwt = state
        .token_service
        .issue(user_id.value(), role, duration_ms, req.session_id)
        .map_err(|e| {
            warn!(
                "Login failed: JWT issue error for user_id={}: {:?}",
                user_id, e
            );
            warp::reject()
        })?;

    Ok(warp::reply::with_status(
        warp::reply::json(&LoginResponse { jwt }),
        StatusCode::OK,
    ))
}
