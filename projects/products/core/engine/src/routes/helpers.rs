// projects/products/core/engine/src/routes/helpers.rs
use std::str::FromStr;
use warp::{http::StatusCode, reply::WithStatus};

use protocol::accounts::AccountSummary;

/// HTTP error response builder
pub fn http_error(code: StatusCode, message: impl Into<String>) -> WithStatus<warp::reply::Json> {
    let err = common_json::pjson!({
        "code": i32::from(code.as_u16()),
        "message": message.into(),
    });
    warp::reply::with_status(warp::reply::json(&err), code)
}

/// Extract Bearer token from Authorization header
pub fn bearer_token(headers: &warp::http::HeaderMap) -> Option<String> {
    let value = headers.get("authorization")?.to_str().ok()?;
    let value = value.trim();
    value
        .strip_prefix("Bearer ")
        .map(|token| token.trim().to_string())
}

/// Validate admin role from token
pub fn require_admin(
    headers: &warp::http::HeaderMap,
    state: &crate::EngineState,
) -> Result<security::Token, WithStatus<warp::reply::Json>> {
    let token = bearer_token(headers)
        .ok_or_else(|| http_error(StatusCode::UNAUTHORIZED, "Missing token"))?;
    let token = state
        .token_service
        .verify(&token)
        .map_err(|_| http_error(StatusCode::UNAUTHORIZED, "Invalid token"))?;
    if token.role != security::Role::Admin {
        return Err(http_error(StatusCode::FORBIDDEN, "Admin role required"));
    }
    Ok(token)
}

/// Parse role string to security::Role
pub fn parse_role(input: &str) -> Result<security::Role, &'static str> {
    security::Role::from_str(input).map_err(|_| "Invalid role")
}

/// Parse permission strings to security::Permission vector
pub fn parse_permissions(values: &[String]) -> Result<Vec<security::Permission>, &'static str> {
    let mut perms = Vec::new();
    for value in values {
        let perm = security::Permission::from_str(value).map_err(|_| "Invalid permission")?;
        perms.push(perm);
    }
    Ok(perms)
}

/// Map backend AccountSummary to protocol AccountSummary
pub fn map_summary(summary: accounts_backend::AccountSummary) -> AccountSummary {
    AccountSummary {
        user_id: summary.user_id,
        role: summary.role.as_str().to_string(),
        permissions: summary
            .permissions
            .into_iter()
            .map(|p| p.as_str().to_string())
            .collect(),
        status: summary.status.as_str().to_string(),
        created_at_ms: summary.created_at_ms,
        updated_at_ms: summary.updated_at_ms,
        last_login_ms: summary.last_login_ms,
    }
}
