use crate::domain::checksum_value::ChecksumValue;
use crate::replay::replay_result::ReplayResult;
use crate::replay::replay_validator::ReplayValidator;

pub struct EquivalenceValidation;

impl EquivalenceValidation {
    pub fn validate(
        original_report_checksum: &ChecksumValue,
        replay_report_checksum: &ChecksumValue,
        original_snapshot_checksum: &ChecksumValue,
        replay_snapshot_checksum: &ChecksumValue,
    ) -> ReplayResult {
        ReplayValidator::validate(
            original_report_checksum,
            replay_report_checksum,
            original_snapshot_checksum,
            replay_snapshot_checksum,
        )
    }
}
