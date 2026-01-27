// projects/products/core/engine/src/routes.rs
use std::{collections::HashMap, convert::Infallible, str::FromStr};

use common::Id128;
use common_json::pjson;
use tracing::warn;
use warp::{Filter, Reply, http::StatusCode};

use crate::{BootstrapError, consume_claim, setup_complete, validate_claim};
use accounts_backend::{AccountStatus, AccountStoreError};
use identity::UserId;

use crate::const_values::{DEFAULT_DURATION_MS, LOGIN_MAX_DURATION_MS};
use crate::{CorsConfig, EngineState};
use protocol::accounts::{
    AccountSummary, AccountsListResponse, CreateAccountRequest, LoginRequest, LoginResponse,
    ResetPasswordRequest, SetupAdminRequest, SetupAdminResponse, SetupStatusResponse,
    UpdateAccountRequest, UpdateStatusRequest,
};

fn http_error(
    code: StatusCode,
    message: impl Into<String>,
) -> warp::reply::WithStatus<warp::reply::Json> {
    let err = common_json::pjson!({
        "code": i32::from(code.as_u16()),
        "message": message.into(),
    });
    warp::reply::with_status(warp::reply::json(&err), code)
}

fn bearer_token(headers: &warp::http::HeaderMap) -> Option<String> {
    let value = headers.get("authorization")?.to_str().ok()?;
    let value = value.trim();
    value
        .strip_prefix("Bearer ")
        .map(|token| token.trim().to_string())
}

fn require_admin(
    headers: &warp::http::HeaderMap,
    state: &EngineState,
) -> Result<security::Token, warp::reply::WithStatus<warp::reply::Json>> {
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

fn parse_role(input: &str) -> Result<security::Role, &'static str> {
    security::Role::from_str(input).map_err(|_| "Invalid role")
}

fn parse_permissions(values: &[String]) -> Result<Vec<security::Permission>, &'static str> {
    let mut perms = Vec::new();
    for value in values {
        let perm = security::Permission::from_str(value).map_err(|_| "Invalid permission")?;
        perms.push(perm);
    }
    Ok(perms)
}

fn map_summary(summary: accounts_backend::AccountSummary) -> AccountSummary {
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

async fn health() -> Result<impl Reply, warp::Rejection> {
    Ok(warp::reply::json(&pjson!({
        "ok": true,
        "service": "engine"
    })))
}

async fn login(req: LoginRequest, state: EngineState) -> Result<impl Reply, warp::Rejection> {
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

fn parse_user_id(input: &str) -> Result<UserId, &'static str> {
    if let Ok(id) = Id128::from_hex(input) {
        return UserId::new(id).map_err(|_| "invalid user id");
    }

    if input.len() == 16 {
        let bytes: [u8; 16] = input
            .as_bytes()
            .try_into()
            .map_err(|_| "user_id must be 16 bytes or 32 hex chars")?;
        let id = Id128::from_bytes_unchecked(bytes);
        return UserId::new(id).map_err(|_| "invalid user id");
    }

    Err("user_id must be 16 bytes or 32 hex chars")
}

fn normalize_user_id(input: &str) -> Result<String, &'static str> {
    parse_user_id(input).map(|id| id.to_string())
}

async fn setup_admin(
    req: SetupAdminRequest,
    state: EngineState,
) -> Result<impl Reply, warp::Rejection> {
    if setup_complete().unwrap_or(false) {
        return Ok(http_error(StatusCode::CONFLICT, "Setup already completed"));
    }

    if req.password.trim().is_empty() {
        return Ok(http_error(StatusCode::BAD_REQUEST, "Password is required"));
    }

    if state.account_manager.user_count().await > 0 {
        return Ok(http_error(StatusCode::CONFLICT, "Admin already exists"));
    }

    let user_id = match parse_user_id(&req.user_id) {
        Ok(id) => id,
        Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
    };

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

async fn list_projects(state: EngineState) -> Result<impl Reply, warp::Rejection> {
    let reg = state.registry.read().await;
    let list: Vec<_> = reg.projects.values().cloned().collect();
    Ok(warp::reply::json(&list))
}

async fn setup_status() -> Result<impl Reply, warp::Rejection> {
    let setup_mode = !setup_complete().unwrap_or(false);
    Ok(warp::reply::json(&SetupStatusResponse { setup_mode }))
}

async fn list_accounts(
    headers: warp::http::HeaderMap,
    state: EngineState,
) -> Result<impl Reply, warp::Rejection> {
    let _token = match require_admin(&headers, &state) {
        Ok(token) => token,
        Err(err) => return Ok(err),
    };

    let users = state
        .account_manager
        .list()
        .await
        .into_iter()
        .map(map_summary)
        .collect::<Vec<_>>();

    Ok(warp::reply::with_status(
        warp::reply::json(&AccountsListResponse { users }),
        StatusCode::OK,
    ))
}

async fn get_account(
    user_id: String,
    headers: warp::http::HeaderMap,
    state: EngineState,
) -> Result<impl Reply, warp::Rejection> {
    let _token = match require_admin(&headers, &state) {
        Ok(token) => token,
        Err(err) => return Ok(err),
    };

    let user_id = match normalize_user_id(&user_id) {
        Ok(id) => id,
        Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
    };

    match state.account_manager.get(&user_id).await {
        Ok(summary) => Ok(warp::reply::with_status(
            warp::reply::json(&map_summary(summary)),
            StatusCode::OK,
        )),
        Err(AccountStoreError::NotFound) => Ok(http_error(StatusCode::NOT_FOUND, "User not found")),
        Err(_) => Ok(http_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to load user",
        )),
    }
}

