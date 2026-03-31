use crate::domain::checksum_value::ChecksumValue;
use crate::replay::replay_result::ReplayResult;

pub struct ReplayValidator;

impl ReplayValidator {
    pub fn validate(
        original_report_checksum: &ChecksumValue,
        replay_report_checksum: &ChecksumValue,
        original_snapshot_checksum: &ChecksumValue,
        replay_snapshot_checksum: &ChecksumValue,
    ) -> ReplayResult {
        let is_equivalent = original_report_checksum == replay_report_checksum
            && original_snapshot_checksum == replay_snapshot_checksum;

        ReplayResult {
            original_report_checksum: original_report_checksum.clone(),
            replay_report_checksum: replay_report_checksum.clone(),
            original_snapshot_checksum: original_snapshot_checksum.clone(),
            replay_snapshot_checksum: replay_snapshot_checksum.clone(),
            is_equivalent,
        }
    }
}
