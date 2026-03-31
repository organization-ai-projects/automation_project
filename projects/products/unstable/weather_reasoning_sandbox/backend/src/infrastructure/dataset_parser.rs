use crate::domain::observation_record::ObservationRecord;
use crate::reporting::checksum_generator::ChecksumGenerator;

pub struct DatasetParser;

impl DatasetParser {
    pub fn compute_checksum(records: &[ObservationRecord]) -> String {
        let canonical: Vec<String> = records
            .iter()
            .map(|r| {
                format!(
                    "ts={},p={:.2},t={:.2},h={:.2},w={:.2},c={:.2},pr={:.2}",
                    r.timestamp_index,
                    r.pressure_hpa,
                    r.temperature_c,
                    r.humidity_pct,
                    r.wind_speed_kmh,
                    r.cloudiness_pct,
                    r.precipitation_mm,
                )
            })
            .collect();
        let data = canonical.join("|");
        ChecksumGenerator::compute(&data).0
    }
}
