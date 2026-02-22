//projects/products/core/central_ui/src/handlers/login.rs
use protocol_accounts::LoginRequest;

use crate::handlers::response_with_status;
use crate::login_input::LoginInput;

pub(crate) async fn handle_login(
    req: LoginInput,
    client: reqwest::Client,
    engine_base: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let body = LoginRequest {
        user_id: req.user_id,
        password: req.password,
        role: None,
        duration_ms: None,
        session_id: None,
    };

    let url = format!("{engine_base}/auth/login");
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
