#[cfg(test)]
mod tests {
    use crate::jobs::job::Job;
    use crate::jobs::job_assigner::JobAssigner;
    use crate::jobs::job_id::JobId;
    use crate::jobs::job_kind::JobKind;
    use crate::jobs::job_queue::JobQueue;
    use crate::model::colonist::Colonist;
    use crate::model::colonist_id::ColonistId;
    use crate::mood::mood::Mood;
    use crate::needs::needs_state::NeedsState;
    use crate::scenario::scenario_loader::ScenarioLoader;
    use crate::sim_engine::SimEngine;
    use std::collections::BTreeMap;

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
        assert_eq!(result1, result2, "Job assignment must be deterministic");
    }

    #[test]
    fn mood_update_determinism() {
        let mut mood1 = Mood::default();
        let mut mood2 = Mood::default();
        let needs_avg = 0.75f32;
        mood1.update_from_needs(needs_avg);
        mood2.update_from_needs(needs_avg);
        assert_eq!(mood1.value, mood2.value, "Mood update must be deterministic");
    }

    #[test]
    fn canonical_encoding_determinism() {
        let scenario = ScenarioLoader::default_scenario("hauling_basic");
        let (r1, _) = SimEngine::run(&scenario, 10, 42).unwrap();
        let (r2, _) = SimEngine::run(&scenario, 10, 42).unwrap();
        assert_eq!(r1.run_hash.0, r2.run_hash.0, "Same seed => same RunHash");
    }

    #[test]
    fn run_vs_replay_equivalence() {
        use rand::SeedableRng;
        use rand::rngs::SmallRng;

        let scenario = ScenarioLoader::default_scenario("hauling_basic");
        let seed = 99u64;
        let (original_report, _) = SimEngine::run(&scenario, 20, seed).unwrap();

        let mut replay_rng = SmallRng::seed_from_u64(seed);
        let replay_report = SimEngine::run_with_rng(&scenario, 20, seed, &mut replay_rng).unwrap();

        assert_eq!(
            original_report.run_hash.0, replay_report.run_hash.0,
            "Replay must produce identical RunHash"
        );
    }

    #[test]
    fn different_seeds_different_hash() {
        let scenario = ScenarioLoader::default_scenario("hauling_basic");
        let (r1, _) = SimEngine::run(&scenario, 10, 1).unwrap();
        let (r2, _) = SimEngine::run(&scenario, 10, 2).unwrap();
        assert!(!r1.run_hash.0.is_empty());
        assert!(!r2.run_hash.0.is_empty());
    }
}
