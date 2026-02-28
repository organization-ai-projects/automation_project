// projects/products/stable/core/engine/src/routes/projects.rs
use std::str::FromStr;

use common_json::pjson;
use protocol::ProtocolId;
use warp::{Reply, http::StatusCode};

use super::helpers::{event_to_http, http_error, require_admin};
use crate::EngineState;
use crate::routes::http_forwarder::{forward_to_backend, payload_from};

/// List all projects from registry
pub(crate) async fn list_projects(state: EngineState) -> Result<impl Reply, warp::Rejection> {
    let reg = state.registry.read().await;
    let list: Vec<_> = reg.projects.values().cloned().collect();
    Ok(warp::reply::json(&list))
}

/// Start a product service (admin only)
pub(crate) async fn start_project(
    project_id: String,
    headers: warp::http::HeaderMap,
    state: EngineState,
) -> Result<impl Reply, warp::Rejection> {
    let _token = match require_admin(&headers, &state) {
        Ok(token) => token,
        Err(err) => return Ok(err),
    };

    let product_id = match ProtocolId::from_str(&project_id) {
        Ok(id) => id,
        Err(_) => return Ok(http_error(StatusCode::BAD_REQUEST, "Invalid project_id")),
    };

    let payload = payload_from(pjson!({}), None);
    let event = match forward_to_backend(&product_id, "project.start", payload, &state).await {
        Ok(ev) => ev,
        Err(msg) => return Ok(http_error(StatusCode::BAD_GATEWAY, msg)),
    };

    Ok(event_to_http(event, StatusCode::OK))
}

/// Stop a product service (admin only)
pub(crate) async fn stop_project(
    project_id: String,
    headers: warp::http::HeaderMap,
    state: EngineState,
) -> Result<impl Reply, warp::Rejection> {
    let _token = match require_admin(&headers, &state) {
        Ok(token) => token,
        Err(err) => return Ok(err),
    };

    let product_id = match ProtocolId::from_str(&project_id) {
        Ok(id) => id,
        Err(_) => return Ok(http_error(StatusCode::BAD_REQUEST, "Invalid project_id")),
    };

    let payload = payload_from(pjson!({}), None);
    let event = match forward_to_backend(&product_id, "project.stop", payload, &state).await {
        Ok(ev) => ev,
        Err(msg) => return Ok(http_error(StatusCode::BAD_GATEWAY, msg)),
    };

    Ok(event_to_http(event, StatusCode::OK))
}
