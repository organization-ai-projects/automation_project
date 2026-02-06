// projects/products/stable/accounts/ui/src/parse_json.rs
use gloo_net::http::Response;

/// Parse JSON response from HTTP request
pub async fn parse_json<T: serde::de::DeserializeOwned>(resp: Response) -> Result<T, String> {
    let text = resp.text().await.map_err(|e| e.to_string())?;
    common_json::from_json_str(&text).map_err(|e| e.to_string())
}

/// Truncate token to show only first 8 characters
pub fn short_token(token: &str) -> String {
    let keep = 8usize.min(token.len());
    format!("{}...", &token[..keep])
}

/// Format timestamp (Option<u64>) to readable string
pub fn format_ts(ts: Option<u64>) -> String {
    ts.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string())
}
