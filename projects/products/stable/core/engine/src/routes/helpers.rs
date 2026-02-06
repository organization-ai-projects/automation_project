// projects/products/stable/core/engine/src/routes/helpers.rs
use warp::{http::StatusCode, reply::WithStatus};

use protocol::{Event, EventType};

/// HTTP error response builder
pub(crate) fn http_error(
    code: StatusCode,
    message: impl Into<String>,
) -> WithStatus<warp::reply::Json> {
    let err = common_json::pjson!({
        "code": i32::from(code.as_u16()),
        "message": message.into(),
    });
    warp::reply::with_status(warp::reply::json(&err), code)
}

/// Extract Bearer token from Authorization header
pub(crate) fn bearer_token(headers: &warp::http::HeaderMap) -> Option<String> {
    let value = headers.get("authorization")?.to_str().ok()?;
    let value = value.trim();
    value
        .strip_prefix("Bearer ")
        .map(|token| token.trim().to_string())
}

/// Validate admin role from token
pub(crate) fn require_admin(
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

pub(crate) fn event_to_http(event: Event, ok_status: StatusCode) -> WithStatus<warp::reply::Json> {
    if event.event_type == EventType::Error {
        if let Some(payload) = event.payload.and_then(|p| p.payload)
            && let common_json::Json::Object(map) = payload
            && let Some(common_json::Json::Number(status)) = map.get("status")
        {
            let code = status.as_f64() as u16;
            let msg = map
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("Request failed");
            let status = StatusCode::from_u16(code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            return http_error(status, msg);
        }
        return http_error(StatusCode::INTERNAL_SERVER_ERROR, "Backend error");
    }

    if let Some(payload) = event.payload.and_then(|p| p.payload) {
        return warp::reply::with_status(warp::reply::json(&payload), ok_status);
    }

    warp::reply::with_status(
        warp::reply::json(&common_json::pjson!({ "ok": true })),
        ok_status,
    )
}
