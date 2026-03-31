use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::agents::agent::Agent;
use crate::agents::agent_id::AgentId;
use crate::market::good::Good;
use crate::time::tick::Tick;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub tick: Tick,
    pub agent_summary: String,
    pub price_summary: String,
}

impl StateSnapshot {
    pub fn capture(
        agents: &BTreeMap<AgentId, Agent>,
        prices: &BTreeMap<Good, i64>,
        tick: Tick,
    ) -> Self {
        // Build canonical agent summary (sorted by agent_id via BTreeMap)
        let agent_parts: Vec<String> = agents
            .iter()
            .map(|(id, a)| {
                let inv: Vec<String> = a
                    .inventory
                    .iter()
                    .map(|(g, q)| format!("{g}={q}"))
                    .collect();
                format!("{}:cash={},inv=[{}]", id, a.cash, inv.join(","))
            })
            .collect();
        let agent_summary = agent_parts.join(";");

        // Build canonical price summary (sorted by Good via BTreeMap)
        let price_parts: Vec<String> = prices.iter().map(|(g, p)| format!("{g}={p}")).collect();
        let price_summary = price_parts.join(",");

        Self {
            tick,
            agent_summary,
            price_summary,
        }
    }
}
