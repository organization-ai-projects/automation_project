#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("accounts-ui (wasm build only)");
}

#[cfg(target_arch = "wasm32")]
fn main() {
    accounts_web::run();
}

#[cfg(target_arch = "wasm32")]
mod accounts_web {
    use dioxus::prelude::*;
    use gloo_net::http::Request;
    use protocol::accounts::{
        AccountSummary, AccountsListResponse, CreateAccountRequest, LoginRequest, LoginResponse,
        ResetPasswordRequest, SetupStatusResponse, UpdateAccountRequest, UpdateStatusRequest,
    };
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    struct SetupAdminInput {
        user_id: String,
        password: String,
    }

    pub fn run() {
        dioxus::launch(app);
    }

    fn app() -> Element {
        let mut setup_mode = use_signal(|| None::<bool>);
        let mut jwt = use_signal(|| None::<String>);
        let mut users = use_signal(Vec::<AccountSummary>::new);

        let setup_msg = use_signal(String::new);
        let login_msg = use_signal(String::new);
        let admin_msg = use_signal(String::new);

        let setup_user = use_signal(String::new);
        let setup_pass = use_signal(String::new);

        let login_user = use_signal(String::new);
        let login_pass = use_signal(String::new);

        let create_user = use_signal(String::new);
        let create_pass = use_signal(String::new);
        let create_role = use_signal(|| String::from("user"));
        let create_permissions = use_signal(Vec::<String>::new);

        let update_user = use_signal(String::new);
        let update_role = use_signal(|| String::from("user"));
        let update_permissions = use_signal(Vec::<String>::new);

        let status_user = use_signal(String::new);
        let status_value = use_signal(|| String::from("active"));

        let reset_user = use_signal(String::new);
        let reset_pass = use_signal(String::new);

        use_resource(move || async move {
            if let Ok(resp) = Request::get("/api/setup/status").send().await {
                if let Ok(status) = parse_json::<SetupStatusResponse>(resp).await {
                    setup_mode.set(Some(status.setup_mode));
                    return;
                }
            }
            setup_mode.set(Some(false));
        });

        let in_setup = setup_mode.read().unwrap_or(false);
        let logged_in = jwt.read().is_some();

        rsx! {
            style { {include_str!("style.css")} }
            div { class: "page",
                header { class: "top",
                    h1 { "Accounts" }
                    if let Some(token) = jwt.read().as_ref() {
                        span { class: "token", "JWT: {short_token(token)}" }
                        button {
                            class: "ghost",
                            onclick: move |_| {
                                jwt.set(None);
                                users.set(Vec::new());
                            },
                            "Logout"
                        }
                    }
                }

                if in_setup {
                    {setup_card(setup_user, setup_pass, setup_msg, setup_mode)}
                } else if !logged_in {
                    {login_card(login_user, login_pass, login_msg, jwt, users)}
                } else {
                    section { class: "grid",
                        div { class: "card",
                            h2 { "Users" }
                            button {
                                class: "primary",
                                onclick: move |_| {
                                    if let Some(token) = jwt.read().clone() {
                                        reload_users_with_token(token, users, admin_msg);
                                    }
                                },
                                "Refresh"
                            }
                            {users_table(users)}
                        }
                        div { class: "card",
                            h2 { "Create user" }
                            {input_field("User ID (32 hex chars)", create_user)}
                            {input_password("Password", create_pass)}
                            {select_role(create_role)}
                            {permission_picker(create_permissions)}
                            button {
                                class: "primary",
                                onclick: move |_| create_user_action(
                                    jwt,
                                    create_user,
                                    create_pass,
                                    create_role,
                                    create_permissions,
                                    users,
                                    admin_msg,
                                ),
                                "Create"
                            }
                            p { class: "status", "{admin_msg.read()}" }
                        }
                        div { class: "card",
                            h2 { "Update role & permissions" }
                            {input_field("User ID", update_user)}
                            {select_role(update_role)}
                            {permission_picker(update_permissions)}
                            button {
                                class: "primary",
                                onclick: move |_| update_user_action(
                                    jwt,
                                    update_user,
                                    update_role,
                                    update_permissions,
                                    users,
                                    admin_msg,
                                ),
                                "Update"
                            }
                        }
                        div { class: "card",
                            h2 { "Update status" }
                            {input_field("User ID", status_user)}
                            {select_status(status_value)}
                            button {
                                class: "primary",
                                onclick: move |_| update_status_action(
                                    jwt,
                                    status_user,
                                    status_value,
                                    users,
                                    admin_msg,
                                ),
                                "Apply"
                            }
                        }
                        div { class: "card",
                            h2 { "Reset password" }
                            {input_field("User ID", reset_user)}
                            {input_password("New password", reset_pass)}
                            button {
                                class: "primary",
                                onclick: move |_| reset_password_action(
                                    jwt,
                                    reset_user,
                                    reset_pass,
                                    admin_msg,
                                ),
                                "Reset"
                            }
                        }
                    }
                }
            }
        }
    }

