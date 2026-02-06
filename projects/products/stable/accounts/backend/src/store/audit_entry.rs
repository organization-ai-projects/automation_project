// projects/products/accounts/backend/src/store/audit_entry.rs

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuditEntry {
    pub timestamp_ms: u64,
    pub actor: String,
    pub action: String,
    pub target: String,
    pub details: Option<String>,
}
