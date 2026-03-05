// projects/products/unstable/colony_manager/backend/src/sim_engine.rs
use crate::diagnostics::colony_manager_error::ColonyManagerError;
use crate::events::colony_event;
use crate::events::event_deck::EventDeck;
use crate::events::event_log::EventLog;
use crate::hauling::haul_engine::HaulEngine;
use crate::hauling::haul_task::HaulTask;
use crate::jobs::job::Job;
use crate::jobs::job_assigner::JobAssigner;
use crate::jobs::job_executor::JobExecutor;
use crate::jobs::job_id::JobId;
use crate::jobs::job_kind::JobKind;
use crate::map::cell_id::CellId;
use crate::map::colony_map::ColonyMap;
use crate::model::colonist::Colonist;
use crate::model::colony_state::ColonyState;
use crate::model::item::Item;
use crate::model::item_id::ItemId;
use crate::moods::mood_modifier::MoodModifier;
use crate::report::colonist_report::ColonistReport;
use crate::report::run_hash::RunHash;
use crate::report::sim_report::SimReport;
use crate::rng::rng_draw::RngDraw;
use crate::rng::seeded_rng::SeededRng;
use crate::scenarios::Scenario;
use crate::snapshot::snapshot_hash::SnapshotHash;
use crate::snapshot::state_snapshot::StateSnapshot;
use std::collections::BTreeMap;

pub struct SimEngine;

impl SimEngine {
    pub fn run(
        scenario: &Scenario,
        ticks: u64,
        seed: u64,
    ) -> Result<(SimReport, Vec<RngDraw>), ColonyManagerError> {
        let mut rng = SeededRng::from_seed(seed);
        let mut rng_draws: Vec<RngDraw> = Vec::new();
        let report = Self::run_inner(scenario, ticks, seed, &mut rng, &mut rng_draws)?;
        Ok((report, rng_draws))
    }

