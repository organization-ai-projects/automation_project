//projects/products/core/central_ui/src/handlers/accounts_proxy.rs
use crate::handlers::response_with_status;

pub(crate) async fn handle_accounts_proxy(
    tail: warp::path::Tail,
    method: warp::http::Method,
    auth_header: Option<String>,
    body: bytes::Bytes,
    client: reqwest::Client,
    engine_base: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let url = format!("{engine_base}/accounts/{}", tail.as_str());
    let mut req = client.request(method, url);

    if let Some(auth) = auth_header {
        req = req.header("authorization", auth);
    }

    if !body.is_empty() {
        req = req.header("content-type", "application/json").body(body);
    }

    let resp = req.send().await.map_err(|_| warp::reject())?;
    let status = resp.status();
    let body = resp.bytes().await.map_err(|_| warp::reject())?;
    Ok(response_with_status(body, status))
}
