#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
const APP_STYLE: &str = r#"
:root { color-scheme: light; }
body {
  margin: 0;
  font-family: "IBM Plex Sans", "Segoe UI", system-ui, sans-serif;
  background:
    radial-gradient(circle at 20% 0%, #f1f7ff 0%, transparent 42%),
    radial-gradient(circle at 100% 100%, #f6f4ff 0%, transparent 35%),
    #f7fafc;
  color: #0f172a;
}
.page { max-width: 1040px; margin: 0 auto; padding: 24px; }
.hero { margin-bottom: 16px; }
.hero h1 { margin: 0; font-size: 30px; letter-spacing: -0.02em; }
.hero p { margin: 6px 0 0; color: #334155; }
.grid { display: grid; gap: 14px; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); }
.card {
  border: 1px solid #d7dfeb;
  border-radius: 14px;
  background: #ffffff;
  box-shadow: 0 6px 24px rgba(12, 27, 54, 0.06);
  padding: 14px;
}
.card h2 { margin: 0 0 10px; font-size: 16px; }
.row { display: flex; flex-wrap: wrap; gap: 8px; align-items: center; }
.pill {
  border-radius: 999px;
  padding: 3px 10px;
  background: #edf2ff;
  color: #25314f;
  font-size: 12px;
}
label { font-size: 13px; color: #334155; }
input, select, textarea {
  width: 100%;
  box-sizing: border-box;
  border-radius: 10px;
  border: 1px solid #cbd5e1;
  padding: 8px 10px;
  font: inherit;
  color: inherit;
  background: #fff;
}
textarea { min-height: 100px; resize: vertical; }
button {
  border: 0;
  border-radius: 10px;
  background: #0f172a;
  color: #fff;
  padding: 8px 12px;
  font-weight: 600;
  cursor: pointer;
}
button:hover { background: #1e293b; }
.list { margin: 0; padding-left: 18px; }
.list li { margin: 4px 0; }
.mono { font-family: "IBM Plex Mono", "Consolas", monospace; font-size: 12px; }
"#;

#[cfg(target_arch = "wasm32")]
pub(crate) fn launch() {
    dioxus::launch(App);
}

#[cfg(target_arch = "wasm32")]
#[component]
fn App() -> Element {
    let mut selected_strategy = use_signal(|| "HighestConfidence".to_string());
    let mut task_input = use_signal(|| "Design a deterministic orchestrator plan".to_string());
    let mut max_experts = use_signal(|| 3_u8);
    let mut simulated_runs = use_signal(|| 0_u32);
    let mut last_selected_expert = use_signal(|| "none".to_string());
    let mut trace_entries = use_signal(|| 0_u32);
    let mut dataset_entries = use_signal(|| 0_u32);
    let mut last_status = use_signal(|| "Idle".to_string());
    let mut log_lines = use_signal(|| {
        vec![
            "UI initialized for neurosymbolic_moe".to_string(),
            "Waiting for run simulation".to_string(),
        ]
    });

    let run_click = move |_| {
        let run_number = *simulated_runs.read() + 1;
        simulated_runs.set(run_number);

        let expert = if task_input.read().to_lowercase().contains("validate") {
            "validator"
        } else if task_input.read().to_lowercase().contains("transform") {
            "code_transform"
        } else {
            "code_gen"
        };

        last_selected_expert.set(expert.to_string());
        trace_entries.set(*trace_entries.read() + 4);
        dataset_entries.set(*dataset_entries.read() + 1);
        last_status.set(format!(
            "Run #{run_number} simulated with strategy {}",
            selected_strategy.read()
        ));

        let mut next = log_lines.read().clone();
        next.push(format!(
            "run#{run_number}: task='{}' max_experts={} expert={expert}",
            task_input.read(),
            max_experts.read()
        ));
        log_lines.set(next);
    };

    rsx! {
        main {
            style { "{APP_STYLE}" }

            div { class: "page",
                section { class: "hero",
                    h1 { "Neurosymbolic MoE Console" }
                    p { "Unstable UI shell for routing, traces, and dataset supervision." }
                }

                section { class: "grid",
                    article { class: "card",
                        h2 { "Execution Setup" }
                        label { "Task Input" }
                        textarea {
                            value: "{task_input}",
                            oninput: move |evt| task_input.set(evt.value().clone())
                        }
                        div { class: "row",
                            div { style: "flex:1; min-width:150px;",
                                label { "Strategy" }
                                select {
                                    value: "{selected_strategy}",
                                    onchange: move |evt| selected_strategy.set(evt.value().clone()),
                                    option { value: "HighestConfidence", "HighestConfidence" }
                                    option { value: "WeightedAverage", "WeightedAverage" }
                                    option { value: "MajorityVote", "MajorityVote" }
                                }
                            }
                            div { style: "width:140px;",
                                label { "Max Experts" }
                                input {
                                    r#type: "number",
                                    min: "1",
                                    max: "8",
                                    value: "{max_experts}",
                                    oninput: move |evt| {
                                        if let Ok(parsed) = evt.value().parse::<u8>() {
                                            max_experts.set(parsed.clamp(1, 8));
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "row", style: "margin-top:10px;",
                            button { onclick: run_click, "Simulate Run" }
                            span { class: "pill", "{last_status}" }
                        }
                    }

                    article { class: "card",
                        h2 { "Live Metrics" }
                        div { class: "row",
                            span { class: "pill", "Runs: {simulated_runs}" }
                            span { class: "pill", "Traces: {trace_entries}" }
                            span { class: "pill", "Dataset: {dataset_entries}" }
                        }
                        p { style: "margin-top:10px;", "Last selected expert: " b { "{last_selected_expert}" } }
                        p { "Current strategy: " b { "{selected_strategy}" } }
                    }

                    article { class: "card",
                        h2 { "Run Log" }
                        ul { class: "list mono",
                            for line in log_lines.read().iter() {
                                li { "{line}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
