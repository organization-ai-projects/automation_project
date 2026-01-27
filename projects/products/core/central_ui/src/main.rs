use std::path::{Path, PathBuf};

use protocol::accounts::{LoginRequest, SetupAdminRequest};
use serde::Deserialize;
use warp::{Filter, Reply};

#[derive(Debug, Deserialize)]
struct OwnerClaim {
    secret: String,
}

#[derive(Debug, Deserialize)]
struct SetupAdminInput {
    user_id: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginInput {
    user_id: String,
    password: String,
}

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

    let ui_route = accounts_ui_route();

    let routes = ui_route
        .or(setup_status)
        .or(setup_admin)
        .or(login)
        .or(accounts_proxy);

    println!("central_ui proxy listening on 127.0.0.1:7171");
    warp::serve(routes).run(([127, 0, 0, 1], 7171)).await;
}

fn with_client(
    client: reqwest::Client,
) -> impl Filter<Extract = (reqwest::Client,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || client.clone())
}

fn with_engine_base(
    engine_base: String,
) -> impl Filter<Extract = (String,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || engine_base.clone())
}

fn with_claim_dir(
    claim_dir: Option<String>,
) -> impl Filter<Extract = (Option<String>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || claim_dir.clone())
}

async fn handle_setup_status(
    client: reqwest::Client,
    engine_base: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let url = format!("{engine_base}/setup/status");
    let resp = client.get(url).send().await.map_err(|_| warp::reject())?;
    let status = resp.status();
    let body = resp.bytes().await.map_err(|_| warp::reject())?;
    Ok(response_with_status(body, status))
}

async fn handle_setup_admin(
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

async fn handle_login(
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

fn owner_claim_path(claim_dir: Option<String>) -> PathBuf {
    if let Some(dir) = claim_dir {
        return PathBuf::from(dir).join("owner.claim");
    }

    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join(".automation_project")
        .join("owner.claim")
}
//replace serde_json with common_json
fn read_claim(path: &PathBuf) -> Result<OwnerClaim, std::io::Error> {
    let data = std::fs::read(path)?;
    let claim: OwnerClaim = common_json::from_slice(&data).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("invalid owner claim {}: {}", path.display(), e),
        )
    })?;
    Ok(claim)
}

async fn handle_accounts_proxy(
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

fn accounts_ui_route() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    let ui_dist = std::env::var("CENTRAL_UI_ACCOUNTS_UI_DIST")
        .ok()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("projects/products/accounts/ui/ui_dist"));

    if let Err(missing) = validate_bundle(&ui_dist) {
        for item in missing {
            eprintln!("central_ui: missing accounts UI bundle file: {}", item);
        }
    }

    let index = warp::path::end()
        .and(warp::fs::file(ui_dist.join("index.html")))
        .map(|file: warp::fs::File| file.into_response());

    let files = warp::fs::dir(ui_dist);

    index.or(files).boxed()
}

fn validate_bundle(ui_dist: &Path) -> Result<(), Vec<String>> {
    let mut missing = Vec::new();
    let index = ui_dist.join("public").join("index.html");
    if !index.exists() {
        missing.push(index.display().to_string());
    }

    let assets_dir = ui_dist.join("public").join("assets");
    let js_found = std::fs::read_dir(&assets_dir)
        .ok()
        .and_then(|mut entries| {
            entries.find_map(|entry| {
                entry.ok().and_then(|entry| {
                    let path = entry.path();
                    if path.extension().and_then(|ext| ext.to_str()) == Some("js") {
                        Some(path)
                    } else {
                        None
                    }
                })
            })
        });
    if js_found.is_none() {
        missing.push(assets_dir.join("*.js").display().to_string());
    }

    let wasm_found = std::fs::read_dir(&assets_dir)
        .ok()
        .and_then(|mut entries| {
            entries.find_map(|entry| {
                entry.ok().and_then(|entry| {
                    let path = entry.path();
                    if path.extension().and_then(|ext| ext.to_str()) == Some("wasm") {
                        Some(path)
                    } else {
                        None
                    }
                })
            })
        });
    if wasm_found.is_none() {
        missing.push(assets_dir.join("*.wasm").display().to_string());
    }

    let manifest = ui_dist.join("ui_manifest.ron");
    if !manifest.exists() {
        missing.push(manifest.display().to_string());
    }
    if missing.is_empty() {
        Ok(())
    } else {
        Err(missing)
    }
}

fn response_with_status(
    body: bytes::Bytes,
    status: warp::http::StatusCode,
) -> warp::reply::Response {
    let mut resp = warp::reply::Response::new(body.into());
    *resp.status_mut() = status;
    resp
}