    fn run_inner(
        scenario: &Scenario,
        ticks: u64,
        seed: u64,
        rng: &mut SeededRng,
        rng_draws: &mut Vec<RngDraw>,
    ) -> Result<SimReport, ColonyManagerError> {
        let map = ColonyMap::new(scenario.map_width, scenario.map_height);
        let mut state = ColonyState::new(map);

        for (id, name) in &scenario.colonists {
            state
                .colonists
                .insert(*id, Colonist::new(*id, name.clone()));
        }

        let mut job_id_counter: u32 = 0;
        for _ in &scenario.colonists {
            state.job_queue.add(Job {
                id: JobId(job_id_counter),
                kind: JobKind::Gather,
                priority: 5,
                assigned_to: None,
                ticks_remaining: 3,
            });
            job_id_counter += 1;
        }

        let event_deck = EventDeck::default_deck();
        let mut event_log = EventLog::default();
        let mut haul_engine = HaulEngine::default();
        let mut snapshot_hashes: BTreeMap<String, String> = BTreeMap::new();
        let mut jobs_completed: BTreeMap<u32, u32> = BTreeMap::new();

        for tick_idx in 0..ticks {
            state.clock.tick();
            let current_tick = state.clock.current();

            for colonist in state.colonists.values_mut() {
                colonist.needs.decay(0.05);
                let avg = colonist.needs.average();
                colonist.mood.update_from_needs(avg);
                colonist.productivity = (colonist.mood.value + 0.5).clamp(0.1, 1.5);
            }

            let assignments = JobAssigner::assign(&state.colonists, &state.job_queue);
            for (cid, jid) in assignments {
                if let Some(c) = state.colonists.get_mut(&cid) {
                    c.assigned_job = Some(jid);
                }
                if let Some(j) = state.job_queue.jobs.get_mut(&jid) {
                    j.assigned_to = Some(cid);
                }
            }

            let before_count = state.job_queue.jobs.len();
            JobExecutor::execute_tick(&mut state.colonists, &mut state.job_queue);
            let after_count = state.job_queue.jobs.len();
            let completed = before_count.saturating_sub(after_count);

            if completed > 0 {
                let cids: Vec<u32> = state.colonists.keys().map(|c| c.0).collect();
                if !cids.is_empty() {
                    for i in 0..completed {
                        let cid = cids[i % cids.len()];
                        *jobs_completed.entry(cid).or_insert(0) += 1;
                    }
                }
                for _ in 0..completed {
                    let item_id = ItemId(job_id_counter);
                    state.inventory.add(Item {
                        id: item_id,
                        kind: "resource".to_string(),
                        quantity: 1,
                    });
                    haul_engine.add_task(HaulTask {
                        item_id,
                        from: CellId(0),
                        to: CellId(1),
                        assigned_to: None,
                    });
                    state.job_queue.add(Job {
                        id: JobId(job_id_counter),
                        kind: JobKind::Gather,
                        priority: 5,
                        assigned_to: None,
                        ticks_remaining: 3,
                    });
                    job_id_counter += 1;
                }
                let available: Vec<_> = state
                    .colonists
                    .values()
                    .filter(|c| c.assigned_job.is_none())
                    .map(|c| c.id)
                    .collect();
                haul_engine.assign_tick(&available);
            }

            let event_roll = rng.next_u64();
            rng_draws.push(RngDraw {
                raw_value: event_roll,
                resolved_index: 0,
            });
            let threshold = (scenario.event_probability as f64 * u64::MAX as f64) as u64;
            if event_roll < threshold
                && let Some((idx, event)) = event_deck.draw(rng, rng_draws)
            {
                match event {
                    colony_event::ColonyEvent::Raid { severity } => {
                        let modifier = MoodModifier {
                            source: "raid".to_string(),
                            delta: -(*severity as f32) * 0.05,
                        };
                        for c in state.colonists.values_mut() {
                            c.mood.apply_modifier(modifier.delta);
                        }
                    }
                    colony_event::ColonyEvent::Sickness { .. } => {
                        if let Some(c) = state.colonists.values_mut().next() {
                            c.needs
                                .levels
                                .insert(crate::needs::need_kind::NeedKind::Food, 0.2);
                        }
                    }
                    colony_event::ColonyEvent::Traders { .. } => {
                        let modifier = MoodModifier {
                            source: "traders".to_string(),
                            delta: 0.05,
                        };
                        for c in state.colonists.values_mut() {
                            c.mood.apply_modifier(modifier.delta);
                        }
                    }
                    colony_event::ColonyEvent::Windfall { .. } => {
                        for c in state.colonists.values_mut() {
                            c.needs
                                .levels
                                .insert(crate::needs::need_kind::NeedKind::Food, 1.0);
                        }
                    }
                }
                event_log.record(current_tick, event.clone(), idx);
            }

            if current_tick.value().is_multiple_of(25) {
                let ids: Vec<_> = state.inventory.items.keys().copied().collect();
                for id in ids {
                    drop(state.inventory.remove(&id));
                }
            }

            if (tick_idx + 1).is_multiple_of(10) {
                let snap = StateSnapshot::take(current_tick, &state);
                let hash = SnapshotHash::compute(&snap);
                snapshot_hashes.insert(format!("tick_{}", current_tick.value()), hash.0);
            }
        }

        let colonist_reports: Vec<ColonistReport> = {
            let mut v: Vec<ColonistReport> = state
                .colonists
                .values()
                .map(|c| ColonistReport {
                    id: c.id,
                    name: c.name.clone(),
                    final_mood: c.mood.value,
                    jobs_completed: *jobs_completed.get(&c.id.0).unwrap_or(&0),
                })
                .collect();
            v.sort_by_key(|r| r.id);
            v
        };

        let mut report = SimReport {
            ticks_run: ticks,
            scenario_name: scenario.name.clone(),
            seed,
            colonist_reports,
            event_count: event_log.entries.len(),
            snapshot_hashes,
            run_hash: RunHash(String::new()),
        };
        report.compute_hash();
        Ok(report)
    }
}
