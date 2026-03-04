use crate::app::app_state::AppState;
use crate::diagnostics::ui_error::UiError;
use common_json::Json;

pub fn apply_response(state: &mut AppState, response_json: &str) -> Result<(), UiError> {
    state.last_response = Some(response_json.to_string());
    let value: Json = common_json::from_json_str(response_json)?;

    if response_type(&value).as_deref() == Some("report")
        && let Some(data) = value
            .as_object()
            .and_then(|obj| obj.get("data"))
            .and_then(Json::as_object)
    {
        state.scan_findings = read_string_list(data.get("scan_findings"));
        state.canon_issues = read_string_list(data.get("canon_issues"));
    }

    Ok(())
}

pub fn response_is_error(response_json: &str) -> bool {
    common_json::from_json_str::<Json>(response_json)
        .ok()
        .and_then(|value| response_type(&value))
        .as_deref()
        == Some("error")
}

fn response_type(value: &Json) -> Option<String> {
    value
        .as_object()
        .and_then(|obj| obj.get("type"))
        .and_then(Json::as_str)
        .map(str::to_string)
}

fn read_string_list(value: Option<&Json>) -> Vec<String> {
    value
        .and_then(Json::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Json::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}
