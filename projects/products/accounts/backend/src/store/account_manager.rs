// projects/products/accounts/backend/src/store/account_manager.rs
use common_json::{from_json_str, to_string};
use common_time::timestamp_utils::current_timestamp_ms;
use security::{Permission, Role};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;

use crate::store::account_record::AccountRecord;
use crate::store::account_store_error::AccountStoreError;
use crate::store::accounts_file::AccountsFile;
use crate::store::audit_entry::AuditEntry;
use protocol::ProtocolId;
use protocol::accounts::AccountStatus;
use protocol::accounts::AccountSummary;

#[derive(Clone)]
pub struct AccountManager {
    data_dir: PathBuf,
    accounts_path: PathBuf,
    audit_path: PathBuf,
    state: Arc<RwLock<HashMap<ProtocolId, AccountRecord>>>,
    dirty: Arc<AtomicBool>,
}

impl AccountManager {
    pub fn default_data_dir() -> PathBuf {
        PathBuf::from("projects")
            .join("products")
            .join("accounts")
            .join("data")
    }

    pub async fn load(data_dir: PathBuf) -> Result<Self, AccountStoreError> {
        if !data_dir.exists() {
            tokio::fs::create_dir_all(&data_dir).await?;
        }

        let accounts_path = data_dir.join("accounts.json");
        let audit_path = data_dir.join("audit.log");

        let users = if accounts_path.exists() {
            let raw = tokio::fs::read_to_string(&accounts_path).await?;
            if raw.trim().is_empty() {
                HashMap::new()
            } else {
                let parsed: AccountsFile =
                    from_json_str(&raw).map_err(|e| AccountStoreError::Json(e.to_string()))?;
                parsed.users.into_iter().map(|u| (u.user_id, u)).collect()
            }
        } else {
            HashMap::new()
        };

        Ok(Self {
            data_dir,
            accounts_path,
            audit_path,
            state: Arc::new(RwLock::new(users)),
            dirty: Arc::new(AtomicBool::new(false)),
        })
    }

    pub async fn save(&self) -> Result<(), AccountStoreError> {
        let users = self.state.read().await;
        let file = AccountsFile {
            schema_version: 1,
            users: users.values().cloned().collect(),
        };
        let out = to_string(&file).map_err(|e| AccountStoreError::Json(e.to_string()))?;
        tokio::fs::write(&self.accounts_path, out).await?;
        self.dirty.store(false, Ordering::Relaxed);
        Ok(())
    }

    pub async fn flush_if_dirty(&self) -> Result<(), AccountStoreError> {
        if self.dirty.load(Ordering::Relaxed) {
            self.save().await?;
        }
        Ok(())
    }

    pub fn data_dir(&self) -> &PathBuf {
        &self.data_dir
    }

    #[cfg(test)]
    pub(crate) fn is_dirty(&self) -> bool {
        self.dirty.load(Ordering::Relaxed)
    }

    #[cfg(test)]
    pub(crate) fn set_dirty(&self, value: bool) {
        self.dirty.store(value, Ordering::Relaxed);
    }

    pub async fn user_count(&self) -> usize {
        self.state.read().await.len()
    }

    pub async fn list(&self) -> Vec<AccountSummary> {
        let users = self.state.read().await;
        users
            .values()
            .map(|u| AccountSummary {
                user_id: u.user_id,
                role: u.role.to_string(),
                permissions: self
                    .effective_permissions(u)
                    .iter()
                    .map(|p| p.to_string())
                    .collect(),
                status: u.status.to_string(),
                created_at_ms: u.created_at_ms,
                updated_at_ms: u.updated_at_ms,
                last_login_ms: u.last_login_ms,
            })
            .collect()
    }

    pub async fn get(&self, user_id: &ProtocolId) -> Result<AccountSummary, AccountStoreError> {
        let users = self.state.read().await;
        let user = users.get(user_id).ok_or(AccountStoreError::NotFound)?;
        Ok(AccountSummary {
            user_id: user.user_id,
            role: user.role.to_string(),
            permissions: self
                .effective_permissions(user)
                .iter()
                .map(|p| p.to_string())
                .collect(),
            status: user.status.to_string(),
            created_at_ms: user.created_at_ms,
            updated_at_ms: user.updated_at_ms,
            last_login_ms: user.last_login_ms,
        })
    }

