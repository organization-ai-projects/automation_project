use crate::domain::dataset_identifier::DatasetIdentifier;
use crate::domain::observation_record::ObservationRecord;

pub struct DatasetLoader;

impl DatasetLoader {
    pub fn load(path: &str) -> Result<(DatasetIdentifier, Vec<ObservationRecord>), String> {
        let contents = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read dataset at {path}: {e}"))?;

        let records: Vec<ObservationRecord> = Self::parse_records(&contents)?;

        let id = DatasetIdentifier {
            name: Self::extract_name(path),
            path: path.to_string(),
        };

        Ok((id, records))
    }

    pub fn load_default(
        seed: u64,
        tick_count: u64,
    ) -> (DatasetIdentifier, Vec<ObservationRecord>) {
        let records = Self::generate_default_observations(seed, tick_count);
        let id = DatasetIdentifier {
            name: "default".to_string(),
            path: format!("generated:seed={seed},ticks={tick_count}"),
        };
        (id, records)
    }

    fn parse_records(contents: &str) -> Result<Vec<ObservationRecord>, String> {
        common_json::from_str(contents).map_err(|e| format!("Failed to parse dataset: {e}"))
    }

    fn extract_name(path: &str) -> String {
        std::path::Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string()
    }

    fn generate_default_observations(seed: u64, tick_count: u64) -> Vec<ObservationRecord> {
        let count = (tick_count * 3).max(10);
        let mut records = Vec::new();

        for i in 0..count {
            let t = i as f64;
            let s = seed as f64;

            let phase = (t * 0.3 + s * 0.01).sin();
            let phase2 = (t * 0.17 + s * 0.03).cos();

            records.push(ObservationRecord {
                timestamp_index: i,
                pressure_hpa: 1013.25 + phase * 15.0 + phase2 * 5.0,
                temperature_c: 20.0 + phase * 8.0 + phase2 * 3.0,
                humidity_pct: (50.0 + phase * 25.0 + phase2 * 15.0).clamp(5.0, 100.0),
                wind_speed_kmh: (10.0 + phase.abs() * 30.0 + phase2.abs() * 20.0)
                    .clamp(0.0, 120.0),
                cloudiness_pct: (30.0 + phase * 35.0 + phase2 * 20.0).clamp(0.0, 100.0),
                precipitation_mm: (phase * 5.0 + phase2 * 3.0).max(0.0),
            });
        }

        records
    }
}
