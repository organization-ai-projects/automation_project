// projects/products/accounts/ui/src/user_actions.rs
use dioxus::prelude::*;
use gloo_net::http::Request;
use protocol::accounts::{
    AccountSummary, AccountsListResponse, CreateAccountRequest, ResetPasswordRequest,
    UpdateAccountRequest, UpdateStatusRequest,
};

/// Reload users list from API with authentication token
pub fn reload_users_with_token(
    token: String,
    mut users: Signal<Vec<AccountSummary>>,
    mut msg: Signal<String>,
) {
    spawn(async move {
        match Request::get("/api/accounts/users")
            .header("authorization", &format!("Bearer {token}"))
            .send()
            .await
        {
            Ok(resp) => {
                let status = resp.status();
                let text = resp.text().await.unwrap_or_default();
                if status == 200 {
                    if let Ok(list) = common_json::from_json_str::<AccountsListResponse>(&text) {
                        users.set(list.users);
                    } else {
                        msg.set("Failed to parse users list".to_string());
                    }
                } else {
                    msg.set(format!("status {}: {}", status, text));
                }
            }
            Err(err) => msg.set(format!("request failed: {err}")),
        }
    });
}

/// Create a new user account
pub fn create_user_action(
    jwt: Signal<Option<String>>,
    user_id: Signal<String>,
    password: Signal<String>,
    role: Signal<String>,
    permissions: Signal<Vec<String>>,
    users: Signal<Vec<AccountSummary>>,
    mut msg: Signal<String>,
) {
    let Some(token) = jwt.read().clone() else {
        msg.set("Not authenticated".to_string());
        return;
    };
    let body = CreateAccountRequest {
        user_id: user_id.read().clone(),
        password: password.read().clone(),
        role: role.read().clone(),
        permissions: permissions.read().clone(),
    };
    let payload = common_json::to_string(&body).unwrap_or_else(|_| "{}".to_string());
    spawn(async move {
        let request = match Request::post("/api/accounts/users")
            .header("authorization", &format!("Bearer {token}"))
            .header("Content-Type", "application/json")
            .body(payload)
        {
            Ok(r) => r,
            Err(e) => {
                msg.set(format!("Failed to create request: {e}"));
                return;
            }
        };
        match request.send().await {
            Ok(resp) => {
                let status = resp.status();
                let text = resp.text().await.unwrap_or_default();
                if status == 201 {
                    msg.set("User created".to_string());
                    reload_users_with_token(token, users, msg);
                } else {
                    msg.set(format!("status {}: {}", status, text));
                }
            }
            Err(err) => msg.set(format!("request failed: {err}")),
        }
    });
}

/// Update user role and permissions
pub fn update_user_action(
    jwt: Signal<Option<String>>,
    user_id: Signal<String>,
    role: Signal<String>,
    permissions: Signal<Vec<String>>,
    users: Signal<Vec<AccountSummary>>,
    mut msg: Signal<String>,
) {
    let Some(token) = jwt.read().clone() else {
        msg.set("Not authenticated".to_string());
        return;
    };
    let body = UpdateAccountRequest {
        role: Some(role.read().clone()),
        permissions: Some(permissions.read().clone()),
    };
    let payload = common_json::to_string(&body).unwrap_or_else(|_| "{}".to_string());
    let user_id_val = user_id.read().clone();
    spawn(async move {
        let request = match Request::patch(&format!("/api/accounts/users/{user_id_val}"))
            .header("authorization", &format!("Bearer {token}"))
            .header("Content-Type", "application/json")
            .body(payload)
        {
            Ok(r) => r,
            Err(e) => {
                msg.set(format!("Failed to create request: {e}"));
                return;
            }
        };
        match request.send().await {
            Ok(resp) => {
                let status = resp.status();
                let text = resp.text().await.unwrap_or_default();
                if status == 200 {
                    msg.set("User updated".to_string());
                    reload_users_with_token(token, users, msg);
                } else {
                    msg.set(format!("status {}: {}", status, text));
                }
            }
            Err(err) => msg.set(format!("request failed: {err}")),
        }
    });
}

/// Update user account status
pub fn update_status_action(
    jwt: Signal<Option<String>>,
    user_id: Signal<String>,
    status: Signal<String>,
    users: Signal<Vec<AccountSummary>>,
    mut msg: Signal<String>,
) {
    let Some(token) = jwt.read().clone() else {
        msg.set("Not authenticated".to_string());
        return;
    };
    let body = UpdateStatusRequest {
        status: status.read().clone(),
    };
    let payload = common_json::to_string(&body).unwrap_or_else(|_| "{}".to_string());
    let user_id_val = user_id.read().clone();
    spawn(async move {
        let request = match Request::post(&format!("/api/accounts/users/{user_id_val}/status"))
            .header("authorization", &format!("Bearer {token}"))
            .header("Content-Type", "application/json")
            .body(payload)
        {
            Ok(r) => r,
            Err(e) => {
                msg.set(format!("Failed to create request: {e}"));
                return;
            }
        };
        match request.send().await {
            Ok(resp) => {
                let status_code = resp.status();
                let text = resp.text().await.unwrap_or_default();
                if status_code == 200 {
                    msg.set("Status updated".to_string());
                    reload_users_with_token(token, users, msg);
                } else {
                    msg.set(format!("status {}: {}", status_code, text));
                }
            }
            Err(err) => msg.set(format!("request failed: {err}")),
        }
    });
}

/// Reset user password
pub fn reset_password_action(
    jwt: Signal<Option<String>>,
    user_id: Signal<String>,
    password: Signal<String>,
    mut msg: Signal<String>,
) {
    let Some(token) = jwt.read().clone() else {
        msg.set("Not authenticated".to_string());
        return;
    };
    let body = ResetPasswordRequest {
        password: password.read().clone(),
    };
    let payload = common_json::to_string(&body).unwrap_or_else(|_| "{}".to_string());
    let user_id_val = user_id.read().clone();
    spawn(async move {
        let request =
            match Request::post(&format!("/api/accounts/users/{user_id_val}/reset_password"))
                .header("authorization", &format!("Bearer {token}"))
                .header("Content-Type", "application/json")
                .body(payload)
            {
                Ok(r) => r,
                Err(e) => {
                    msg.set(format!("Failed to create request: {e}"));
                    return;
                }
            };
        match request.send().await {
            Ok(resp) => {
                let status = resp.status();
                let text = resp.text().await.unwrap_or_default();
                if status == 200 {
                    msg.set("Password reset".to_string());
                } else {
                    msg.set(format!("status {}: {}", status, text));
                }
            }
            Err(err) => msg.set(format!("request failed: {err}")),
        }
    });
}
