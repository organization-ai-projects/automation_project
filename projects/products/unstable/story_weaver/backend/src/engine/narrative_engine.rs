use crate::config::StoryConfig;
use crate::diagnostics::Error;
use crate::dsl::{Condition, Effect, Rule, Script};
use crate::events::{EventLog, StoryEvent, StoryEventKind};
use crate::replay::ReplayFile;
use crate::report::{RunHash, StoryReport};
use crate::rng::SeededRng;
use crate::snapshot::SnapshotHash;
use crate::state::{StateValue, StoryState};

pub struct NarrativeEngine;

impl NarrativeEngine {
    pub fn run(script: &Script, config: &StoryConfig) -> Result<(StoryReport, ReplayFile), Error> {
        let mut state = StoryState::new(script.initial_state.clone());
        let mut rng = SeededRng::new(config.seed);
        let mut event_log = EventLog::new();
        let mut steps_taken: u64 = 0;

        for step in 0..config.max_steps {
            let applicable: Vec<&Rule> = script
                .rules
                .iter()
                .filter(|r| Self::evaluate_conditions(&r.conditions, &state))
                .collect();

            if applicable.is_empty() {
                event_log.push(StoryEvent {
                    step,
                    kind: StoryEventKind::NoApplicableRules,
                });
                steps_taken = step + 1;
                break;
            }

            let selected = Self::weighted_select(&applicable, &mut rng, step);

            event_log.push(StoryEvent {
                step,
                kind: StoryEventKind::RuleApplied {
                    rule_id: selected.id.clone(),
                },
            });

            Self::apply_effects(&selected.effects, &mut state, &mut event_log, step);

            steps_taken = step + 1;
        }

        let snapshot_hash = SnapshotHash::compute(&state);
        let event_count = event_log.len();
        let run_hash = RunHash::compute(config.seed, steps_taken, event_count, &snapshot_hash);

        let report = StoryReport {
            run_hash,
            seed: config.seed,
            steps_taken,
            event_count,
            snapshot_hash,
            title: script.title.clone(),
        };

        let replay = ReplayFile::new(config.seed, script.clone(), event_log.events().to_vec());

        Ok((report, replay))
    }

    fn evaluate_conditions(conditions: &[Condition], state: &StoryState) -> bool {
        conditions.iter().all(|c| Self::evaluate_condition(c, state))
    }

    fn evaluate_condition(condition: &Condition, state: &StoryState) -> bool {
        match condition {
            Condition::Equals { variable, value } => state.get(variable) == Some(value),
            Condition::GreaterThan { variable, value } => {
                matches!(state.get(variable), Some(StateValue::Number(n)) if *n > *value)
            }
            Condition::LessThan { variable, value } => {
                matches!(state.get(variable), Some(StateValue::Number(n)) if *n < *value)
            }
        }
    }

    fn weighted_select<'a>(
        rules: &[&'a Rule],
        rng: &mut SeededRng,
        step: u64,
    ) -> &'a Rule {
        let total_weight: u64 = rules.iter().map(|r| r.weight).sum();
        let roll = rng.draw_u64(&format!("step_{}_select", step)) % total_weight;

        let mut accumulated: u64 = 0;
        for rule in rules {
            accumulated += rule.weight;
            if roll < accumulated {
                return rule;
            }
        }

        rules.last().unwrap()
    }

    fn apply_effects(
        effects: &[Effect],
        state: &mut StoryState,
        event_log: &mut EventLog,
        step: u64,
    ) {
        for effect in effects {
            match effect {
                Effect::Set { variable, value } => {
                    let old_value = state
                        .get(variable)
                        .cloned()
                        .unwrap_or(StateValue::Flag(false));
                    state.set(variable.clone(), value.clone());
                    event_log.push(StoryEvent {
                        step,
                        kind: StoryEventKind::StateChanged {
                            variable: variable.clone(),
                            old_value,
                            new_value: value.clone(),
                        },
                    });
                }
                Effect::Add { variable, amount } => {
                    let old_value = state
                        .get(variable)
                        .cloned()
                        .unwrap_or(StateValue::Number(0));
                    state.add(variable, *amount);
                    let new_value = state.get(variable).cloned().unwrap_or(StateValue::Number(0));
                    event_log.push(StoryEvent {
                        step,
                        kind: StoryEventKind::StateChanged {
                            variable: variable.clone(),
                            old_value,
                            new_value,
                        },
                    });
                }
                Effect::Log { message } => {
                    event_log.push(StoryEvent {
                        step,
                        kind: StoryEventKind::Narration {
                            message: message.clone(),
                        },
                    });
                }
            }
        }
    }
}
