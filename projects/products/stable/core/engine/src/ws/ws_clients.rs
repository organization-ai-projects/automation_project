// projects/products/stable/core/engine/src/ws/ws_clients.rs
use anyhow::Result;
use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

pub(crate) struct WsClient {
    url: String,
}

impl WsClient {
    pub(crate) fn new(url: &str) -> Result<Self> {
        Ok(Self {
            url: url.to_string(),
        })
    }

    pub(crate) async fn send_request(&self, request: &str) -> Result<String> {
        let url = self.url.clone();
        let (mut ws_stream, _) = connect_async(&url)
            .await
            .map_err(|e| anyhow::anyhow!("WebSocket connection failed: {}", e))?;

        let utf8_request = Bytes::from(request.to_string());
        ws_stream
            .send(Message::Binary(utf8_request))
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send message: {}", e))?;

        // Receive the response
        if let Some(Ok(Message::Text(response))) = ws_stream.next().await {
            return Ok(response.to_string());
        }

        Err(anyhow::anyhow!("Failed to receive response"))
    }
}
