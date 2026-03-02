use crate::actions::Action;
use crate::decision::utility_score::UtilityScore;
use crate::model::agent::Agent;

pub struct UtilityModel;

impl UtilityModel {
    /// Compute a utility score for the given action based on agent's current needs and traits.
    pub fn score(agent: &Agent, action: &Action) -> UtilityScore {
        let mut total: f64 = 0.0;
        for (need_kind, delta) in &action.effect.need_deltas {
            let current = agent.needs.get(*need_kind).0 as f64;
            // Higher utility when need is low and action increases it
            let urgency = (100.0 - current) / 100.0;
            let contribution = *delta as f64 * urgency;
            total += contribution;
        }
        UtilityScore(total)
    }
}
