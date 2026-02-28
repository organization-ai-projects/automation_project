//projects/products/core/central_ui/src/main.rs
mod claims;
mod filters;
mod handlers;
mod login_input;
mod owner_claim;
mod setup_admin_input;
mod ui;

use warp::Filter;

use crate::filters::{with_claim_dir, with_client, with_engine_base};
use crate::handlers::{
    handle_accounts_proxy, handle_login, handle_project_start, handle_project_stop,
    handle_setup_admin, handle_setup_status,
};
use crate::ui::accounts_ui_route;

#[tokio::main]
async fn main() {
    let engine_base = std::env::var("CENTRAL_UI_ENGINE_HTTP")
        .unwrap_or_else(|_| "http://127.0.0.1:3030".to_string());
    let claim_dir = std::env::var("CENTRAL_UI_OWNER_CLAIM_DIR").ok();

    let client = reqwest::Client::new();

    let api = warp::path("api");

    let setup_status = api
        .and(warp::path!("setup" / "status"))
        .and(warp::get())
        .and(with_client(client.clone()))
        .and(with_engine_base(engine_base.clone()))
        .and_then(handle_setup_status);

    let setup_admin = api
        .and(warp::path!("setup" / "admin"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_client(client.clone()))
        .and(with_engine_base(engine_base.clone()))
        .and(with_claim_dir(claim_dir.clone()))
        .and_then(handle_setup_admin);

    let login = api
        .and(warp::path!("login"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_client(client.clone()))
        .and(with_engine_base(engine_base.clone()))
        .and_then(handle_login);

    let accounts_proxy = api
        .and(warp::path("accounts"))
        .and(warp::path::tail())
        .and(warp::method())
        .and(warp::header::optional::<String>("authorization"))
        .and(warp::body::bytes())
        .and(with_client(client.clone()))
        .and(with_engine_base(engine_base.clone()))
        .and_then(handle_accounts_proxy);

    let project_start = api
        .and(warp::path("projects"))
        .and(warp::path::param::<String>())
        .and(warp::path("start"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::header::optional::<String>("authorization"))
        .and(with_client(client.clone()))
        .and(with_engine_base(engine_base.clone()))
        .and_then(handle_project_start);

    let project_stop = api
        .and(warp::path("projects"))
        .and(warp::path::param::<String>())
        .and(warp::path("stop"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::header::optional::<String>("authorization"))
        .and(with_client(client.clone()))
        .and(with_engine_base(engine_base.clone()))
        .and_then(handle_project_stop);

    let ui_route = accounts_ui_route();

    let routes = ui_route
        .or(setup_status)
        .or(setup_admin)
        .or(login)
        .or(accounts_proxy)
        .or(project_start)
        .or(project_stop);

    println!("central_ui proxy listening on 127.0.0.1:7171");
    warp::serve(routes).run(([127, 0, 0, 1], 7171)).await;
}
