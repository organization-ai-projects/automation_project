// projects/products/accounts/ui/src/form_components.rs
use dioxus::prelude::*;

/// Text input field component
pub fn input_field(label: &str, mut value: Signal<String>) -> Element {
    rsx! {
        label { "{label}" }
        input {
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
pub fn select_role(mut role: Signal<String>) -> Element {
    rsx! {
        label { "Role" }
        select {
            value: "{role.read()}",
            onchange: move |evt| role.set(evt.value().clone()),
            option { value: "admin", "admin" }
            option { value: "moderator", "moderator" }
            option { value: "user", "user" }
            option { value: "guest", "guest" }
        }
    }
}

/// Status selector dropdown
pub fn select_status(mut status: Signal<String>) -> Element {
    rsx! {
        label { "Status" }
        select {
            value: "{status.read()}",
            onchange: move |evt| status.set(evt.value().clone()),
            option { value: "active", "active" }
            option { value: "suspended", "suspended" }
            option { value: "disabled", "disabled" }
        }
    }
}
