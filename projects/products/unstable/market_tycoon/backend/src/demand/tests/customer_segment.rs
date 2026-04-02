use crate::demand::customer_segment::CustomerSegment;
use crate::model::good::Good;

#[test]
fn segment_fields() {
    let s = CustomerSegment {
        name: "Budget".into(),
        base_demand: 50,
        price_sensitivity: 80,
        good: Good::Widget,
    };
    assert_eq!(s.name, "Budget");
    assert_eq!(s.base_demand, 50);
}
