use std::sync::Arc;

use accounts_backend::AccountManager;
use common::Id128;
use common_json::{Json, pjson};
use engine::{EngineState, build_routes, ensure_owner_claim};
use identity::UserId;
use security::{Role, TokenService};
use tokio::sync::RwLock;
use warp::http::StatusCode;

// These tests mutate process-wide env vars to point to temp claim dirs.
// We serialize access with CLAIM_LOCK and ensure the env var is only read
// during test setup (no concurrent reads in the tested code paths).
static CLAIM_LOCK: std::sync::OnceLock<tokio::sync::Mutex<()>> = std::sync::OnceLock::new();

fn build_state(account_manager: AccountManager) -> EngineState {
    let registry = engine::Registry::default();
    let token_service =
        TokenService::new_hs256("this_is_a_very_long_test_secret_key_for_testing_purposes")
            .expect("Failed to create TokenService");

    EngineState {
        registry: Arc::new(RwLock::new(registry)),
        token_service: Arc::new(token_service),
        backend_registry: Arc::new(RwLock::new(engine::BackendRegistry::new())),
        account_manager: Arc::new(account_manager),
    }
}

#[tokio::test]
async fn login_rejects_invalid_credentials() {
    let user_id = UserId::new(Id128::from_bytes_unchecked([1u8; 16])).unwrap();
    let mut data_dir = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    data_dir.push(format!("accounts_store_login_invalid_{nanos}"));
    let manager = AccountManager::load(data_dir.clone()).await.unwrap();
    manager
        .create(
            user_id.to_string(),
            "correct_password",
            Role::User,
            Vec::new(),
            "test",
        )
        .await
        .unwrap();

    let state = build_state(manager);
    let routes = build_routes(state, engine::CorsConfig::default());

    let body = pjson!({
        "user_id": user_id.to_string(),
        "password": "wrong_password"
    });

    let response = warp::test::request()
        .method("POST")
        .path("/auth/login")
        .json(&body)
        .reply(&routes)
        .await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn login_blocks_role_escalation() {
    let user_id = UserId::new(Id128::from_bytes_unchecked([2u8; 16])).unwrap();
    let mut data_dir = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    data_dir.push(format!("accounts_store_login_escalation_{nanos}"));
    let manager = AccountManager::load(data_dir.clone()).await.unwrap();
    manager
        .create(
            user_id.to_string(),
            "password",
            Role::User,
            Vec::new(),
            "test",
        )
        .await
        .unwrap();

    let state = build_state(manager);
    let token_service = state.token_service.clone();
    let routes = build_routes(state, engine::CorsConfig::default());

    let body = pjson!({
        "user_id": user_id.to_string(),
        "password": "password",
        "role": "Admin"
    });

    let response = warp::test::request()
        .method("POST")
        .path("/auth/login")
        .json(&body)
        .reply(&routes)
        .await;

    assert_eq!(response.status(), StatusCode::OK);
    let payload: Json = common_json::from_slice(response.body()).expect("json response");
    let jwt = match payload {
        Json::Object(map) => map
            .get("jwt")
            .and_then(|v| {
                if let Json::String(s) = v {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .expect("jwt string"),
        _ => panic!("invalid json response"),
    };
    let token = token_service.verify(&jwt).expect("verify token");

    assert_eq!(token.role, Role::User);
}

#[tokio::test]
async fn setup_admin_creates_first_admin_and_consumes_claim() {
    let _guard = CLAIM_LOCK
        .get_or_init(|| tokio::sync::Mutex::new(()))
        .lock()
        .await;
    let mut claim_dir = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    claim_dir.push(format!("engine_claim_dir_create_admin_{nanos}"));
    unsafe {
        std::env::set_var("ENGINE_OWNER_CLAIM_DIR", &claim_dir);
    }

    let setup_state = ensure_owner_claim().expect("claim creation");
    let claim: engine::OwnerClaim = {
        let data = std::fs::read(&setup_state.claim_path).expect("read claim");
        common_json::from_slice(&data).expect("parse claim")
    };

    let mut data_dir = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    data_dir.push(format!("accounts_store_setup_admin_{nanos}"));
    let manager = AccountManager::load(data_dir).await.unwrap();
    let state = build_state(manager);
    let routes = build_routes(state.clone(), engine::CorsConfig::default());

    let user_id = UserId::new(Id128::from_bytes_unchecked([3u8; 16])).unwrap();
    let body = pjson!({
        "claim": claim.secret,
        "user_id": user_id.to_string(),
        "password": "admin_password"
    });

    let response = warp::test::request()
        .method("POST")
        .path("/setup/owner/admin")
        .json(&body)
        .reply(&routes)
        .await;

    assert_eq!(response.status(), StatusCode::CREATED);

    let login_body = pjson!({
        "user_id": user_id.to_string(),
        "password": "admin_password"
    });
    let login_response = warp::test::request()
        .method("POST")
        .path("/auth/login")
        .json(&login_body)
        .reply(&routes)
        .await;
    assert_eq!(login_response.status(), StatusCode::OK);

    let replay_response = warp::test::request()
        .method("POST")
        .path("/setup/owner/admin")
        .json(&body)
        .reply(&routes)
        .await;
    assert_eq!(replay_response.status(), StatusCode::CONFLICT);

    unsafe {
        std::env::remove_var("ENGINE_OWNER_CLAIM_DIR");
    }
    let _ = std::fs::remove_dir_all(&claim_dir);
}

#[tokio::test]
async fn setup_admin_rejects_invalid_claim() {
    let _guard = CLAIM_LOCK
        .get_or_init(|| tokio::sync::Mutex::new(()))
        .lock()
        .await;
    let mut claim_dir = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    claim_dir.push(format!("engine_claim_dir_invalid_claim_{nanos}"));
    unsafe {
        std::env::set_var("ENGINE_OWNER_CLAIM_DIR", &claim_dir);
    }

    let _ = ensure_owner_claim().expect("claim creation");

    let mut data_dir = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    data_dir.push(format!("accounts_store_invalid_claim_{nanos}"));
    let manager = AccountManager::load(data_dir).await.unwrap();
    let state = build_state(manager);
    let routes = build_routes(state, engine::CorsConfig::default());

    let user_id = UserId::new(Id128::from_bytes_unchecked([4u8; 16])).unwrap();
    let body = pjson!({
        "claim": "invalid_claim",
        "user_id": user_id.to_string(),
        "password": "admin_password"
    });

    let response = warp::test::request()
        .method("POST")
        .path("/setup/owner/admin")
        .json(&body)
        .reply(&routes)
        .await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    unsafe {
        std::env::remove_var("ENGINE_OWNER_CLAIM_DIR");
    }
    let _ = std::fs::remove_dir_all(&claim_dir);
}

#[tokio::test]
async fn setup_status_flips_after_admin_creation() {
    let _guard = CLAIM_LOCK
        .get_or_init(|| tokio::sync::Mutex::new(()))
        .lock()
        .await;
    let mut claim_dir = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    claim_dir.push(format!("engine_claim_dir_status_flip_{nanos}"));
    unsafe {
        std::env::set_var("ENGINE_OWNER_CLAIM_DIR", &claim_dir);
    }

    let setup_state = ensure_owner_claim().expect("claim creation");
    let claim: engine::OwnerClaim = {
        let data = std::fs::read(&setup_state.claim_path).expect("read claim");
        common_json::from_slice(&data).expect("parse claim")
    };

    let mut data_dir = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    data_dir.push(format!("accounts_store_status_flip_{nanos}"));
    let manager = AccountManager::load(data_dir).await.unwrap();
    let state = build_state(manager);
    let routes = build_routes(state.clone(), engine::CorsConfig::default());

    let status_before = warp::test::request()
        .method("GET")
        .path("/setup/status")
        .reply(&routes)
        .await;
    assert_eq!(status_before.status(), StatusCode::OK);
    let payload: Json = common_json::from_slice(status_before.body()).expect("status json");
    match payload {
        Json::Object(map) => {
            let setup_mode = map
                .get("setup_mode")
                .and_then(|v| {
                    if let Json::Bool(b) = v {
                        Some(*b)
                    } else {
                        None
                    }
                })
                .expect("setup_mode");
            assert!(setup_mode);
        }
        _ => panic!("invalid status response"),
    }

    let user_id = UserId::new(Id128::from_bytes_unchecked([9u8; 16])).unwrap();
    let body = pjson!({
        "claim": claim.secret,
        "user_id": user_id.to_string(),
        "password": "admin_password"
    });

    let response = warp::test::request()
        .method("POST")
        .path("/setup/owner/admin")
        .json(&body)
        .reply(&routes)
        .await;
    assert_eq!(response.status(), StatusCode::CREATED);

    let status_after = warp::test::request()
        .method("GET")
        .path("/setup/status")
        .reply(&routes)
        .await;
    assert_eq!(status_after.status(), StatusCode::OK);
    let payload: Json = common_json::from_slice(status_after.body()).expect("status json");
    match payload {
        Json::Object(map) => {
            let setup_mode = map
                .get("setup_mode")
                .and_then(|v| {
                    if let Json::Bool(b) = v {
                        Some(*b)
                    } else {
                        None
                    }
                })
                .expect("setup_mode");
            assert!(!setup_mode);
        }
        _ => panic!("invalid status response"),
    }

    let login_body = pjson!({
        "user_id": user_id.to_string(),
        "password": "admin_password"
    });
    let login_response = warp::test::request()
        .method("POST")
        .path("/auth/login")
        .json(&login_body)
        .reply(&routes)
        .await;
    assert_eq!(login_response.status(), StatusCode::OK);

    let payload: Json = common_json::from_slice(login_response.body()).expect("login json");
    let jwt = match payload {
        Json::Object(map) => map
            .get("jwt")
            .and_then(|v| {
                if let Json::String(s) = v {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .expect("jwt"),
        _ => panic!("invalid login response"),
    };

    let list_response = warp::test::request()
        .method("GET")
        .path("/accounts/users")
        .header("authorization", format!("Bearer {jwt}"))
        .reply(&routes)
        .await;
    assert_eq!(list_response.status(), StatusCode::OK);

    unsafe {
        std::env::remove_var("ENGINE_OWNER_CLAIM_DIR");
    }
    let _ = std::fs::remove_dir_all(&claim_dir);
}
