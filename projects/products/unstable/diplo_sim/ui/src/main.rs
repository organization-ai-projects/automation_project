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
    let run = backend.client().send(transport::request::Request::RunMatch);
    let replay_response = backend
        .client()
        .send(transport::request::Request::ReplayMatch);
    let error = match replay_response {
        transport::response::Response::Ok => diagnostics::error::Error::Ui("ready".to_string()),
        transport::response::Response::Error(message) => {
            diagnostics::error::Error::Transport(message)
        }
    };
    println!("health={health:?} run={run:?} replay={error:?}");
    println!("{}", screens::match_screen::screen_subtitle());
    println!("{} ui (wasm build target)", ui_app::product_name());
}

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(app::App);
}
