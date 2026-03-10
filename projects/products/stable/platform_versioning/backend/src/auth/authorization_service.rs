// projects/products/stable/platform_versioning/backend/src/auth/authorization_service.rs
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

fn next_nonce() -> u32 {
    static NONCE: AtomicU32 = AtomicU32::new(1);
    NONCE.fetch_add(1, Ordering::Relaxed)
}

use axum::http::HeaderMap;

use crate::auth::{
    AuditEntry, AuditLog, AuditOutcome, AuthToken, Permission, TokenClaims, TokenVerifier,
};
use crate::errors::PvError;
use crate::ids::RepoId;
use crate::indexes::SafePath;

/// Centralized authorization service for all platform-versioning endpoints.
///
/// Every permission check—repo-level, path-level, and issue-level—must go
/// through this service so that audit logging and enforcement are consistent
/// across all endpoints.
pub struct AuthorizationService {
    verifier: Arc<TokenVerifier>,
    audit_log: Arc<AuditLog>,
}

impl AuthorizationService {
    /// Creates a new service wrapping the given verifier and audit log.
    pub fn new(verifier: Arc<TokenVerifier>, audit_log: Arc<AuditLog>) -> Self {
        Self {
            verifier,
            audit_log,
        }
    }

    /// Returns a shared reference to the audit log.
    pub fn audit_log(&self) -> &Arc<AuditLog> {
        &self.audit_log
    }

    /// Returns `true` if `headers` contains a bearer token.
    pub fn has_bearer_token(headers: &HeaderMap) -> bool {
        headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .is_some_and(|s| s.starts_with("Bearer "))
    }

    fn now_secs() -> u64 {
        next_nonce() as u64
    }

    fn bearer_token(headers: &HeaderMap) -> Option<String> {
        let raw = headers.get("authorization")?.to_str().ok()?;
        raw.strip_prefix("Bearer ").map(|s| s.trim().to_string())
    }

    fn record(
        &self,
        subject: String,
        action: &str,
        repo_id: Option<RepoId>,
        outcome: AuditOutcome,
    ) {
        self.audit_log.record(AuditEntry {
            timestamp_secs: Self::now_secs(),
            subject,
            action: action.to_string(),
            repo_id,
            outcome,
        });
    }

    /// Verifies the bearer token and checks that the claims grant `permission` on `repo_id`.
    ///
    /// Records an audit entry for the attempt and returns the decoded claims on success.
    pub fn require_permission(
        &self,
        headers: &HeaderMap,
        repo_id: Option<&RepoId>,
        permission: Permission,
        action: &str,
    ) -> Result<TokenClaims, PvError> {
        let token_str = Self::bearer_token(headers)
            .ok_or_else(|| PvError::AuthRequired("missing bearer token".to_string()))?;

        let claims = self.verifier.verify(&AuthToken::new(token_str))?;

        if !claims.is_valid_at(Self::now_secs()) {
            self.record(
                claims.subject.clone(),
                action,
                repo_id.cloned(),
                AuditOutcome::Denied,
            );
            return Err(PvError::AuthRequired("token expired".to_string()));
        }

        let allowed = match repo_id {
            Some(repo) => claims.has_permission(repo, permission),
            None => {
                if let Ok(global_repo) = "global".parse::<RepoId>() {
                    claims.has_permission(&global_repo, permission)
                } else {
                    false
                }
            }
        };

        if !allowed {
            self.record(
                claims.subject.clone(),
                action,
                repo_id.cloned(),
                AuditOutcome::Denied,
            );
            return Err(PvError::PermissionDenied(format!(
                "permission '{permission:?}' required"
            )));
        }

        self.record(
            claims.subject.clone(),
            action,
            repo_id.cloned(),
            AuditOutcome::Allowed,
        );
        Ok(claims)
    }

    /// Checks whether `claims` can access `path` within `repo_id`.
    ///
    /// Returns `Ok(())` when no path grants restrict the repository (unrestricted)
    /// or when the path is covered by an allowed grant. Returns
    /// `Err(PvError::PermissionDenied)` when path grants exist and the path
    /// is not in the allowlist.
    pub fn check_path_access(
        &self,
        claims: &TokenClaims,
        repo_id: &RepoId,
        path: &SafePath,
    ) -> Result<(), PvError> {
        if claims.path_is_accessible(repo_id, path) {
            Ok(())
        } else {
            self.record(
                claims.subject.clone(),
                "path.denied",
                Some(repo_id.clone()),
                AuditOutcome::Denied,
            );
            Err(PvError::PermissionDenied(format!(
                "path '{}' is not in the allowed path list",
                path
            )))
        }
    }

    /// Issues a new signed token for the given claims.
    pub fn issue_token(&self, claims: &TokenClaims) -> Result<crate::auth::AuthToken, PvError> {
        self.verifier.issue(claims)
    }

    /// Verifies the bearer token and returns the decoded claims without
    /// checking any specific permission.
    ///
    /// Use this when any authenticated caller is allowed to reach an endpoint
    /// (visibility filtering is applied at the data layer, not the auth layer).
    pub fn authenticate(&self, headers: &HeaderMap, action: &str) -> Result<TokenClaims, PvError> {
        let token_str = Self::bearer_token(headers)
            .ok_or_else(|| PvError::AuthRequired("missing bearer token".to_string()))?;

        let claims = self.verifier.verify(&AuthToken::new(token_str))?;

        if !claims.is_valid_at(Self::now_secs()) {
            self.record(claims.subject.clone(), action, None, AuditOutcome::Denied);
            return Err(PvError::AuthRequired("token expired".to_string()));
        }

        Ok(claims)
    }

    /// Records a permission change event in the audit log.
    ///
    /// `actor` is the admin making the change; `target_subject` is the user whose
    /// permissions are being modified.
    pub fn record_permission_change(
        &self,
        actor: &str,
        target_subject: &str,
        repo_id: Option<RepoId>,
    ) {
        self.record(
            actor.to_string(),
            &format!("permission.change[target={target_subject}]"),
            repo_id,
            AuditOutcome::Allowed,
        );
    }

    /// Records a slice access event in the audit log.
    pub fn record_slice_access(&self, subject: &str, issue_id: &str, repo_id: Option<RepoId>) {
        self.record(
            subject.to_string(),
            &format!("slice.access[issue={issue_id}]"),
            repo_id,
            AuditOutcome::Allowed,
        );
    }

    /// Records a slice creation event in the audit log.
    pub fn record_slice_created(&self, actor: &str, issue_id: &str, repo_id: Option<RepoId>) {
        self.record(
            actor.to_string(),
            &format!("slice.created[issue={issue_id}]"),
            repo_id,
            AuditOutcome::Allowed,
        );
    }
}
