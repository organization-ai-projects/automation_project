// projects/products/unstable/city_builder/ui/src/web_app.rs
use dioxus::prelude::*;

#[component]
pub fn web_app() -> Element {
    let mut flow = use_signal(|| "run".to_string());
    let mut scenario = use_signal(|| "scenarios/small_town_growth.json".to_string());
    let mut out = use_signal(|| "report.json".to_string());
    let mut replay = use_signal(|| "replay.json".to_string());
    let mut replay_out = use_signal(|| "replay.json".to_string());
    let mut ticks = use_signal(|| "200".to_string());
    let mut seed = use_signal(|| "42".to_string());
    let mut at_tick = use_signal(|| "50".to_string());
    let status = use_signal(|| "Ready".to_string());

    let mut submit = {
        let flow = flow;
        let scenario = scenario;
        let out = out;
        let replay = replay;
        let replay_out = replay_out;
        let ticks = ticks;
        let seed = seed;
        let at_tick = at_tick;
        let mut status = status;

        move || {
            let flow_value = flow.read().clone();
            let (url, body) = build_http_request(
                &flow_value,
                &scenario.read(),
                &out.read(),
                &replay.read(),
                &replay_out.read(),
                &ticks.read(),
                &seed.read(),
                &at_tick.read(),
            );

            status.set("Sending request...".to_string());
            spawn(async move {
                let message = match send_command(&url, &body).await {
                    Ok(ok) => ok,
                    Err(err) => format!("Request failed: {err}"),
                };
                status.set(message);
            });
        }
    };

    rsx! {
        style { {r#"
            :root {
                --bg: #f4f8fb;
                --panel: #ffffff;
                --ink: #0f172a;
                --muted: #475569;
                --accent: #0f766e;
                --border: #dbe4ec;
            }
            body {
                margin: 0;
                font-family: "IBM Plex Sans", "Segoe UI", sans-serif;
                background: linear-gradient(145deg, #edf4fb, var(--bg));
                color: var(--ink);
            }
            .shell {
                max-width: 980px;
                margin: 32px auto;
                padding: 0 16px;
            }
            .card {
                background: var(--panel);
                border: 1px solid var(--border);
                border-radius: 14px;
                box-shadow: 0 12px 30px rgba(15, 23, 42, 0.05);
                padding: 18px 20px;
                margin-bottom: 14px;
            }
            .row {
                display: grid;
                grid-template-columns: 1fr 1fr;
                gap: 12px;
            }
            .field {
                display: flex;
                flex-direction: column;
                gap: 6px;
                margin-bottom: 10px;
            }
            label {
                color: var(--muted);
                font-size: 0.9rem;
            }
            input, select {
                border: 1px solid var(--border);
                border-radius: 8px;
                padding: 8px 10px;
                font-size: 0.95rem;
                background: #fff;
            }
            button {
                border: 0;
                border-radius: 10px;
                padding: 10px 16px;
                font-size: 0.95rem;
                background: var(--accent);
                color: #fff;
                cursor: pointer;
            }
            .status {
                font-family: "IBM Plex Mono", "Fira Code", monospace;
                color: #0b3954;
                white-space: pre-wrap;
                word-break: break-word;
            }
            @media (max-width: 720px) {
                .row { grid-template-columns: 1fr; }
            }
        "#} }

        div {
            class: "shell",
            div {
                class: "card",
                h1 { "City Builder UI (Dioxus)" }
                p { "Web UI with real HTTP calls to city_builder endpoints." }
            }

            div {
                class: "card",
                div {
                    class: "field",
                    label { "Flow" }
                    select {
                        value: "{flow}",
                        onchange: move |evt| flow.set(evt.value()),
                        option { value: "run", "run" }
                        option { value: "replay", "replay" }
                        option { value: "snapshot", "snapshot" }
                        option { value: "validate", "validate" }
                    }
                }

                div {
                    class: "row",
                    if flow.read().as_str() == "run" || flow.read().as_str() == "validate" {
                        div {
                            class: "field",
                            label { "Scenario path" }
                            input {
                                value: "{scenario}",
                                oninput: move |evt| scenario.set(evt.value()),
                            }
                        }
                    }
                    if flow.read().as_str() != "validate" {
                        div {
                            class: "field",
                            label { "Output path" }
                            input {
                                value: "{out}",
                                oninput: move |evt| out.set(evt.value()),
                            }
                        }
                    }
                    if flow.read().as_str() == "run" {
                        div {
                            class: "field",
                            label { "Seed" }
                            input {
                                value: "{seed}",
                                oninput: move |evt| seed.set(evt.value()),
                            }
                        }
                        div {
                            class: "field",
                            label { "Ticks" }
                            input {
                                value: "{ticks}",
                                oninput: move |evt| ticks.set(evt.value()),
                            }
                        }
                        div {
                            class: "field",
                            label { "Replay output path" }
                            input {
                                value: "{replay_out}",
                                oninput: move |evt| replay_out.set(evt.value()),
                            }
                        }
                    }
                    if flow.read().as_str() == "replay" || flow.read().as_str() == "snapshot" {
                        div {
                            class: "field",
                            label { "Replay path" }
                            input {
                                value: "{replay}",
                                oninput: move |evt| replay.set(evt.value()),
                            }
                        }
                    }
                    if flow.read().as_str() == "snapshot" {
                        div {
                            class: "field",
                            label { "Tick" }
                            input {
                                value: "{at_tick}",
                                oninput: move |evt| at_tick.set(evt.value()),
                            }
                        }
                    }
                }

                button {
                    onclick: move |_| submit(),
                    "Execute"
                }
            }

            div {
                class: "card",
                h2 { "Backend response" }
                div { class: "status", "{status}" }
            }
        }
    }
}

fn build_http_request(
    flow: &str,
    scenario: &str,
    out: &str,
    replay: &str,
    replay_out: &str,
    ticks: &str,
    seed: &str,
    at_tick: &str,
) -> (String, String) {
    let base = city_builder_api_base();
    match flow {
        "replay" => (
            format!("{base}/replay"),
            format!(
                "{{\"replay\":\"{}\",\"out\":\"{}\"}}",
                json_escape(replay),
                json_escape(out),
            ),
        ),
        "snapshot" => (
            format!("{base}/snapshot"),
            format!(
                "{{\"replay\":\"{}\",\"out\":\"{}\",\"at_tick\":{}}}",
                json_escape(replay),
                json_escape(out),
                parse_u64_or_default(at_tick, 1),
            ),
        ),
        "validate" => (
            format!("{base}/validate"),
            format!("{{\"scenario\":\"{}\"}}", json_escape(scenario)),
        ),
        _ => (
            format!("{base}/run"),
            format!(
                "{{\"scenario\":\"{}\",\"out\":\"{}\",\"replay_out\":\"{}\",\"ticks\":{},\"seed\":{}}}",
                json_escape(scenario),
                json_escape(out),
                json_escape(replay_out),
                parse_u64_or_default(ticks, 200),
                parse_u64_or_default(seed, 42),
            ),
        ),
    }
}

fn city_builder_api_base() -> String {
    std::env::var("CITY_BUILDER_UI_API_BASE").unwrap_or_else(|_| "/api/city_builder".to_string())
}

async fn send_command(url: &str, body: &str) -> Result<String, String> {
    let response = gloo_net::http::Request::post(url)
        .header("content-type", "application/json")
        .body(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = response.status();
    let text = response.text().await.map_err(|e| e.to_string())?;
    Ok(format!("HTTP {status}\n{text}"))
}

fn parse_u64_or_default(value: &str, default: u64) -> u64 {
    value.trim().parse::<u64>().unwrap_or(default)
}

fn json_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            _ => escaped.push(ch),
        }
    }
    escaped
}
