//projects/products/core/central_ui/src/handlers/setup_status.rs
use crate::handlers::response_with_status;

pub(crate) async fn handle_setup_status(
    client: reqwest::Client,
    engine_base: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let url = format!("{engine_base}/setup/status");
    let resp = client.get(url).send().await.map_err(|_| warp::reject())?;
    let status = resp.status();
    let body = resp.bytes().await.map_err(|_| warp::reject())?;
    Ok(response_with_status(body, status))
}
