use crate::domain::contradiction_memory::ContradictionMemory;
use crate::domain::dataset_identifier::DatasetIdentifier;
use crate::domain::journal_event::JournalEvent;
use crate::domain::observation_record::ObservationRecord;
use crate::domain::observation_slice::ObservationSlice;
use crate::domain::report_model::TickReport;
use crate::domain::run_metadata::RunMetadata;
use crate::domain::tick_index::TickIndex;
use crate::domain::weather_state::WeatherState;
use crate::simulation::tick_processor::TickProcessor;

pub struct SimulationEngine;

pub struct SimulationOutput {
    pub metadata: RunMetadata,
    pub tick_reports: Vec<TickReport>,
    pub journal: Vec<JournalEvent>,
    pub contradiction_memory: ContradictionMemory,
    pub final_state: WeatherState,
}

impl SimulationEngine {
    pub fn run(
        seed: u64,
        tick_count: u64,
        dataset_id: DatasetIdentifier,
        dataset_checksum: String,
        observations: Vec<ObservationRecord>,
    ) -> SimulationOutput {
        let metadata = RunMetadata {
            seed,
            tick_count,
            dataset: dataset_id,
            dataset_checksum,
        };

        let mut journal = vec![JournalEvent::RunStarted {
            metadata: metadata.clone(),
        }];

        let mut state = WeatherState::initial();
        let mut memory = ContradictionMemory::new();
        let mut tick_reports = Vec::new();

        let slices = Self::build_slices(tick_count, &observations);

        for (i, slice) in slices.iter().enumerate() {
            let tick = TickIndex(i as u64);
            let (tick_report, tick_events, new_state) =
                TickProcessor::process(tick, &state, slice, seed, &mut memory);

            journal.extend(tick_events);
            tick_reports.push(tick_report);
            state = new_state;
        }

        journal.push(JournalEvent::RunCompleted);

        SimulationOutput {
            metadata,
            tick_reports,
            journal,
            contradiction_memory: memory,
            final_state: state,
        }
    }

    fn build_slices(
        tick_count: u64,
        observations: &[ObservationRecord],
    ) -> Vec<ObservationSlice> {
        if observations.is_empty() || tick_count == 0 {
            return Vec::new();
        }

        let total = observations.len();
        let per_tick = (total as f64 / tick_count as f64).ceil() as usize;
        let per_tick = per_tick.max(1);

        let mut slices = Vec::new();
        for i in 0..tick_count {
            let start = (i as usize) * per_tick;
            let end = ((i as usize + 1) * per_tick).min(total);
            let records = if start < total {
                observations[start..end].to_vec()
            } else {
                vec![observations[total - 1].clone()]
            };

            slices.push(ObservationSlice {
                tick: TickIndex(i),
                records,
            });
        }

        slices
    }
}
