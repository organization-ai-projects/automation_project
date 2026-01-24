#[tokio::test]
async fn ws_rejects_invalid_json() {
    use protocol::json;
    use warp::test::WsClient;
    use warp::Filter;

    // Setup the WebSocket server filter
    let ws_filter = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket| async move {
                let (mut tx, mut rx) = websocket.split();
                while let Some(Ok(msg)) = rx.next().await {
                    if let Ok(text) = msg.to_str() {
                        // Simulate rejecting invalid JSON
                        if json::from_json_str::<json::Json>(text).is_err() {
                            let _ = tx.send(warp::ws::Message::text("Invalid JSON"));
                        }
                    }
                }
            })
        });

    // Start the WebSocket server
    let server = warp::serve(ws_filter).bind_ephemeral(([127, 0, 0, 1], 0));
    let addr = server.0;
    tokio::task::spawn(server.1);

    // Connect to the WebSocket server
    let client = warp::test::ws().path("/ws").handshake(addr).await.or_else(|_| panic!("Handshake failed"));
    // Send invalid JSON
    client.send(warp::ws::Message::text("{invalid_json}")).await.or_else(|_| panic!("Send failed"));
    // Assert the server responds with "Invalid JSON"
    let response = client.recv().await.or_else(|_| panic!("Receive failed"));
    assert_eq!(response.to_str().or_else(|_| panic!("Invalid response")), "Invalid JSON");
}