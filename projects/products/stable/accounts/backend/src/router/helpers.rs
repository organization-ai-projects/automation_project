// projects/products/stable/accounts/backend/src/router/helpers.rs
use std::str::FromStr;

use common_json::{Json, JsonMap, from_value, to_json_string, to_value};
use protocol::{Command, Event, EventType, EventVariant, Metadata, Payload, ProtocolId};
use security::Permission;

use crate::store::account_store_error::AccountStoreError;

pub fn payload_as<T>(cmd: &Command) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    let payload = payload_value(cmd)?;
    from_value(payload).map_err(|e| e.to_string())
}

pub fn payload_value(cmd: &Command) -> Result<Json, String> {
    cmd.payload
        .as_ref()
        .and_then(|p| p.payload.clone())
        .ok_or_else(|| "Missing payload".to_string())
}

pub fn get_user_id(cmd: &Command) -> Result<ProtocolId, String> {
    let payload = payload_value(cmd)?;
    let map = match payload {
        Json::Object(map) => map,
        _ => return Err("Expected payload object".to_string()),
    };

    match map.get("user_id") {
        Some(Json::String(id)) if !id.trim().is_empty() => id
            .parse::<ProtocolId>()
            .map_err(|_| "Invalid user_id".to_string()),
        _ => Err("Missing user_id".to_string()),
    }
}

pub fn parse_permissions(values: &[String]) -> Result<Vec<Permission>, String> {
    let mut perms = Vec::new();
    for value in values {
        let perm =
            Permission::from_str(value).map_err(|_| format!("Invalid permission: {value}"))?;
        perms.push(perm);
    }
    Ok(perms)
}

pub fn ok_payload_json() -> Json {
    let mut map = JsonMap::new();
    map.insert("ok".to_string(), Json::Bool(true));
    Json::Object(map)
}

pub fn ok_payload<T: serde::Serialize>(
    meta: &Metadata,
    name: &str,
    payload_type: &str,
    value: T,
) -> Event {
    let payload_value = to_value(&value).unwrap_or(Json::Object(JsonMap::new()));
    let payload = Payload {
        payload_type: Some(payload_type.to_string()),
        payload: Some(payload_value.clone()),
    };

    Event {
        name: name.to_string(),
        event_type: EventType::Payload,
        data: to_json_string(&payload_value).unwrap_or_else(|_| "{}".to_string()),
        metadata: meta.clone(),
        payload: Some(payload),
        level: None,
        message: None,
        pct: None,
        variant: EventVariant::Default,
    }
}

pub fn err_event(meta: &Metadata, status: u16, message: &str) -> Event {
    let mut map = JsonMap::new();
    map.insert("status".to_string(), common_json::number_u64(status as u64));
    map.insert("message".to_string(), Json::String(message.to_string()));
    let payload_value = Json::Object(map);
    let payload = Payload {
        payload_type: Some("accounts/error".to_string()),
        payload: Some(payload_value.clone()),
    };

    Event {
        name: "AccountsError".to_string(),
        event_type: EventType::Error,
        data: to_json_string(&payload_value).unwrap_or_else(|_| "{}".to_string()),
        metadata: meta.clone(),
        payload: Some(payload),
        level: None,
        message: Some(message.to_string()),
        pct: None,
        variant: EventVariant::Error {
            id: common::Id128::new(0, None, None),
            message: message.to_string(),
        },
    }
}

pub fn map_store_error(meta: &Metadata, err: AccountStoreError) -> Event {
    match err {
        AccountStoreError::NotFound => err_event(meta, 404, "User not found"),
        AccountStoreError::AlreadyExists => err_event(meta, 409, "User already exists"),
        AccountStoreError::InvalidCredentials => err_event(meta, 401, "Invalid credentials"),
        AccountStoreError::InvalidPassword => err_event(meta, 400, "Invalid password"),
        AccountStoreError::InvalidRole => err_event(meta, 400, "Invalid role"),
        AccountStoreError::InvalidPermission => err_event(meta, 400, "Invalid permission"),
        AccountStoreError::InvalidStatus => err_event(meta, 400, "Invalid status"),
        AccountStoreError::Io(e) => err_event(meta, 500, &format!("IO error: {e}")),
        AccountStoreError::Json(e) => err_event(meta, 500, &format!("JSON error: {e}")),
        AccountStoreError::Password(e) => err_event(meta, 500, &format!("Password error: {e}")),
    }
}
