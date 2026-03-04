// projects/products/unstable/meta_determinism_guard/ui/src/screens/scan_screen.rs
use crate::widgets::table_widget;
use common_json::Json;

pub fn display(response_json: &str, json_mode: bool) {
    if json_mode {
        println!("{response_json}");
        return;
    }

    println!("=== Scan Results ===");
    let Ok(value) = common_json::from_json_str::<Json>(response_json) else {
        println!("{response_json}");
        return;
    };

    match response_type(&value).as_deref() {
        Some("report") => {
            let findings = value
                .as_object()
                .and_then(|obj| obj.get("data"))
                .and_then(Json::as_object)
                .and_then(|data| data.get("scan_findings"))
                .and_then(Json::as_array)
                .cloned()
                .unwrap_or_default();

            if findings.is_empty() {
                println!("No forbidden pattern detected.");
                return;
            }

            let rows: Vec<Vec<String>> = findings
                .iter()
                .enumerate()
                .filter_map(|(idx, item)| {
                    item.as_str()
                        .map(|finding| vec![(idx + 1).to_string(), finding.to_string()])
                })
                .collect();
            table_widget::render_table(&["#", "Finding"], &rows);
        }
        Some("error") => {
            let message = value
                .as_object()
                .and_then(|obj| obj.get("message"))
                .and_then(Json::as_str)
                .unwrap_or("unknown error");
            println!("Error: {message}");
        }
        _ => println!("{response_json}"),
    }
}

fn response_type(value: &Json) -> Option<String> {
    value
        .as_object()
        .and_then(|obj| obj.get("type"))
        .and_then(Json::as_str)
        .map(str::to_string)
}
