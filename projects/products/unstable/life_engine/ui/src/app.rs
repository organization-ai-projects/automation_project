use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OnboardingState {
    pub step: OnboardingStep,
    pub event_type: Option<String>,
    pub income: Option<f64>,
    pub goal: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum OnboardingStep {
    #[default]
    Intent,
    Details,
    Results,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecommendationOutput {
    pub actions: Vec<String>,
    pub estimations: Vec<String>,
    pub warnings: Vec<String>,
    pub opportunities: Vec<String>,
}

#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
#[component]
pub fn app() -> Element {
    let mut state = use_signal(OnboardingState::default);
    let mut results = use_signal(|| Option::<RecommendationOutput>::None);

    let current_step = state.read().step.clone();

    match current_step {
        OnboardingStep::Intent => rsx! {
            div {
                h1 { "Life Engine" }
                h2 { "What brings you here?" }
                button {
                    onclick: move |_| {
                        state.write().event_type = Some("job_loss".to_string());
                        state.write().step = OnboardingStep::Details;
                    },
                    "Job loss"
                }
                button {
                    onclick: move |_| {
                        state.write().event_type = Some("new_job".to_string());
                        state.write().step = OnboardingStep::Details;
                    },
                    "New job"
                }
                button {
                    onclick: move |_| {
                        state.write().event_type = Some("health_issue".to_string());
                        state.write().step = OnboardingStep::Details;
                    },
                    "Health issue"
                }
            }
        },
        OnboardingStep::Details => rsx! {
            div {
                h1 { "Life Engine" }
                h2 { "Tell us more (optional)" }
                p { "Add your salary to unlock estimation" }
                button {
                    onclick: move |_| {
                        let st = state.read().clone();
                        let output = evaluate_locally(&st);
                        results.set(Some(output));
                        state.write().step = OnboardingStep::Results;
                    },
                    "See my recommendations"
                }
            }
        },
        OnboardingStep::Results => {
            let res = results.read().clone().unwrap_or_default();
            rsx! {
                div {
                    h1 { "Life Engine — Results" }
                    h3 { "Actions" }
                    ul {
                        for action in res.actions.iter() {
                            li { "{action}" }
                        }
                    }
                    if !res.estimations.is_empty() {
                        h3 { "Estimations" }
                        ul {
                            for est in res.estimations.iter() {
                                li { "{est}" }
                            }
                        }
                    }
                    if !res.warnings.is_empty() {
                        h3 { "Warnings" }
                        ul {
                            for warn in res.warnings.iter() {
                                li { "{warn}" }
                            }
                        }
                    }
                    if !res.opportunities.is_empty() {
                        h3 { "Opportunities" }
                        ul {
                            for opp in res.opportunities.iter() {
                                li { "{opp}" }
                            }
                        }
                    }
                    button {
                        onclick: move |_| {
                            state.set(OnboardingState::default());
                            results.set(None);
                        },
                        "Start over"
                    }
                }
            }
        }
    }
}

fn evaluate_locally(state: &OnboardingState) -> RecommendationOutput {
    let mut output = RecommendationOutput::default();

    match state.event_type.as_deref() {
        Some("job_loss") => {
            output.actions.push("Declare situation to CAF".to_string());
            output
                .actions
                .push("Prepare France Travail registration".to_string());
            output
                .actions
                .push("Check mutuelle portability".to_string());

            if let Some(income) = state.income {
                let benefit = income * 0.57;
                output.estimations.push(format!(
                    "Estimated monthly unemployment benefit: {benefit:.2} EUR"
                ));
            }

            output.warnings.push(
                "Risk of missing France Travail registration deadline (12 months)".to_string(),
            );
            output
                .opportunities
                .push("Explore training programs (CPF)".to_string());
            output
                .opportunities
                .push("Job suggestions (basic placeholder)".to_string());
        }
        Some("new_job") => {
            output
                .actions
                .push("Update CAF with new employment status".to_string());
            output
                .actions
                .push("Notify France Travail of employment".to_string());
        }
        Some("health_issue") => {
            output
                .actions
                .push("Contact CPAM for sick leave declaration".to_string());
            output
                .actions
                .push("Notify employer within 48 hours".to_string());
            output
                .warnings
                .push("48-hour deadline for sick leave declaration".to_string());
        }
        _ => {
            output
                .actions
                .push("Please select an event to get recommendations".to_string());
        }
    }

    output
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run() -> Result<(), String> {
    println!("life_engine_ui - native mode placeholder");
    println!("Build with --target wasm32-unknown-unknown for web UI");
    Ok(())
}
