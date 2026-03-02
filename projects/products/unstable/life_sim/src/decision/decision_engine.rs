use crate::actions::Action;
use crate::decision::decision_context::DecisionContext;
use crate::decision::utility_model::UtilityModel;
use crate::model::agent::Agent;

pub struct DecisionEngine;

impl DecisionEngine {
    /// Pick one action from context using utility scores.
    /// Deterministic tie-breaking: highest score, then ActionKind discriminant, then target_agent id, then target_object id.
    pub fn pick(agent: &Agent, ctx: &DecisionContext) -> Option<Action> {
        ctx.available_actions
            .iter()
            .max_by(|a, b| {
                let sa = UtilityModel::score(agent, a);
                let sb = UtilityModel::score(agent, b);
                sa.cmp(&sb)
                    .then_with(|| (a.kind as u8).cmp(&(b.kind as u8)))
                    .then_with(|| {
                        a.target_agent
                            .map(|x| x.0)
                            .unwrap_or(u64::MAX)
                            .cmp(&b.target_agent.map(|x| x.0).unwrap_or(u64::MAX))
                    })
                    .then_with(|| {
                        a.target_object
                            .map(|x| x.0)
                            .unwrap_or(u64::MAX)
                            .cmp(&b.target_object.map(|x| x.0).unwrap_or(u64::MAX))
                    })
            })
            .cloned()
    }
}
