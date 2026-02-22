//projects/products/core/central_ui/src/handlers/setup_admin.rs
use protocol_accounts::SetupAdminRequest;

use crate::claims::{owner_claim_path, read_claim};
use crate::handlers::response_with_status;
use crate::setup_admin_input::SetupAdminInput;

pub(crate) async fn handle_setup_admin(
    req: SetupAdminInput,
    client: reqwest::Client,
    engine_base: String,
    claim_dir: Option<String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let claim_path = owner_claim_path(claim_dir);
    let claim = read_claim(&claim_path).map_err(|_| warp::reject())?;

    let body = SetupAdminRequest {
        claim: claim.secret,
        user_id: req.user_id,
        password: req.password,
    };

    let url = format!("{engine_base}/setup/owner/admin");
    let resp = client
        .post(url)
        .json(&body)
        .send()
        .await
        .map_err(|_| warp::reject())?;
    let status = resp.status();
    let body = resp.bytes().await.map_err(|_| warp::reject())?;
    Ok(response_with_status(body, status))
}
