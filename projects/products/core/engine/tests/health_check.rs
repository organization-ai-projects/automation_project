// projects/products/core/engine/tests/health_check.rs
use engine::{BackendRegistry, CorsConfig, EngineState, build_routes};
use security::TokenService;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Filter;
use warp::http::header::ACCESS_CONTROL_ALLOW_ORIGIN;

#[tokio::test]
async fn health_check_works() {
    // Arrange: create a minimal EngineState
    let registry = engine::Registry::default();
    let token_service =
        TokenService::new_hs256("this_is_a_very_long_test_secret_key_for_testing_purposes")
            .unwrap();
    let state = EngineState {
        registry: Arc::new(RwLock::new(registry)),
        token_service: Arc::new(token_service),
        backend_registry: Arc::new(RwLock::new(BackendRegistry::new())),
    };

    let cors = CorsConfig {
        allow_any_origin: true,
        ..Default::default()
    };
    let routes = build_routes(state, cors);

    // Act: simulate a request to the /health endpoint with Origin header
    let response = warp::test::request()
        .method("GET")
        .path("/health")
        .header("origin", "http://localhost:3000")
        .reply(&routes)
        .await;

    // Debug: log headers for inspection
    println!("Response headers: {:?}", response.headers());

    // Assert: verify the response
    assert_eq!(response.status(), 200);

    // Parse the response body as a string and sort the keys manually
    let body = std::str::from_utf8(response.body()).unwrap();
    let mut body_parts: Vec<&str> = body
        .trim_matches(|c| c == '{' || c == '}')
        .split(',')
        .map(|s| s.trim())
        .collect();
    body_parts.sort();

    let expected_body = ["\"ok\":true", "\"service\":\"engine\""];
    assert_eq!(body_parts, expected_body);

    // When allow_any_origin is true, Warp echoes back the origin for security reasons
    assert_eq!(
        response.headers().get(ACCESS_CONTROL_ALLOW_ORIGIN).unwrap(),
        "http://localhost:3000"
    );
}

#[tokio::test]
async fn health_check_handles_not_found() {
    // Arrange: create a minimal EngineState
    let registry = engine::Registry::default();
    let token_service =
        TokenService::new_hs256("this_is_a_very_long_test_secret_key_for_testing_purposes")
            .unwrap();
    let state = EngineState {
        registry: Arc::new(RwLock::new(registry)),
        token_service: Arc::new(token_service),
        backend_registry: Arc::new(RwLock::new(BackendRegistry::new())),
    }; // Ajout du champ manquant

    let cors = CorsConfig::default();
    let routes = build_routes(state, cors);

    // Act: simulate a request to an unknown endpoint
    let response = warp::test::request()
        .method("GET")
        .path("/unknown")
        .reply(&routes)
        .await;

    // Assert: verify the response
    assert_eq!(response.status(), 404);
    let body = std::str::from_utf8(response.body()).unwrap();
    assert!(body.contains("Not Found"));
}

#[tokio::test]
async fn cors_applied_to_other_routes() {
    // Arrange: create a minimal EngineState
    let registry = engine::Registry::default();
    let token_service =
        TokenService::new_hs256("this_is_a_very_long_test_secret_key_for_testing_purposes")
            .unwrap();
    let state = EngineState {
        registry: Arc::new(RwLock::new(registry)),
        token_service: Arc::new(token_service),
        backend_registry: Arc::new(RwLock::new(BackendRegistry::new())),
    }; // Ajout du champ manquant

    let cors = CorsConfig {
        allow_any_origin: true,
        ..Default::default()
    };
    let routes = build_routes(state, cors);

    // Act: simulate a request to the /projects endpoint with Origin header
    let response = warp::test::request()
        .method("GET")
        .path("/projects")
        .header("origin", "http://localhost:3000")
        .reply(&routes)
        .await;

    // Debug: log headers for inspection
    println!("Response headers for /projects: {:?}", response.headers());

    // Assert: verify the CORS header (Warp echoes back the origin)
    assert_eq!(
        response.headers().get(ACCESS_CONTROL_ALLOW_ORIGIN).unwrap(),
        "http://localhost:3000"
    );
}

#[tokio::test]
async fn minimal_cors_test() {
    // Arrange: create a minimal EngineState
    let registry = engine::Registry::default();
    let token_service =
        TokenService::new_hs256("this_is_a_longer_test_key_to_avoid_secrettoosmall_error").unwrap();
    let state = EngineState {
        registry: Arc::new(RwLock::new(registry)),
        token_service: Arc::new(token_service),
        backend_registry: Arc::new(RwLock::new(BackendRegistry::new())),
    }; // Ajout du champ manquant

    let cors = CorsConfig {
        allow_any_origin: true,
        ..Default::default()
    };
    let routes = build_routes(state, cors);

    // Act: simulate a request to a simple route with Origin header
    let response = warp::test::request()
        .method("GET")
        .path("/health")
        .header("origin", "http://example.com")
        .reply(&routes)
        .await;

    // Debug: log headers for inspection
    println!("Minimal test response headers: {:?}", response.headers());

    // Assert: verify the CORS header (Warp echoes back the origin)
    assert_eq!(
        response.headers().get(ACCESS_CONTROL_ALLOW_ORIGIN).unwrap(),
        "http://example.com"
    );
}

#[tokio::test]
async fn custom_cors_test() {
    // Arrange: create a simple route with a custom CORS implementation
    let route = warp::path("test")
        .map(|| warp::reply::with_header("OK", "Access-Control-Allow-Origin", "*"));

    // Act: simulate a request to the /test endpoint
    let response = warp::test::request()
        .method("GET")
        .path("/test")
        .reply(&route)
        .await;

    // Debug: log headers for inspection
    println!(
        "Custom CORS test response headers: {:?}",
        response.headers()
    );

    // Assert: verify the CORS header
    assert_eq!(
        response.headers().get(ACCESS_CONTROL_ALLOW_ORIGIN).unwrap(),
        "*"
    );
}

#[tokio::test]
async fn isolated_cors_route_test() {
    // Arrange: create a simple route with CORS middleware
    let cors = warp::cors().allow_any_origin().allow_methods(["GET"]);

    let route = warp::path("isolated").map(|| "OK").with(cors);

    // Act: simulate a request to the /isolated endpoint with Origin header
    let response = warp::test::request()
        .method("GET")
        .path("/isolated")
        .header("origin", "http://test.com")
        .reply(&route)
        .await;

    // Debug: log headers for inspection
    println!("Isolated route response headers: {:?}", response.headers());

    // Assert: verify the CORS header (Warp echoes back the origin)
    assert_eq!(
        response.headers().get(ACCESS_CONTROL_ALLOW_ORIGIN).unwrap(),
        "http://test.com"
    );
}
