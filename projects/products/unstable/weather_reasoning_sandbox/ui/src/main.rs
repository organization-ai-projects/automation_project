mod models;

#[cfg(test)]
mod tests;

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(app);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("weather_reasoning_sandbox_ui (wasm build only)");
    println!();
    println!("This UI displays weather reasoning sandbox outputs.");
    println!("Build for wasm32 target to run in browser.");
    println!();
    println!("Supported views:");
    println!("  - Run simulation with seed, ticks, dataset");
    println!("  - Replay prior journal");
    println!("  - Per-tick raw predictions and corrections");
    println!("  - Constraint violations");
    println!("  - Contradiction history");
    println!("  - Report and snapshot checksums");
    println!("  - Replay equivalence results");
}

#[cfg(target_arch = "wasm32")]
fn app() -> dioxus::prelude::Element {
    use dioxus::prelude::*;

    let run_state = use_signal(|| models::run_request::RunRequest::default());
    let report_view = use_signal(|| Option::<models::report_view::ReportView>::None);

    rsx! {
        div {
            class: "weather-reasoning-sandbox",
            h1 { "Weather Reasoning Sandbox" }
            p { "Deterministic Neurosymbolic Weather Reasoning Engine" }

            div {
                class: "controls",
                h2 { "Simulation Controls" }

                div {
                    label { "Seed: " }
                    input {
                        r#type: "number",
                        value: "{run_state.read().seed}",
                    }
                }

                div {
                    label { "Ticks: " }
                    input {
                        r#type: "number",
                        value: "{run_state.read().ticks}",
                    }
                }

                button { "Run Simulation" }
                button { "Replay Journal" }
            }

            div {
                class: "results",
                h2 { "Results" }

                if let Some(ref report) = *report_view.read() {
                    div {
                        class: "report-summary",
                        h3 { "Report Summary" }
                        p { "Seed: {report.seed}" }
                        p { "Ticks: {report.tick_count}" }
                        p { "Contradictions: {report.contradiction_count}" }
                        p { "Total Violations: {report.total_violations}" }
                        p { "Total Corrections: {report.total_corrections}" }
                        p { "Report Checksum: {report.report_checksum}" }
                    }
                } else {
                    p { "No simulation results yet. Run a simulation to see results." }
                }
            }

            div {
                class: "contradiction-history",
                h2 { "Contradiction History" }
                p { "Contradictions will appear here after running a simulation." }
            }

            div {
                class: "replay-equivalence",
                h2 { "Replay Equivalence" }
                p { "Replay equivalence results will appear here after replaying a journal." }
            }
        }
    }
}