    fn setup_card(
        setup_user: Signal<String>,
        setup_pass: Signal<String>,
        mut setup_msg: Signal<String>,
        mut setup_mode: Signal<Option<bool>>,
    ) -> Element {
        rsx! {
            section { class: "card",
                h2 { "Create first admin" }
                {input_field("User ID (32 hex chars)", setup_user)}
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
                                        setup_mode.set(Some(false));
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

    fn login_card(
        login_user: Signal<String>,
        login_pass: Signal<String>,
        mut login_msg: Signal<String>,
        mut jwt: Signal<Option<String>>,
        users: Signal<Vec<AccountSummary>>,
    ) -> Element {
        rsx! {
            section { class: "card",
                h2 { "Login" }
                {input_field("User ID (32 hex chars)", login_user)}
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
                                    token_msg.set(format!("Failed to create request: {e}"));
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

    fn users_table(users: Signal<Vec<AccountSummary>>) -> Element {
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

    fn input_field(label: &str, mut value: Signal<String>) -> Element {
        rsx! {
            label { "{label}" }
            input {
                value: "{value.read()}",
                oninput: move |evt| value.set(evt.value().clone()),
            }
        }
    }

    fn input_password(label: &str, mut value: Signal<String>) -> Element {
        rsx! {
            label { "{label}" }
            input {
                r#type: "password",
                value: "{value.read()}",
                oninput: move |evt| value.set(evt.value().clone()),
            }
        }
    }

    fn select_role(mut role: Signal<String>) -> Element {
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

    fn select_status(mut status: Signal<String>) -> Element {
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

    fn permission_picker(mut selected: Signal<Vec<String>>) -> Element {
        let options = [
            "read",
            "write",
            "execute",
            "delete",
            "admin",
            "train",
            "view_logs",
            "configure_system",
        ];

        rsx! {
            label { "Extra permissions" }
            div { class: "pill-grid",
                for option in options.iter() {
                    {
                        let is_selected = selected.read().contains(&option.to_string());
                        let opt = option.to_string();
                        rsx! {
                            button {
                                class: if is_selected { "pill active" } else { "pill" },
                                onclick: move |_| {
                                    let mut next = selected.read().clone();
                                    if let Some(pos) = next.iter().position(|v| v == &opt) {
                                        next.remove(pos);
                                    } else {
                                        next.push(opt.clone());
                                    }
                                    selected.set(next);
                                },
                                "{option}"
                            }
                        }
                    }
                }
            }
        }
    }

    fn reload_users_with_token(
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
                        if let Ok(list) = common_json::from_json_str::<AccountsListResponse>(&text)
                        {
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

    fn create_user_action(
        jwt: Signal<Option<String>>,
        user_id: Signal<String>,
        password: Signal<String>,
        role: Signal<String>,
        permissions: Signal<Vec<String>>,
        users: Signal<Vec<AccountSummary>>,
        mut msg: Signal<String>,
    ) {
        let token = jwt.read().clone();
        if token.is_none() {
            msg.set("Not authenticated".to_string());
            return;
        }
        let token = token.unwrap();
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

    fn update_user_action(
        jwt: Signal<Option<String>>,
        user_id: Signal<String>,
        role: Signal<String>,
        permissions: Signal<Vec<String>>,
        users: Signal<Vec<AccountSummary>>,
        mut msg: Signal<String>,
    ) {
        let token = jwt.read().clone();
        if token.is_none() {
            msg.set("Not authenticated".to_string());
            return;
        }
        let token = token.unwrap();
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

    fn update_status_action(
        jwt: Signal<Option<String>>,
        user_id: Signal<String>,
        status: Signal<String>,
        users: Signal<Vec<AccountSummary>>,
        mut msg: Signal<String>,
    ) {
        let token = jwt.read().clone();
        if token.is_none() {
            msg.set("Not authenticated".to_string());
            return;
        }
        let token = token.unwrap();
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

    fn reset_password_action(
        jwt: Signal<Option<String>>,
        user_id: Signal<String>,
        password: Signal<String>,
        mut msg: Signal<String>,
    ) {
        let token = jwt.read().clone();
        if token.is_none() {
            msg.set("Not authenticated".to_string());
            return;
        }
        let token = token.unwrap();
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

    async fn parse_json<T: serde::de::DeserializeOwned>(
        resp: gloo_net::http::Response,
    ) -> Result<T, String> {
        let text = resp.text().await.map_err(|e| e.to_string())?;
        common_json::from_json_str(&text).map_err(|e| e.to_string())
    }

    fn short_token(token: &str) -> String {
        let keep = 8usize.min(token.len());
        format!("{}...", &token[..keep])
    }

    fn format_ts(ts: Option<u64>) -> String {
        ts.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string())
    }
}
