mod diagnostics;
mod screens;
mod transport;

#[cfg(target_arch = "wasm32")]
mod app;
#[cfg(test)]
mod tests;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let screen = screens::campaign_screen::CampaignScreen::new(42, 30, "ready".to_string());
    let request = transport::request::Request::RunSimulation {
        seed: screen.seed,
        days: screen.days,
    };
    let response = transport::response::Response::RunAccepted {
        summary: screen.summary_line(),
    };
    let error = diagnostics::error::Error::Ui("native placeholder".to_string());
    let transport_error = diagnostics::error::Error::Transport("native placeholder".to_string());
    let _ = (request, response, error, transport_error);
}

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(app::App);
}
