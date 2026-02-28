//projects/products/core/central_ui/src/handlers/projects_proxy.rs
use crate::handlers::response_with_status;

pub(crate) async fn handle_project_start(
    project_id: String,
    auth_header: Option<String>,
    client: reqwest::Client,
    engine_base: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    forward_project_action(project_id, "start", auth_header, client, engine_base).await
}

pub(crate) async fn handle_project_stop(
    project_id: String,
    auth_header: Option<String>,
    client: reqwest::Client,
    engine_base: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    forward_project_action(project_id, "stop", auth_header, client, engine_base).await
}

async fn forward_project_action(
    project_id: String,
    action: &str,
    auth_header: Option<String>,
    client: reqwest::Client,
    engine_base: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let url = format!("{engine_base}/projects/{project_id}/{action}");
    let mut req = client.post(&url);

    if let Some(auth) = auth_header {
        req = req.header("authorization", auth);
    }

    let resp = req.send().await.map_err(|_| warp::reject())?;
    let status = resp.status();
    let body = resp.bytes().await.map_err(|_| warp::reject())?;
    Ok(response_with_status(body, status))
}
