use crate::fundamentals::BalanceSheet;

#[test]
fn debt_to_equity_positive() {
    let bs = BalanceSheet {
        period: "2024-Q4".to_string(),
        total_assets: 100_000.0,
        total_liabilities: 40_000.0,
        total_equity: 60_000.0,
        cash_and_equivalents: 10_000.0,
        total_debt: 30_000.0,
    };
    assert!((bs.debt_to_equity().unwrap() - 0.5).abs() < f64::EPSILON);
}

#[test]
fn net_debt_calculation() {
    let bs = BalanceSheet {
        period: "2024-Q4".to_string(),
        total_assets: 100_000.0,
        total_liabilities: 40_000.0,
        total_equity: 60_000.0,
        cash_and_equivalents: 10_000.0,
        total_debt: 30_000.0,
    };
    assert!((bs.net_debt() - 20_000.0).abs() < f64::EPSILON);
}

#[test]
fn zero_equity_returns_none() {
    let bs = BalanceSheet {
        period: "2024-Q4".to_string(),
        total_assets: 0.0,
        total_liabilities: 0.0,
        total_equity: 0.0,
        cash_and_equivalents: 0.0,
        total_debt: 0.0,
    };
    assert!(bs.debt_to_equity().is_none());
}
