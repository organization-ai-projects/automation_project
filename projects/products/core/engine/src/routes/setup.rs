// projects/products/core/engine/src/routes/setup.rs
use common_json::pjson;
use tracing::warn;
use warp::{Reply, http::StatusCode};

use super::auth::parse_user_id;
use super::helpers::http_error;
use crate::{BootstrapError, EngineState, consume_claim, setup_complete, validate_claim};
use protocol::accounts::{SetupAdminRequest, SetupAdminResponse, SetupStatusResponse};

/// Health check endpoint
pub async fn health() -> Result<impl Reply, warp::Rejection> {
    Ok(warp::reply::json(&pjson!({
        "ok": true,
        "service": "engine"
    })))
}

/// Setup admin account (initial bootstrap)
pub async fn setup_admin(
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

    if state.account_manager.user_count().await > 0 {
        return Ok(http_error(StatusCode::CONFLICT, "Admin already exists"));
    }

    if let Err(e) = state
        .account_manager
        .create(
            user_id.to_string(),
            &req.password,
            security::Role::Admin,
            Vec::new(),
            "bootstrap",
        )
        .await
    {
        warn!(error = %e, "Setup failed to create admin user");
        return Ok(http_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Setup failed",
        ));
    }

    if let Err(e) = consume_claim() {
        warn!(error = %e, "Setup failed to consume claim");
        return Ok(http_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Setup failed",
        ));
    }

    Ok(warp::reply::with_status(
        warp::reply::json(&SetupAdminResponse { ok: true }),
        StatusCode::CREATED,
    ))
}

/// Get setup status (whether setup is required)
pub async fn setup_status() -> Result<impl Reply, warp::Rejection> {
    let setup_mode = !setup_complete().unwrap_or(false);
    Ok(warp::reply::json(&SetupStatusResponse { setup_mode }))
}
