mod app;
mod components;
mod diagnostics;
mod screens;
#[cfg(test)]
mod tests;
mod transport;
mod ui_app;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let backend = transport::backend_process::BackendProcess::new();
    let health = backend.client().send(transport::request::Request::Health);
    let maps = backend.client().send(transport::request::Request::ListMaps);
    let map_info = backend
        .client()
        .send(transport::request::Request::GetMapInfo {
            map_id: "tiny_triangle".to_string(),
        });
    let run = backend
        .client()
        .send(transport::request::Request::RunMatch {
            map_id: "tiny_triangle".to_string(),
            turns: 5,
            seed: 42,
            players: 2,
        });
    let replay_response = match run {
        transport::response::Response::MatchRun { run_id } => backend
            .client()
            .send(transport::request::Request::ReplayMatch { run_id }),
        _ => transport::response::Response::Error("run did not produce run_id".to_string()),
    };
    let run_status = match run {
        transport::response::Response::MatchRun { run_id } => backend
            .client()
            .send(transport::request::Request::GetRunStatus { run_id }),
        _ => transport::response::Response::Error("run did not produce run_id".to_string()),
    };
    let error = match replay_response {
        transport::response::Response::ReplayReady { .. } => {
            diagnostics::error::Error::Ui("ready".to_string())
        }
        transport::response::Response::Error(message) => {
            diagnostics::error::Error::Transport(message)
        }
        _ => diagnostics::error::Error::Transport("unexpected replay response".to_string()),
    };
    println!(
        "health={health:?} maps={maps:?} map_info={map_info:?} run={run:?} status={run_status:?} replay={error:?}"
    );
    println!("{}", screens::match_screen::screen_subtitle());
    println!("{} ui (wasm build target)", ui_app::product_name());
}

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(app::App);
}
