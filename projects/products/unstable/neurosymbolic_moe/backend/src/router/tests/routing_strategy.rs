use crate::router::RoutingStrategy;

#[test]
fn routing_strategy_variants_are_constructible() {
    let single = RoutingStrategy::SingleExpert;
    let multi = RoutingStrategy::MultiExpert;
    let fallback = RoutingStrategy::Fallback;
    let round_robin = RoutingStrategy::RoundRobin;

    assert!(matches!(single, RoutingStrategy::SingleExpert));
    assert!(matches!(multi, RoutingStrategy::MultiExpert));
    assert!(matches!(fallback, RoutingStrategy::Fallback));
    assert!(matches!(round_robin, RoutingStrategy::RoundRobin));
}
