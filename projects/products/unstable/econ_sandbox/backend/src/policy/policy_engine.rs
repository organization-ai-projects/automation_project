use crate::agents::agent::Agent;
use crate::agents::agent_id::AgentId;
use crate::events::sim_event::{SimEvent, SimEventKind};
use crate::policy::policy_kind::PolicyKind;
use crate::time::tick::Tick;

pub struct PolicyEngine;

impl PolicyEngine {
    pub fn apply(
        policies: &[PolicyKind],
        agents: &mut std::collections::BTreeMap<AgentId, Agent>,
        tick: Tick,
        events: &mut Vec<SimEvent>,
    ) {
        let agent_ids: Vec<AgentId> = agents.keys().copied().collect();

        for policy in policies {
            for &aid in &agent_ids {
                if let Some(agent) = agents.get_mut(&aid) {
                    match policy {
                        PolicyKind::FlatTax { rate_pct } => {
                            let tax = (agent.cash * (*rate_pct as i64)) / 100;
                            if tax > 0 {
                                agent.cash -= tax;
                                events.push(SimEvent {
                                    tick,
                                    kind: SimEventKind::TaxCollected {
                                        agent_id: aid,
                                        amount: tax,
                                    },
                                });
                            }
                        }
                        PolicyKind::Subsidy { per_agent } => {
                            agent.cash += per_agent;
                            events.push(SimEvent {
                                tick,
                                kind: SimEventKind::SubsidyPaid {
                                    agent_id: aid,
                                    amount: *per_agent,
                                },
                            });
                        }
                    }
                }
            }
        }
    }
}
