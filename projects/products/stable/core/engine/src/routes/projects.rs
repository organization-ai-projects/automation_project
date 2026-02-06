// projects/products/stable/core/engine/src/routes/projects.rs
use warp::Reply;

use crate::EngineState;

/// List all projects from registry
pub(crate) async fn list_projects(state: EngineState) -> Result<impl Reply, warp::Rejection> {
    let reg = state.registry.read().await;
    let list: Vec<_> = reg.projects.values().cloned().collect();
    Ok(warp::reply::json(&list))
}
