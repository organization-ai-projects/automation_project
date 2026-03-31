use crate::fundamentals::CashFlowStatement;

#[test]
fn fcf_yield_computes_correctly() {
    let cf = CashFlowStatement {
        period: "2024-Q4".to_string(),
        operating_cash_flow: 50_000.0,
        capital_expenditures: 10_000.0,
        free_cash_flow: 40_000.0,
        dividends_paid: 5_000.0,
    };
    let yield_pct = cf.fcf_yield(1_000_000.0).unwrap();
    assert!((yield_pct - 0.04).abs() < f64::EPSILON);
}

#[test]
fn zero_market_cap_returns_none() {
    let cf = CashFlowStatement {
        period: "2024-Q4".to_string(),
        operating_cash_flow: 50_000.0,
        capital_expenditures: 10_000.0,
        free_cash_flow: 40_000.0,
        dividends_paid: 5_000.0,
    };
    assert!(cf.fcf_yield(0.0).is_none());
}
