use crate::pricing::pricing_policy::PricingPolicy;

#[test]
fn default_policy() {
    let p = PricingPolicy::default();
    assert_eq!(p.markup_percent, 30);
    assert_eq!(p.discount_threshold, 100);
    assert_eq!(p.discount_percent, 10);
}
