// projects/libraries/security/src/claims.rs
use serde::{Deserialize, Serialize};

use crate::Role;
use protocol::ProtocolId;

/// JWT claims (standard-ish).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject identifier
    pub sub: ProtocolId,
    /// JWT ID = UUIDv7 (unique token id)
    pub jti: ProtocolId,
    /// Role
    pub role: Role,
    /// Issued-at (seconds since epoch)
    pub iat: u64,
    /// Expiration (seconds since epoch)
    pub exp: u64,
    /// Optional session id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<ProtocolId>,
}
