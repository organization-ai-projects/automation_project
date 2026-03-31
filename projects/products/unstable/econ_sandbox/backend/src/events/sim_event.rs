use serde::{Deserialize, Serialize};

use crate::agents::agent_id::AgentId;
use crate::market::good::Good;
use crate::time::tick::Tick;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimEvent {
    pub tick: Tick,
    pub kind: SimEventKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimEventKind {
    Produced {
        agent_id: AgentId,
        good: Good,
        amount: u64,
    },
    Consumed {
        agent_id: AgentId,
        good: Good,
        amount: u64,
    },
    OrderMatched {
        buyer: AgentId,
        seller: AgentId,
        good: Good,
        quantity: u64,
        price: i64,
    },
    TaxCollected {
        agent_id: AgentId,
        amount: i64,
    },
    SubsidyPaid {
        agent_id: AgentId,
        amount: i64,
    },
}
