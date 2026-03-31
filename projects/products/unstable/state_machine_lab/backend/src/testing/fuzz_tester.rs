use crate::diagnostics::error::BackendError;
use crate::execute::runner::Runner;
use crate::model::machine::Machine;
use crate::testing::test_report::TestReport;

pub struct FuzzTester;

impl FuzzTester {
    pub fn test(machine: &Machine, seed: u64, steps: u64) -> Result<TestReport, BackendError> {
        let mut runner = Runner::new(machine.clone());
        let mut violations = Vec::new();
        let mut rng_state = seed;

        for _ in 0..steps {
            if machine.events.is_empty() {
                break;
            }
            let event_index = (rng_state % machine.events.len() as u64) as usize;
            let event = &machine.events[event_index];
            rng_state = Self::next_rng(rng_state);

            match runner.step(event) {
                Ok(_) => {}
                Err(e) => {
                    violations.push(format!(
                        "step {} from '{}' event '{}': {}",
                        runner.step_count,
                        runner.current_state.0,
                        event.0,
                        e
                    ));
                }
            }
        }

        Ok(TestReport::fuzz(seed, steps, violations))
    }

    fn next_rng(state: u64) -> u64 {
        // Simple xorshift64
        let mut s = state;
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::event_id::EventId;
    use crate::model::machine::{Machine, Transition};
    use crate::model::machine_id::MachineId;
    use crate::model::state_id::StateId;
    use std::collections::BTreeMap;

    fn toggle_machine() -> Machine {
        let mut transitions = BTreeMap::new();
        transitions.insert(
            Machine::transition_key(&StateId("off".into()), &EventId("flip".into())),
            vec![Transition {
                event: EventId("flip".into()),
                target: StateId("on".into()),
                guard: None,
                action: None,
            }],
        );
        transitions.insert(
            Machine::transition_key(&StateId("on".into()), &EventId("flip".into())),
            vec![Transition {
                event: EventId("flip".into()),
                target: StateId("off".into()),
                guard: None,
                action: None,
            }],
        );
        Machine {
            id: MachineId("toggle".into()),
            initial_state: StateId("off".into()),
            states: vec![StateId("off".into()), StateId("on".into())],
            events: vec![EventId("flip".into())],
            transitions,
            variables: BTreeMap::new(),
        }
    }

    #[test]
    fn fuzz_deterministic_with_same_seed() {
        let m = toggle_machine();
        let r1 = FuzzTester::test(&m, 42, 100).unwrap();
        let r2 = FuzzTester::test(&m, 42, 100).unwrap();
        assert_eq!(r1, r2);
    }

    #[test]
    fn fuzz_different_seeds_may_differ() {
        let m = toggle_machine();
        let r1 = FuzzTester::test(&m, 1, 50).unwrap();
        let r2 = FuzzTester::test(&m, 9999, 50).unwrap();
        // Both should pass for toggle machine
        assert!(r1.passed);
        assert!(r2.passed);
    }
}
