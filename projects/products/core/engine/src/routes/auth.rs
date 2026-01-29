// projects/products/core/engine/src/routes/auth.rs
use common::Id128;
use identity::UserId;
use tracing::warn;
use warp::{Reply, http::StatusCode};

use super::helpers::{event_to_http, http_error};
use crate::EngineState;
use crate::const_values::{DEFAULT_DURATION_MS, LOGIN_MAX_DURATION_MS};
use crate::routes::http_forwarder::{accounts_product_id, forward_to_backend, payload_from};
use protocol::accounts::LoginRequest;

/// Parse user_id from hex string
pub(crate) fn parse_user_id(input: &str) -> Result<UserId, &'static str> {
    if let Ok(id) = Id128::from_hex(input) {
        return UserId::new(id).map_err(|_| "invalid user id");
    }

    Err("user_id must be 32 hex chars")
}

/// Normalize user_id string (validate and return canonical form)
pub(crate) fn normalize_user_id(input: &str) -> Result<String, &'static str> {
    parse_user_id(input).map(|id| id.to_string())
}

/// Login handler - forward to accounts backend
pub(crate) async fn login(
    req: LoginRequest,
    state: EngineState,
) -> Result<impl Reply, warp::Rejection> {
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
        .unwrap_or(DEFAULT_DURATION_MS)
        .min(LOGIN_MAX_DURATION_MS);

    let mut req = req;
    req.duration_ms = Some(duration_ms);

    let product_id = match accounts_product_id() {
        Ok(id) => id,
        Err(msg) => return Ok(http_error(StatusCode::BAD_GATEWAY, msg)),
    };
    let payload = payload_from(req, None);
    let event = match forward_to_backend(&product_id, "accounts.login", payload, &state).await {
        Ok(ev) => ev,
        Err(msg) => return Ok(http_error(StatusCode::BAD_GATEWAY, msg)),
    };

    Ok(event_to_http(event, StatusCode::OK))
}
