// projects/products/unstable/meta_determinism_guard/ui/src/screens/stability_screen.rs
use crate::widgets::diff_widget;
use crate::widgets::table_widget;
use common_json::Json;

pub fn display(response_json: &str, json_mode: bool) {
    if json_mode {
        println!("{response_json}");
        return;
    }

    println!("=== Stability Results ===");
    let Ok(value) = common_json::from_json_str::<Json>(response_json) else {
        println!("{response_json}");
        return;
    };

    match response_type(&value).as_deref() {
        Some("report") => {
            let Some(stability) = value
                .as_object()
                .and_then(|obj| obj.get("data"))
                .and_then(Json::as_object)
                .and_then(|data| data.get("stability"))
                .and_then(Json::as_object)
            else {
                println!("No stability report available yet.");
                return;
            };

            let stable = stability
                .get("stable")
                .and_then(Json::as_bool)
                .unwrap_or(false);
            let runs = stability.get("runs").and_then(Json::as_u64).unwrap_or(0);
            println!("Stable: {stable}");
            println!("Runs: {runs}");

            if let Some(run_hashes) = stability.get("run_hashes").and_then(Json::as_array) {
                let rows: Vec<Vec<String>> = run_hashes
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, hash)| {
                        hash.as_str()
                            .map(|value| vec![(idx + 1).to_string(), value.to_string()])
                    })
                    .collect();
                if !rows.is_empty() {
                    table_widget::render_table(&["Run", "Hash"], &rows);
                }
            }

            if let Some(diff) = stability.get("diff").and_then(Json::as_str) {
                println!();
                println!("Diff:");
                diff_widget::render_prefixed_diff(diff);
            }
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
