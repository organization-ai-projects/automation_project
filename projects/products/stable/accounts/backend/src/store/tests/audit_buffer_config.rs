use crate::store::audit_buffer_config::AuditBufferConfig;

#[test]
fn default_audit_buffer_config_has_expected_limits() {
    let config = AuditBufferConfig::default();

    assert_eq!(config.max_batch_size, 100);
    assert_eq!(config.flush_interval_secs, 5);
    assert_eq!(config.max_pending_entries, 10_000);
}
