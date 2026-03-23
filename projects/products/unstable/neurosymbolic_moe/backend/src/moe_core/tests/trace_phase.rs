use crate::moe_core::TracePhase;

#[test]
fn trace_phase_variants_are_constructible() {
    let routing = TracePhase::Routing;
    let aggregation = TracePhase::Aggregation;
    assert!(matches!(routing, TracePhase::Routing));
    assert!(matches!(aggregation, TracePhase::Aggregation));
}
