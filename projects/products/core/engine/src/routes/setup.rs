// projects/products/core/engine/src/routes/setup.rs
use common_json::pjson;
use tracing::warn;
use warp::{Reply, http::StatusCode};

use super::auth::parse_user_id;
use super::helpers::{event_to_http, http_error};
use crate::routes::http_forwarder::{accounts_product_id, forward_to_backend, payload_from};
use crate::{BootstrapError, EngineState, consume_claim, setup_complete, validate_claim};
use protocol::accounts::{SetupAdminRequest, SetupStatusResponse};

/// Health check endpoint
pub(crate) async fn health() -> Result<impl Reply, warp::Rejection> {
    Ok(warp::reply::json(&pjson!({
        "ok": true,
        "service": "engine"
    })))
}

/// Setup admin account (initial bootstrap)
pub(crate) async fn setup_admin(
    req: SetupAdminRequest,
    state: EngineState,
) -> Result<impl Reply, warp::Rejection> {
    if req.password.trim().is_empty() {
        return Ok(http_error(StatusCode::BAD_REQUEST, "Password is required"));
    }

    if let Err(err) = validate_claim(&req.claim) {
        return Ok(match err {
            BootstrapError::ClaimMissing
            | BootstrapError::ClaimInvalid
            | BootstrapError::ClaimExpired => http_error(StatusCode::UNAUTHORIZED, "Invalid claim"),
            BootstrapError::SetupAlreadyCompleted => {
                http_error(StatusCode::CONFLICT, "Setup already completed")
            }
            _ => http_error(StatusCode::INTERNAL_SERVER_ERROR, "Setup failed"),
        });
    }

    let user_id = match parse_user_id(&req.user_id) {
        Ok(id) => id,
        Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
    };

    let product_id = match accounts_product_id() {
        Ok(id) => id,
        Err(msg) => return Ok(http_error(StatusCode::BAD_GATEWAY, msg)),
    };
    let payload = payload_from(
        pjson!({
            "claim": req.claim,
            "user_id": user_id.to_string(),
            "password": req.password
        }),
        None,
    );
    let event = match forward_to_backend(&product_id, "accounts.setup_admin", payload, &state).await
    {
        Ok(ev) => ev,
        Err(msg) => return Ok(http_error(StatusCode::BAD_GATEWAY, msg)),
    };

    if let Err(e) = consume_claim() {
        warn!(error = %e, "Setup failed to consume claim");
        return Ok(http_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Setup failed",
        ));
    }

    Ok(event_to_http(event, StatusCode::CREATED))
}

/// Get setup status (whether setup is required)
pub(crate) async fn setup_status() -> Result<impl Reply, warp::Rejection> {
    let setup_mode = !setup_complete().unwrap_or(false);
    Ok(warp::reply::json(&SetupStatusResponse { setup_mode }))
}
