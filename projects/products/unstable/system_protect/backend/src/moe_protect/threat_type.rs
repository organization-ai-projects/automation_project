use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatType {
    Virus,
    Malware,
    Trojan,
    Ransomware,
    Worm,
    NetworkIntrusion,
    DenialOfService,
    UnauthorizedAccess,
    DataExfiltration,
    PortScan,
    BruteForce,
    Custom(String),
}
