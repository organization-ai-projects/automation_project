//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/tests/concurrent_lock_metrics.rs
use crate::orchestrator::ConcurrentLockMetrics;

#[test]
fn concurrent_lock_metrics_rates_are_stable_for_zero_acquisitions() {
    let metrics = ConcurrentLockMetrics::default();
    assert_eq!(metrics.total_lock_acquisitions(), 0);
    assert_eq!(metrics.total_contention_events(), 0);
    assert_eq!(metrics.total_timeout_events(), 0);
    assert_eq!(metrics.contention_rate(), 0.0);
    assert_eq!(metrics.timeout_rate(), 0.0);
    assert_eq!(metrics.avg_read_spin_attempts(), 0.0);
    assert_eq!(metrics.avg_write_spin_attempts(), 0.0);
}

#[test]
fn concurrent_lock_metrics_rates_match_aggregates() {
    let metrics = ConcurrentLockMetrics {
        read_lock_acquisitions: 60,
        write_lock_acquisitions: 40,
        read_lock_contention: 20,
        write_lock_contention: 10,
        read_lock_timeouts: 3,
        write_lock_timeouts: 2,
        read_lock_spin_attempts_total: 90,
        write_lock_spin_attempts_total: 70,
    };
    assert_eq!(metrics.total_lock_acquisitions(), 100);
    assert_eq!(metrics.total_contention_events(), 30);
    assert_eq!(metrics.total_timeout_events(), 5);
    assert!((metrics.contention_rate() - 0.30).abs() < f64::EPSILON);
    assert!((metrics.timeout_rate() - 0.05).abs() < f64::EPSILON);
    assert!((metrics.avg_read_spin_attempts() - 1.5).abs() < f64::EPSILON);
    assert!((metrics.avg_write_spin_attempts() - 1.75).abs() < f64::EPSILON);
}
