#[tokio::test]
async fn ws_rejects_too_large() {
    use warp::test::WsClient;
    use warp::Filter;

    // Setup the WebSocket server filter with a size limit
    let ws_filter = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket| async move {
                let (mut tx, mut rx) = websocket.split();
                while let Some(Ok(msg)) = rx.next().await {
                    if let Ok(text) = msg.to_str() {
                        // Simulate rejecting messages that are too large
                        if text.len() > 1024 {
                            let _ = tx.send(warp::ws::Message::text("Message too large"));
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
    let client = warp::test::ws().path("/ws").handshake(addr).await.unwrap();

    // Send a message that exceeds the size limit
    let large_message = "a".repeat(2048);
    client.send(warp::ws::Message::text(large_message)).await.unwrap();

    // Assert the server responds with "Message too large"
    let response = client.recv().await.unwrap();
    assert_eq!(response.to_str().unwrap(), "Message too large");
}