use crate::fundamentals::IncomeStatement;

#[test]
fn margins_compute_correctly() {
    let is = IncomeStatement {
        period: "2024-Q4".to_string(),
        revenue: 100_000.0,
        gross_profit: 60_000.0,
        operating_income: 30_000.0,
        net_income: 20_000.0,
        eps: 2.0,
    };
    assert!((is.gross_margin().unwrap() - 0.6).abs() < f64::EPSILON);
    assert!((is.operating_margin().unwrap() - 0.3).abs() < f64::EPSILON);
    assert!((is.net_margin().unwrap() - 0.2).abs() < f64::EPSILON);
}

#[test]
fn zero_revenue_returns_none() {
    let is = IncomeStatement {
        period: "2024-Q4".to_string(),
        revenue: 0.0,
        gross_profit: 0.0,
        operating_income: 0.0,
        net_income: 0.0,
        eps: 0.0,
    };
    assert!(is.gross_margin().is_none());
    assert!(is.operating_margin().is_none());
    assert!(is.net_margin().is_none());
}
