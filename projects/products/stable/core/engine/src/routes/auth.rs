// projects/products/stable/core/engine/src/routes/auth.rs
use identity::UserId;
use protocol::ProtocolId;
use std::str::FromStr;
use tracing::warn;
use warp::{Reply, http::StatusCode};

use super::helpers::{event_to_http, http_error};
use crate::EngineState;
use crate::const_values::{DEFAULT_DURATION_MS, LOGIN_MAX_DURATION_MS};
use crate::routes::http_forwarder::{accounts_product_id, forward_to_backend, payload_from};
use protocol_accounts::LoginRequest;

/// Validate a ProtocolId as a user id
pub(crate) fn validate_user_id(id: ProtocolId) -> Result<ProtocolId, &'static str> {
    UserId::new(id).map(|_| id).map_err(|_| "invalid user id")
}

/// Parse user_id from hex string
pub(crate) fn parse_user_id(input: &str) -> Result<ProtocolId, &'static str> {
    let id = ProtocolId::from_str(input).map_err(|_| "user_id must be 32 hex chars")?;
    validate_user_id(id)
}

/// Normalize user_id string (validate and return canonical form)
pub(crate) fn normalize_user_id(input: &str) -> Result<ProtocolId, &'static str> {
    parse_user_id(input)
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

    let user_id = match validate_user_id(req.user_id) {
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
