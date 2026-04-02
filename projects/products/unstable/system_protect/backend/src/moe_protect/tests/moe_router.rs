use crate::antivirus::antivirus_expert::AntivirusExpert;
use crate::firewall::firewall_expert::FirewallExpert;
use crate::moe_protect::expert::ProtectionExpert;
use crate::moe_protect::moe_router::MoeRouter;
use crate::moe_protect::threat_event::ThreatEvent;
use crate::moe_protect::threat_id::ThreatId;
use crate::moe_protect::threat_level::ThreatLevel;
use crate::moe_protect::threat_type::ThreatType;
use crate::symbolic_engine::symbolic_expert::SymbolicExpert;

fn make_experts() -> Vec<Box<dyn ProtectionExpert>> {
    vec![
        Box::new(AntivirusExpert::new()),
        Box::new(FirewallExpert::new()),
        Box::new(SymbolicExpert::new()),
    ]
}

#[test]
fn routes_virus_to_antivirus_and_symbolic() {
    let experts = make_experts();
    let event = ThreatEvent::new(
        ThreatType::Virus,
        ThreatLevel::High,
        "src",
        "dst",
        "payload",
    )
    .with_id(ThreatId::from_str("test-route-001"));
    let ids = MoeRouter::route(&event, &experts);
    // Should include antivirus and symbolic (symbolic handles all)
    assert!(ids.len() >= 2);
}

#[test]
fn routes_network_intrusion_to_firewall_and_symbolic() {
    let experts = make_experts();
    let event = ThreatEvent::new(
        ThreatType::NetworkIntrusion,
        ThreatLevel::High,
        "src",
        "dst",
        "payload",
    )
    .with_id(ThreatId::from_str("test-route-002"));
    let ids = MoeRouter::route(&event, &experts);
    assert!(ids.len() >= 2);
}
