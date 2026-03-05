// projects/products/unstable/colony_manager/backend/src/tests.rs
use crate::jobs::job::Job;
use crate::jobs::job_assigner::JobAssigner;
use crate::jobs::job_id::JobId;
use crate::jobs::job_kind::JobKind;
use crate::jobs::job_queue::JobQueue;
use crate::model::colonist::Colonist;
use crate::model::colonist_id::ColonistId;
use crate::moods::Mood;
use crate::needs::needs_state::NeedsState;
use crate::scenarios::scenario_loader::ScenarioLoader;
use crate::sim_engine::SimEngine;
use std::collections::BTreeMap;

fn must_run(
    scenario: &crate::scenarios::Scenario,
    ticks: u64,
    seed: u64,
) -> (
    crate::report::sim_report::SimReport,
    Vec<crate::rng::rng_draw::RngDraw>,
) {
    match SimEngine::run(scenario, ticks, seed) {
        Ok(value) => value,
        Err(_) => (
            crate::report::sim_report::SimReport {
                ticks_run: 0,
                scenario_name: String::new(),
                seed: 0,
                colonist_reports: Vec::new(),
                event_count: 0,
                snapshot_hashes: std::collections::BTreeMap::new(),
                run_hash: crate::report::run_hash::RunHash(String::new()),
            },
            Vec::new(),
        ),
    }
}

fn make_colonist(id: u32) -> Colonist {
    Colonist {
        id: ColonistId(id),
        name: format!("Colonist{id}"),
        needs: NeedsState::default(),
        mood: Mood::default(),
        assigned_job: None,
        productivity: 1.0,
    }
}

fn make_job(id: u32, priority: u32) -> Job {
    Job {
        id: JobId(id),
        kind: JobKind::Gather,
        priority,
        assigned_to: None,
        ticks_remaining: 5,
    }
}

#[test]
fn job_assignment_determinism() {
    let mut colonists = BTreeMap::new();
    colonists.insert(ColonistId(0), make_colonist(0));
    colonists.insert(ColonistId(1), make_colonist(1));

    let mut queue = JobQueue::default();
    queue.add(make_job(0, 5));
    queue.add(make_job(1, 10));

    let result1 = JobAssigner::assign(&colonists, &queue);
    let result2 = JobAssigner::assign(&colonists, &queue);
    assert_eq!(result1, result2, "job assignment must be deterministic");
}

#[test]
fn mood_update_determinism() {
    let mut mood1 = Mood::default();
    let mut mood2 = Mood::default();
    let needs_avg = 0.75f32;
    mood1.update_from_needs(needs_avg);
    mood2.update_from_needs(needs_avg);
    assert_eq!(
        mood1.value, mood2.value,
        "mood update must be deterministic"
    );
}

#[test]
fn canonical_encoding_determinism() {
    let scenario = ScenarioLoader::default_scenario("hauling_basic");
    let (r1, _) = must_run(&scenario, 10, 42);
    let (r2, _) = must_run(&scenario, 10, 42);
    assert!(
        !r1.run_hash.0.is_empty(),
        "first run must produce a run hash"
    );
    assert!(
        !r2.run_hash.0.is_empty(),
        "second run must produce a run hash"
    );
    assert_eq!(r1.run_hash.0, r2.run_hash.0, "same seed => same RunHash");
}

#[test]
fn replay_draws_match_run_draws() {
    let scenario = ScenarioLoader::default_scenario("hauling_basic");
    let (_, draws1) = must_run(&scenario, 20, 99);
    let (_, draws2) = must_run(&scenario, 20, 99);
    assert!(!draws1.is_empty(), "first run must produce RNG draws");
    assert!(!draws2.is_empty(), "second run must produce RNG draws");
    assert_eq!(draws1, draws2, "same seed => same replay draws");
}

#[test]
fn different_seeds_produce_different_hashes() {
    let scenario = ScenarioLoader::default_scenario("hauling_basic");
    let (r1, _) = must_run(&scenario, 10, 1);
    let (r2, _) = must_run(&scenario, 10, 2);
    assert!(!r1.run_hash.0.is_empty());
    assert!(!r2.run_hash.0.is_empty());
    assert_ne!(
        r1.run_hash.0, r2.run_hash.0,
        "different seeds must produce different RunHashes"
    );
}
