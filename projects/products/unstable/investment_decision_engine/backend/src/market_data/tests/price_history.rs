use crate::market_data::{PriceHistory, PricePoint};

fn sample_history() -> PriceHistory {
    PriceHistory::new(
        "AAPL",
        vec![
            PricePoint::new("2025-01-13", 100.0, 110.0, 95.0, 105.0),
            PricePoint::new("2025-01-14", 105.0, 115.0, 100.0, 108.0),
            PricePoint::new("2025-01-15", 108.0, 112.0, 102.0, 103.0),
        ],
    )
}

#[test]
fn latest_close_returns_last_point() {
    let h = sample_history();
    assert!((h.latest_close().unwrap() - 103.0).abs() < f64::EPSILON);
}

#[test]
fn recent_high_finds_max() {
    let h = sample_history();
    assert!((h.recent_high(3).unwrap() - 115.0).abs() < f64::EPSILON);
}

#[test]
fn drawdown_from_recent_peak() {
    let h = sample_history();
    let dd = h.drawdown_from_recent_peak(3).unwrap();
    let expected = (103.0 - 115.0) / 115.0;
    assert!((dd - expected).abs() < 1e-10);
}

#[test]
fn empty_history_returns_none() {
    let h = PriceHistory::new("AAPL", vec![]);
    assert!(h.latest_close().is_none());
    assert!(h.recent_high(5).is_none());
}
