// projects/products/accounts/ui/src/card_components.rs
use dioxus::prelude::*;
use gloo_net::http::Request;
use protocol::accounts::{AccountSummary, LoginRequest, LoginResponse};

use crate::{
    components::form_components::{input_field, input_password},
    parse_json::format_ts,
    setup_admin_input::SetupAdminInput,
    user_actions::reload_users_with_token,
};

/// Setup admin card for initial account creation
pub fn setup_card(
    setup_user: Signal<String>,
    setup_pass: Signal<String>,
    mut setup_msg: Signal<String>,
    mut setup_mode: Signal<bool>,
) -> Element {
    rsx! {
        section { class: "card",
            h2 { "Create first admin" }
            {input_field("User ID (32 hex chars)", setup_user, false)}
            {input_password("Password", setup_pass)}
            button {
                class: "primary",
                onclick: move |_| {
                    let user_id = setup_user.read().clone();
                    let password = setup_pass.read().clone();
                    spawn(async move {
                        let body = SetupAdminInput { user_id, password };
                        let payload =
                            common_json::to_string(&body).unwrap_or_else(|_| "{}".to_string());
                        let request = match Request::post("/api/setup/admin")
                            .header("Content-Type", "application/json")
                            .body(payload) {
                            Ok(r) => r,
                            Err(e) => {
                                setup_msg.set(format!("Failed to create request: {e}"));
                                return;
                            }
                        };
                        match request.send().await {
                            Ok(resp) => {
                                let status = resp.status();
                                let text = resp.text().await.unwrap_or_default();
                                setup_msg.set(format!("status {}: {}", status, text));
                                if status == 201 {
                                    setup_mode.set(false);
                                }
                            }
                            Err(err) => {
                                setup_msg.set(format!("request failed: {err}"));
                            }
                        }
                    });
                },
                "Create admin"
            }
            p { class: "status", "{setup_msg.read()}" }
        }
    }
}

/// Login card for user authentication
pub fn login_card(
    login_user: Signal<String>,
    login_pass: Signal<String>,
    mut login_msg: Signal<String>,
    mut jwt: Signal<Option<String>>,
    users: Signal<Vec<AccountSummary>>,
) -> Element {
    rsx! {
        section { class: "card",
            h2 { "Login" }
            {input_field("User ID (32 hex chars)", login_user, false)}
            {input_password("Password", login_pass)}
            button {
                class: "primary",
                onclick: move |_| {
                    let user_id = login_user.read().clone();
                    let password = login_pass.read().clone();
                    spawn(async move {
                        let body = LoginRequest {
                            user_id,
                            password,
                            role: None,
                            duration_ms: None,
                            session_id: None,
                        };
                        let payload = common_json::to_string(&body).unwrap_or_else(|_| "{}".to_string());
                        let request = match Request::post("/api/login")
                            .header("Content-Type", "application/json")
                            .body(payload) {
                            Ok(r) => r,
                            Err(e) => {
                                login_msg.set(format!("Failed to create request: {e}"));
                                return;
                            }
                        };
                        match request.send().await {
                            Ok(resp) => {
                                let status = resp.status();
                                let text = resp.text().await.unwrap_or_default();
                                if status == 200 {
                                    if let Ok(login_resp) = common_json::from_json_str::<LoginResponse>(&text) {
                                        jwt.set(Some(login_resp.jwt.clone()));
                                        login_msg.set("Login ok".to_string());
                                        reload_users_with_token(login_resp.jwt, users, login_msg);
                                    } else {
                                        login_msg.set("Failed to parse login response".to_string());
                                    }
                                } else {
                                    login_msg.set(format!("status {}: {}", status, text));
                                }
                            }
                            Err(err) => {
                                login_msg.set(format!("request failed: {err}"));
                            }
                        }
                    });
                },
                "Login"
            }
            p { class: "status", "{login_msg.read()}" }
        }
    }
}

/// Users table displaying all accounts
pub fn users_table(users: Signal<Vec<AccountSummary>>) -> Element {
    rsx! {
        table { class: "table",
            thead {
                tr {
                    th { "User ID" }
                    th { "Role" }
                    th { "Status" }
                    th { "Permissions" }
                    th { "Last login" }
                }
            }
            tbody {
                for user in users.read().iter() {
                    {
                        let perms = user.permissions.join(", ");
                        rsx! {
                            tr {
                                td { "{user.user_id}" }
                                td { "{user.role}" }
                                td { "{user.status}" }
                                td { "{perms}" }
                                td { "{format_ts(user.last_login_ms)}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
