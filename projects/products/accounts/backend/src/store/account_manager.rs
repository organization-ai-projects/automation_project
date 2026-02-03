// projects/products/accounts/backend/src/store/account_manager.rs
use common_json::{from_json_str, to_string};
use common_time::timestamp_utils::current_timestamp_ms;
use security::{Permission, Role};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use std::sync::atomic::{AtomicBool, Ordering};
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

#[cfg(test)]
mod tests {
    use super::*;
    use security::Role;
    use std::sync::atomic::Ordering;
    use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};

    // Shared counter for unique test directory names
    static TEST_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn create_unique_temp_dir() -> PathBuf {
        let id = TEST_DIR_COUNTER.fetch_add(1, AtomicOrdering::Relaxed);
        std::env::temp_dir().join(format!("accounts_test_{}_{}", current_timestamp_ms(), id))
    }

    async fn create_test_manager() -> AccountManager {
        let temp_dir = create_unique_temp_dir();
        tokio::fs::create_dir_all(&temp_dir).await.unwrap();
        AccountManager::load(temp_dir).await.unwrap()
    }

    #[tokio::test]
    async fn test_login_sets_dirty_flag() {
        let manager = create_test_manager().await;
        let user_id = ProtocolId::default();
        
        // Create a test user
        manager.create(user_id, "test_password", Role::User, vec![], "test_actor")
            .await
            .unwrap();
        
        // Clear dirty flag after create
        manager.dirty.store(false, Ordering::Relaxed);
        assert!(!manager.dirty.load(Ordering::Relaxed), "Dirty flag should be false initially");
        
        // Authenticate (login)
        manager.authenticate(&user_id, "test_password").await.unwrap();
        
        // Check that dirty flag is set
        assert!(manager.dirty.load(Ordering::Relaxed), "Dirty flag should be true after login");
        
        // Cleanup
        tokio::fs::remove_dir_all(manager.data_dir()).await.ok();
    }

    #[tokio::test]
    async fn test_flush_if_dirty_saves_data() {
        let manager = create_test_manager().await;
        let user_id = ProtocolId::default();
        
        // Create a test user
        manager.create(user_id, "test_password", Role::User, vec![], "test_actor")
            .await
            .unwrap();
        
        // Authenticate to update last_login_ms
        manager.authenticate(&user_id, "test_password").await.unwrap();
        
        // Get last_login_ms before flush
        let user_before = manager.get(&user_id).await.unwrap();
        assert!(user_before.last_login_ms.is_some(), "last_login_ms should be set");
        let login_time = user_before.last_login_ms.unwrap();
        
        // Flush the dirty data
        assert!(manager.dirty.load(Ordering::Relaxed), "Should be dirty before flush");
        manager.flush_if_dirty().await.unwrap();
        assert!(!manager.dirty.load(Ordering::Relaxed), "Should not be dirty after flush");
        
        // Reload from disk
        let data_dir = manager.data_dir().clone();
        drop(manager);
        let reloaded = AccountManager::load(data_dir.clone()).await.unwrap();
        
        // Verify last_login_ms persisted
        let user_after = reloaded.get(&user_id).await.unwrap();
        assert_eq!(user_after.last_login_ms, Some(login_time), "last_login_ms should persist across reload");
        
        // Cleanup
        tokio::fs::remove_dir_all(&data_dir).await.ok();
    }

    #[tokio::test]
    async fn test_flush_if_dirty_skips_when_clean() {
        let manager = create_test_manager().await;
        
        // Ensure dirty flag is false
        manager.dirty.store(false, Ordering::Relaxed);
        
        // Call flush_if_dirty when clean - should not error
        manager.flush_if_dirty().await.unwrap();
        
        // Cleanup
        tokio::fs::remove_dir_all(manager.data_dir()).await.ok();
    }

    #[tokio::test]
    async fn test_last_login_survives_restart() {
        let temp_dir = create_unique_temp_dir();
        tokio::fs::create_dir_all(&temp_dir).await.unwrap();
        
        let user_id = ProtocolId::default();
        
        // First session: create user and login
        {
            let manager = AccountManager::load(temp_dir.clone()).await.unwrap();
            manager.create(user_id, "test_password", Role::User, vec![], "test_actor")
                .await
                .unwrap();
            
            manager.authenticate(&user_id, "test_password").await.unwrap();
            let user = manager.get(&user_id).await.unwrap();
            assert!(user.last_login_ms.is_some(), "last_login_ms should be set after login");
            
            // Flush to disk (simulate periodic flush)
            manager.flush_if_dirty().await.unwrap();
        }
        
        // Second session: reload and verify persistence
        {
            let manager = AccountManager::load(temp_dir.clone()).await.unwrap();
            let user = manager.get(&user_id).await.unwrap();
            assert!(user.last_login_ms.is_some(), "last_login_ms should survive restart after flush");
        }
        
        // Cleanup
        tokio::fs::remove_dir_all(&temp_dir).await.ok();
    }
}
