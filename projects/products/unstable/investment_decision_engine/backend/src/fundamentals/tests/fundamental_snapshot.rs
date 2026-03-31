use crate::fundamentals::{FundamentalSnapshot, IncomeStatement};

#[test]
fn new_creates_empty_snapshot() {
    let snap = FundamentalSnapshot::new("AAPL");
    assert_eq!(snap.ticker, "AAPL");
    assert!(snap.income_statements.is_empty());
    assert!(snap.latest_income().is_none());
}

#[test]
fn latest_income_returns_last() {
    let mut snap = FundamentalSnapshot::new("AAPL");
    snap.income_statements.push(IncomeStatement {
        period: "2024-Q3".to_string(),
        revenue: 80_000.0,
        gross_profit: 40_000.0,
        operating_income: 20_000.0,
        net_income: 15_000.0,
        eps: 1.5,
    });
    snap.income_statements.push(IncomeStatement {
        period: "2024-Q4".to_string(),
        revenue: 100_000.0,
        gross_profit: 60_000.0,
        operating_income: 30_000.0,
        net_income: 20_000.0,
        eps: 2.0,
    });
    let latest = snap.latest_income().unwrap();
    assert_eq!(latest.period, "2024-Q4");
}

#[test]
fn serialization_roundtrip() {
    let snap = FundamentalSnapshot::new("GOOG");
    let json = common_json::to_json_string(&snap).unwrap();
    let restored: FundamentalSnapshot = common_json::from_str(&json).unwrap();
    assert_eq!(snap, restored);
}
