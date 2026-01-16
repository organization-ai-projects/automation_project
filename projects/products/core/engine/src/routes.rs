// projects/products/core/engine/src/routes.rs
use std::{collections::HashMap, convert::Infallible};

use common::Id128;
use common_json::pjson;
use tracing::warn;
use warp::{Filter, Reply, http::StatusCode};

use protocol::ProtocolError;
use security::{Role, auth::UserId};

use crate::const_values::{DEFAULT_DURATION_MS, LOGIN_MAX_DURATION_MS};
use crate::{CorsConfig, EngineState, LoginRequest, LoginResponse};

fn http_error(code: StatusCode, message: impl Into<String>) -> impl Reply {
    let err = ProtocolError {
        code: i32::from(code.as_u16()),
        message: message.into(),
    };
    warp::reply::with_status(warp::reply::json(&err), code)
}

async fn health() -> Result<impl Reply, warp::Rejection> {
    Ok(warp::reply::json(&pjson!({
        "ok": true,
        "service": "engine"
    })))
}

async fn login(req: LoginRequest, state: EngineState) -> Result<impl Reply, warp::Rejection> {
    if req.password.trim().is_empty() {
        warn!("Login failed: empty password for user_id={}", req.user_id);
        return Err(warp::reject());
    }

    let user_id = UserId::from(Id128::from_bytes_unchecked(
        req.user_id.as_bytes().try_into().unwrap(),
    ));
    let role = req.role.unwrap_or(Role::User);

    let duration_ms = req
        .duration_ms
        .unwrap_or(DEFAULT_DURATION_MS)
        .min(LOGIN_MAX_DURATION_MS);

    let jwt = state
        .token_service
        .issue(user_id.clone(), role, duration_ms, req.session_id)
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

async fn list_projects(state: EngineState) -> Result<impl Reply, warp::Rejection> {
    let reg = state.registry.read().await;
    let list: Vec<_> = reg.projects.values().cloned().collect();
    Ok(warp::reply::json(&list))
}

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
pub fn build_routes(
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

    let projects_route = warp::path!("projects")
        .and(warp::get())
        .and(with_state.clone())
        .and_then(list_projects);

    let websocket_route = ws_route(state);

    let base = health_route
        .or(login_route)
        .or(projects_route)
        .or(websocket_route);

    let mut cors_mw = warp::cors()
        .allow_methods(["GET", "POST", "OPTIONS"])
        .allow_headers(["content-type", "authorization"])
        .max_age(60);

    if cors.allow_any_origin {
        warn!("CORS: allow_any_origin enabled. Not recommended for production.");
        cors_mw = cors_mw.allow_any_origin();
    } else if let Some(origin) = cors.allow_origin.as_deref() {
        cors_mw = cors_mw.allow_origin(origin);
    }

    base.with(cors_mw).recover(recover)
}

// Fin du fichier routes.rs
