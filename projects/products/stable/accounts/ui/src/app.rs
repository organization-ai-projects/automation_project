// projects/products/stable/accounts/ui/src/app.rs
use dioxus::prelude::*;
use protocol::accounts::AccountSummary;

use crate::components::card_components::{login_card, setup_card, users_table};
use crate::components::form_components::{input_field, input_password, select_role, select_status};
use crate::parse_json::parse_json;
use crate::permission_picker::permission_picker;
use crate::user_actions::{create_user_action, reset_password_action, update_user_action};

/// Main application component with setup, login, and user management
#[component]
pub fn app() -> Element {
    let mut setup_mode = use_signal(|| false);
    let mut jwt = use_signal(|| Option::<String>::None);
    let mut users = use_signal(|| Vec::<AccountSummary>::new());

    // Form state
    let mut user_id = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut role = use_signal(|| "user".to_string());
    let mut permissions = use_signal(|| Vec::new());
    let mut status_filter = use_signal(|| "active".to_string());
    let mut msg = use_signal(|| String::new());

    // Initialize on mount
    use_effect(move || {
        spawn(async move {
            match gloo_net::http::Request::get("/api/accounts/setup_status")
                .send()
                .await
            {
                Ok(resp) => match parse_json::<serde_json::Value>(resp).await {
                    Ok(status) => {
                        let needs_setup = status
                            .get("needs_setup")
                            .and_then(|value| value.as_bool())
                            .unwrap_or(false);
                        if needs_setup {
                            setup_mode.set(true);
                        }
                    }
                    Err(err) => {
                        msg.set(format!("Failed to parse setup status: {err}"));
                    }
                },
                Err(err) => {
                    msg.set(format!("Failed to check setup status: {err}"));
                }
            }
        });
    });

    rsx! {
        style { {
            r#"
            body {{ font-family: sans-serif; margin: 0; padding: 20px; }}
            .container {{ max-width: 800px; margin: 0 auto; }}
            .card {{ border: 1px solid #ddd; border-radius: 8px; padding: 20px; margin: 10px 0; }}
            button {{ padding: 8px 16px; border: none; border-radius: 4px;
                    background-color: #0066cc; color: white; cursor: pointer; }}
            button:hover {{ background-color: #0052a3; }}
            input, select {{ padding: 8px; margin: 5px; border: 1px solid #ccc; border-radius: 4px; }}
            .error {{ color: #cc0000; margin: 10px 0; }}
            .success {{ color: #00aa00; margin: 10px 0; }}
            table {{ width: 100%; border-collapse: collapse; }}
            th, td {{ border: 1px solid #ddd; padding: 10px; text-align: left; }}
            th {{ background-color: #f0f0f0; }}
            .pill {{ display: inline-block; background-color: #e0e0e0; border-radius: 20px;
                    padding: 4px 12px; margin: 2px; }}
            .pill.selected {{ background-color: #0066cc; color: white; }}
            "#
        } }

        div {
            class: "container",

            if setup_mode.read().clone() {
                {setup_card(user_id, password, msg, setup_mode)}
            } else if jwt.read().is_none() {
                {login_card(user_id, password, msg, jwt, users)}
            } else {
                // User management dashboard
                div {
                    key: "dashboard",

                    h1 { "User Management" }

                    // Create new user form
                    div {
                        class: "card",
                        h2 { "Create New User" }

                        {input_field("User ID", user_id, false)}
                        {input_password("Password", password)}
                        {select_role("Role", role)}

                        div {
                            h3 { "Permissions" }
                            {permission_picker(permissions)}
                        }

                        button {
                            onclick: move |_| {
                                create_user_action(
                                    jwt,
                                    user_id,
                                    password,
                                    role,
                                    permissions,
                                    users,
                                    msg,
                                );
                                user_id.set(String::new());
                                password.set(String::new());
                                permissions.set(Vec::new());
                            },
                            "Create User"
                        }
                    }

                    // Users table
                    div {
                        class: "card",
                        h2 { "Users" }

                        {select_status("Filter by status", status_filter)}

                        {users_table(users)}
                    }

                    // User edit form (when a user is selected)
                    if !user_id.read().is_empty() {
                        div {
                            class: "card",
                            h2 { "Edit User: {user_id}" }

                            {input_field("User ID (read-only)", user_id, true)}
                            {input_password("New Password", password)}
                            {select_role("Role", role)}

                            div {
                                h3 { "Permissions" }
                                {permission_picker(permissions)}
                            }

                            button {
                                onclick: move |_| {
                                    update_user_action(
                                        jwt,
                                        user_id,
                                        role,
                                        permissions,
                                        users,
                                        msg,
                                    );
                                },
                                "Update User"
                            }

                            if !password.read().is_empty() {
                                button {
                                    onclick: move |_| {
                                        reset_password_action(jwt, user_id, password, msg);
                                        password.set(String::new());
                                    },
                                    "Reset Password"
                                }
                            }

                            button {
                                onclick: move |_| {
                                    user_id.set(String::new());
                                    password.set(String::new());
                                    permissions.set(Vec::new());
                                },
                                "Clear Selection"
                            }
                        }
                    }
                }
            }

            // Message display
            if !msg.read().is_empty() {
                div {
                    class: if msg.read().contains("error") || msg.read().contains("failed")
                        || msg.read().contains("status")
                    {
                        "error"
                    } else {
                        "success"
                    },
                    "{msg}"
                }
            }
        }
    }
}
