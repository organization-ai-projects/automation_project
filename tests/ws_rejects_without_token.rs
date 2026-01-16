use warp::test::WsClient;

#[tokio::test]
async fn ws_rejects_without_token() {
    // Arrange: Build the app with routes
    let config = engine::config::EngineConfig::from_env().expect("Failed to load config");
    let state = engine::EngineState::default();
    let routes = engine::routes::build_routes(state, config.cors);

    // Act: Attempt to connect to the WebSocket without a token
    let client = warp::test::ws().path("/ws").handshake(routes).await;

    // Assert: Ensure the connection is rejected
    assert!(client.is_err(), "WebSocket connection should be rejected without a token");
}