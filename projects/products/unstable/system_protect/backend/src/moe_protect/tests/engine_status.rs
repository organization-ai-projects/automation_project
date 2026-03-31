use crate::moe_protect::engine_status::EngineStatus;

#[test]
fn engine_status_holds_counts() {
    let status = EngineStatus {
        expert_count: 3,
        firewall_rule_count: 5,
        signature_count: 10,
        events_analyzed: 100,
        threats_blocked: 42,
    };
    assert_eq!(status.expert_count, 3);
    assert_eq!(status.threats_blocked, 42);
}