async fn create_account(
    req: CreateAccountRequest,
    headers: warp::http::HeaderMap,
    state: EngineState,
) -> Result<impl Reply, warp::Rejection> {
    let token = match require_admin(&headers, &state) {
        Ok(token) => token,
        Err(err) => return Ok(err),
    };

    let user_id = match normalize_user_id(&req.user_id) {
        Ok(id) => id,
        Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
    };

    let role = match parse_role(&req.role) {
        Ok(role) => role,
        Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
    };

    let perms = match parse_permissions(&req.permissions) {
        Ok(perms) => perms,
        Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
    };

    match state
        .account_manager
        .create(
            user_id,
            &req.password,
            role,
            perms,
            &token.subject_id.to_string(),
        )
        .await
    {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&pjson!({ "ok": true })),
            StatusCode::CREATED,
        )),
        Err(AccountStoreError::AlreadyExists) => {
            Ok(http_error(StatusCode::CONFLICT, "User already exists"))
        }
        Err(AccountStoreError::InvalidPassword) => {
            Ok(http_error(StatusCode::BAD_REQUEST, "Invalid password"))
        }
        Err(_) => Ok(http_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Create failed",
        )),
    }
}

async fn update_account(
    user_id: String,
    req: UpdateAccountRequest,
    headers: warp::http::HeaderMap,
    state: EngineState,
) -> Result<impl Reply, warp::Rejection> {
    let token = match require_admin(&headers, &state) {
        Ok(token) => token,
        Err(err) => return Ok(err),
    };

    let role = match req.role {
        Some(role) => match parse_role(&role) {
            Ok(role) => Some(role),
            Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
        },
        None => None,
    };

    let perms = match req.permissions {
        Some(perms) => match parse_permissions(&perms) {
            Ok(perms) => Some(perms),
            Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
        },
        None => None,
    };

    let user_id = match normalize_user_id(&user_id) {
        Ok(id) => id,
        Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
    };

    match state
        .account_manager
        .update_role_permissions(&user_id, role, perms, &token.subject_id.to_string())
        .await
    {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&pjson!({ "ok": true })),
            StatusCode::OK,
        )),
        Err(AccountStoreError::NotFound) => Ok(http_error(StatusCode::NOT_FOUND, "User not found")),
        Err(_) => Ok(http_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Update failed",
        )),
    }
}

async fn update_status(
    user_id: String,
    req: UpdateStatusRequest,
    headers: warp::http::HeaderMap,
    state: EngineState,
) -> Result<impl Reply, warp::Rejection> {
    let token = match require_admin(&headers, &state) {
        Ok(token) => token,
        Err(err) => return Ok(err),
    };

    let user_id = match normalize_user_id(&user_id) {
        Ok(id) => id,
        Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
    };

    let status = match req.status.parse::<AccountStatus>() {
        Ok(status) => status,
        Err(_) => return Ok(http_error(StatusCode::BAD_REQUEST, "Invalid status")),
    };

    match state
        .account_manager
        .update_status(&user_id, status, &token.subject_id.to_string())
        .await
    {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&pjson!({ "ok": true })),
            StatusCode::OK,
        )),
        Err(AccountStoreError::NotFound) => Ok(http_error(StatusCode::NOT_FOUND, "User not found")),
        Err(_) => Ok(http_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Update failed",
        )),
    }
}

async fn reset_password(
    user_id: String,
    req: ResetPasswordRequest,
    headers: warp::http::HeaderMap,
    state: EngineState,
) -> Result<impl Reply, warp::Rejection> {
    let token = match require_admin(&headers, &state) {
        Ok(token) => token,
        Err(err) => return Ok(err),
    };

    let user_id = match normalize_user_id(&user_id) {
        Ok(id) => id,
        Err(e) => return Ok(http_error(StatusCode::BAD_REQUEST, e)),
    };

    match state
        .account_manager
        .reset_password(&user_id, &req.password, &token.subject_id.to_string())
        .await
    {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&pjson!({ "ok": true })),
            StatusCode::OK,
        )),
        Err(AccountStoreError::NotFound) => Ok(http_error(StatusCode::NOT_FOUND, "User not found")),
        Err(AccountStoreError::InvalidPassword) => {
            Ok(http_error(StatusCode::BAD_REQUEST, "Invalid password"))
        }
        Err(_) => Ok(http_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Reset failed",
        )),
    }
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
        warn!("CORS: allow_any_origin enabled. Not recommended for production.");
        cors_mw = cors_mw.allow_any_origin();
    } else if let Some(origin) = cors.allow_origin.as_deref() {
        cors_mw = cors_mw.allow_origin(origin);
    }

    base.with(cors_mw).recover(recover)
}
