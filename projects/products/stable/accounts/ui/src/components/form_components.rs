// projects/products/stable/accounts/ui/src/form_components.rs
use dioxus::prelude::*;

pub const ROLE_OPTIONS: [&str; 4] = ["admin", "moderator", "user", "guest"];
pub const STATUS_OPTIONS: [&str; 3] = ["active", "suspended", "disabled"];

/// Text input field component
pub fn input_field(label: &str, mut value: Signal<String>, read_only: bool) -> Element {
    rsx! {
        label { "{label}" }
        input {
            readonly: read_only,
            value: "{value.read()}",
            oninput: move |evt| value.set(evt.value().clone()),
        }
    }
}

/// Password input field component
pub fn input_password(label: &str, mut value: Signal<String>) -> Element {
    rsx! {
        label { "{label}" }
        input {
            r#type: "password",
            value: "{value.read()}",
            oninput: move |evt| value.set(evt.value().clone()),
        }
    }
}

/// Role selector dropdown
pub fn select_role(label: &str, mut role: Signal<String>) -> Element {
    rsx! {
        label { "{label}" }
        select {
            value: "{role.read()}",
            onchange: move |evt| role.set(evt.value().clone()),
            for option_name in ROLE_OPTIONS {
                option { value: "{option_name}", "{option_name}" }
            }
        }
    }
}

/// Status selector dropdown
pub fn select_status(label: &str, mut status: Signal<String>) -> Element {
    rsx! {
        label { "{label}" }
        select {
            value: "{status.read()}",
            onchange: move |evt| status.set(evt.value().clone()),
            for option_name in STATUS_OPTIONS {
                option { value: "{option_name}", "{option_name}" }
            }
        }
    }
}
