// projects/products/stable/core/engine/src/routes/orchestration.rs
use std::{collections::HashMap, convert::Infallible};

use warp::{Filter, Reply, http::StatusCode};

use super::{
    create_account, get_account, health, http_error, list_accounts, list_projects, login,
    reset_password, setup_admin, setup_status, update_account, update_status,
};
use crate::{CorsConfig, EngineState};

/// WebSocket route handler
fn ws_route(
    state: EngineState,
) -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path("ws")
        .and(warp::get())
        .and(warp::ws())
        .and(warp::query::<HashMap<String, String>>())
        .map(move |ws: warp::ws::Ws, q: HashMap<String, String>| {
            let jwt = q.get("token").cloned().unwrap_or_default();
            let st = state.clone();
            ws.on_upgrade(move |socket| crate::ws::ws_handle(socket, st, jwt))
        })
}

/// Error recovery handler
async fn recover(rejection: warp::Rejection) -> Result<impl Reply, Infallible> {
    if rejection.is_not_found() {
        return Ok(http_error(StatusCode::NOT_FOUND, "Not Found").into_response());
    }
    if rejection.find::<warp::reject::PayloadTooLarge>().is_some() {
        return Ok(http_error(StatusCode::PAYLOAD_TOO_LARGE, "Payload too large").into_response());
    }
    if rejection.find::<warp::reject::MethodNotAllowed>().is_some() {
        return Ok(
            http_error(StatusCode::METHOD_NOT_ALLOWED, "Method not allowed").into_response(),
        );
    }

    Ok(http_error(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response())
}

// Note: The CORS middleware in Warp requires an `Origin` header in the request
// to apply the CORS rules. When `allow_any_origin()` is used, Warp echoes back
// the `Origin` value from the request instead of returning `*`. This behavior
// ensures compatibility with credentials and improves security.
// Ensure that all CORS-related tests include an `Origin` header to validate
// the middleware behavior correctly.

/// Build complete route filter with all endpoints
pub(crate) fn build_routes(
    state: EngineState,
    cors: CorsConfig,
) -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone {
    let with_state = {
        let st = state.clone();
        warp::any().and_then(move || {
            let s = st.clone();
            async move { Ok::<EngineState, warp::Rejection>(s) }
        })
    };

    let health_route = warp::path!("health").and(warp::get()).and_then(health);

    let login_route = warp::path!("auth" / "login")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state.clone())
        .and_then(login);

    let setup_route = warp::path!("setup" / "owner" / "admin")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state.clone())
        .and_then(setup_admin);

    let setup_status_route = warp::path!("setup" / "status")
        .and(warp::get())
        .and_then(setup_status);

    let projects_route = warp::path!("projects")
        .and(warp::get())
        .and(with_state.clone())
        .and_then(list_projects);

    let accounts_list_route = warp::path!("accounts" / "users")
        .and(warp::get())
        .and(warp::header::headers_cloned())
        .and(with_state.clone())
        .and_then(list_accounts);

    let accounts_create_route = warp::path!("accounts" / "users")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::header::headers_cloned())
        .and(with_state.clone())
        .and_then(create_account);

    let accounts_get_route = warp::path!("accounts" / "users" / String)
        .and(warp::get())
        .and(warp::header::headers_cloned())
        .and(with_state.clone())
        .and_then(get_account);

    let accounts_update_route = warp::path!("accounts" / "users" / String)
        .and(warp::patch())
        .and(warp::body::json())
        .and(warp::header::headers_cloned())
        .and(with_state.clone())
        .and_then(update_account);

    let accounts_status_route = warp::path!("accounts" / "users" / String / "status")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::header::headers_cloned())
        .and(with_state.clone())
        .and_then(update_status);

    let accounts_reset_route = warp::path!("accounts" / "users" / String / "reset_password")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::header::headers_cloned())
        .and(with_state.clone())
        .and_then(reset_password);

    let websocket_route = ws_route(state);

    let base = health_route
        .or(login_route)
        .or(setup_route)
        .or(setup_status_route)
        .or(projects_route)
        .or(accounts_list_route)
        .or(accounts_create_route)
        .or(accounts_get_route)
        .or(accounts_update_route)
        .or(accounts_status_route)
        .or(accounts_reset_route)
        .or(websocket_route);

    let mut cors_mw = warp::cors()
        .allow_methods(["GET", "POST", "PATCH", "OPTIONS"])
        .allow_headers(["content-type", "authorization"])
        .max_age(60);

    if cors.allow_any_origin {
        tracing::warn!("CORS: allow_any_origin enabled. Not recommended for production.");
        cors_mw = cors_mw.allow_any_origin();
    } else if let Some(origin) = cors.allow_origin.as_deref() {
        cors_mw = cors_mw.allow_origin(origin);
    }

    base.with(cors_mw).recover(recover)
}
