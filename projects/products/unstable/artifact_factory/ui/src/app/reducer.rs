use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::screen::Screen;
use common_json::Json;

pub fn reduce(state: &mut AppState, action: Action) {
    match action {
        Action::LoadInputs(paths) => {
            state.input_paths = paths;
            state.current_screen = Screen::Graph;
        }
        Action::Analyze => {
            state.current_screen = Screen::Graph;
        }
        Action::RenderDocs => {
            state.current_screen = Screen::Render;
        }
        Action::BuildBundle | Action::GetBundle => {
            state.current_screen = Screen::Bundle;
        }
        Action::Quit => {
            state.running = false;
        }
    }
}

pub fn apply_response_payload(state: &mut AppState, payload: &Json) {
    let Some(response) = payload
        .as_object()
        .and_then(|root| root.get("response"))
        .and_then(Json::as_object)
    else {
        return;
    };

    let Some(response_type) = response.get("type").and_then(Json::as_str) else {
        return;
    };

    match response_type {
        "inputs_loaded" => {
            state.inputs_total = read_usize(response.get("total"));
            state.input_reports = read_usize(response.get("reports"));
            state.input_replays = read_usize(response.get("replays"));
            state.input_manifests = read_usize(response.get("manifests"));
            state.input_protocol_schemas = read_usize(response.get("protocol_schemas"));
            state.input_unknown = read_usize(response.get("unknown"));
            state.last_error = None;
        }
        "analysis_complete" => {
            state.events_count = read_usize(response.get("events"));
            state.protocols_count = read_usize(response.get("protocols"));
            state.nodes_count = read_usize(response.get("nodes"));
            state.edges_count = read_usize(response.get("edges"));
            state.last_error = None;
        }
        "docs_rendered" => {
            state.markdown_bytes = read_usize(response.get("markdown_bytes"));
            state.svg_bytes = read_usize(response.get("svg_bytes"));
            state.html_bytes = read_usize(response.get("html_bytes"));
            state.last_error = None;
        }
        "bundle" => {
            state.bundle_hash = response
                .get("hash")
                .and_then(Json::as_str)
                .map(str::to_string);
            state.bundle_manifest = response
                .get("manifest")
                .and_then(Json::as_array)
                .map(|items| {
                    items
                        .iter()
                        .filter_map(Json::as_str)
                        .map(str::to_string)
                        .collect()
                })
                .unwrap_or_default();
            state.last_error = None;
        }
        "error" => {
            state.last_error = response
                .get("message")
                .and_then(Json::as_str)
                .map(str::to_string)
                .or_else(|| Some("unknown backend error".to_string()));
        }
        _ => {}
    }
}

fn read_usize(value: Option<&Json>) -> usize {
    value.and_then(Json::as_u64).unwrap_or(0) as usize
}