    pub async fn create(
        &self,
        user_id: ProtocolId,
        password: &str,
        role: Role,
        extra_permissions: Vec<Permission>,
        actor: &str,
    ) -> Result<(), AccountStoreError> {
        if password.trim().is_empty() {
            return Err(AccountStoreError::InvalidPassword);
        }

        let mut users = self.state.write().await;
        if users.contains_key(&user_id) {
            return Err(AccountStoreError::AlreadyExists);
        }

        let hash = security::password::hash_password(password)
            .map_err(|e| AccountStoreError::Password(e.to_string()))?;
        let now = current_timestamp_ms();

        let record = AccountRecord {
            user_id,
            password_hash: hash,
            role,
            extra_permissions,
            status: AccountStatus::Active,
            created_at_ms: now,
            updated_at_ms: now,
            last_login_ms: None,
        };

        users.insert(user_id, record);
        drop(users);

        self.save().await?;
        self.append_audit(AuditEntry {
            timestamp_ms: now,
            actor: actor.to_string(),
            action: "create".to_string(),
            target: user_id.to_string(),
            details: None,
        })
        .await?;

        Ok(())
    }

    pub async fn reset_password(
        &self,
        user_id: &ProtocolId,
        password: &str,
        actor: &str,
    ) -> Result<(), AccountStoreError> {
        if password.trim().is_empty() {
            return Err(AccountStoreError::InvalidPassword);
        }

        let mut users = self.state.write().await;
        let user = users.get_mut(user_id).ok_or(AccountStoreError::NotFound)?;
        let hash = security::password::hash_password(password)
            .map_err(|e| AccountStoreError::Password(e.to_string()))?;
        user.password_hash = hash;
        user.updated_at_ms = current_timestamp_ms();
        drop(users);

        self.save().await?;
        self.append_audit(AuditEntry {
            timestamp_ms: current_timestamp_ms(),
            actor: actor.to_string(),
            action: "reset_password".to_string(),
            target: user_id.to_string(),
            details: None,
        })
        .await?;
        Ok(())
    }

    pub async fn update_role_permissions(
        &self,
        user_id: &ProtocolId,
        role: Option<Role>,
        permissions: Option<Vec<Permission>>,
        actor: &str,
    ) -> Result<(), AccountStoreError> {
        let mut users = self.state.write().await;
        let user = users.get_mut(user_id).ok_or(AccountStoreError::NotFound)?;
        if let Some(role) = role {
            user.role = role;
        }
        if let Some(perms) = permissions {
            user.extra_permissions = perms;
        }
        user.updated_at_ms = current_timestamp_ms();
        drop(users);

        self.save().await?;
        self.append_audit(AuditEntry {
            timestamp_ms: current_timestamp_ms(),
            actor: actor.to_string(),
            action: "update_role_permissions".to_string(),
            target: user_id.to_string(),
            details: None,
        })
        .await?;
        Ok(())
    }

    pub async fn update_status(
        &self,
        user_id: &ProtocolId,
        status: AccountStatus,
        actor: &str,
    ) -> Result<(), AccountStoreError> {
        let mut users = self.state.write().await;
        let user = users.get_mut(user_id).ok_or(AccountStoreError::NotFound)?;
        user.status = status;
        user.updated_at_ms = current_timestamp_ms();
        drop(users);

        self.save().await?;
        self.append_audit(AuditEntry {
            timestamp_ms: current_timestamp_ms(),
            actor: actor.to_string(),
            action: format!("status:{}", status.as_str()),
            target: user_id.to_string(),
            details: None,
        })
        .await?;
        Ok(())
    }

    pub async fn authenticate(
        &self,
        user_id: &ProtocolId,
        password: &str,
    ) -> Result<Role, AccountStoreError> {
        let mut users = self.state.write().await;
        let user = users
            .get_mut(user_id)
            .ok_or(AccountStoreError::InvalidCredentials)?;

        if user.status != AccountStatus::Active {
            return Err(AccountStoreError::InvalidCredentials);
        }

        let ok = security::password::verify_password(password, &user.password_hash)
            .map_err(|e| AccountStoreError::Password(e.to_string()))?;
        if !ok {
            return Err(AccountStoreError::InvalidCredentials);
        }

        let login_ts = current_timestamp_ms();
        user.last_login_ms = Some(login_ts);
        let role = user.role;
        drop(users);

        // Mark accounts as dirty to trigger batched persistence
        self.dirty.store(true, Ordering::Relaxed);

        self.append_audit(AuditEntry {
            timestamp_ms: login_ts,
            actor: user_id.to_string(),
            action: "login".to_string(),
            target: user_id.to_string(),
            details: None,
        })
        .await?;
        Ok(role)
    }

    fn effective_permissions(&self, user: &AccountRecord) -> Vec<Permission> {
        let mut perms = user.role.permissions().to_vec();
        for perm in &user.extra_permissions {
            if !perms.contains(perm) {
                perms.push(*perm);
            }
        }
        perms
    }

    async fn append_audit(&self, entry: AuditEntry) -> Result<(), AccountStoreError> {
        let line = to_string(&entry).map_err(|e| AccountStoreError::Json(e.to_string()))?;
        let mut payload = line;
        payload.push('\n');
        tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.audit_path)
            .await?
            .write_all(payload.as_bytes())
            .await?;
        Ok(())
    }
}
