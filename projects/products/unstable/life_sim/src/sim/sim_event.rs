use crate::actions::ActionKind;
use crate::interaction::Interaction;
use crate::model::agent_id::AgentId;
use crate::model::room_id::RoomId;
use crate::needs::NeedKind;
use crate::time::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimEvent {
    AgentActed {
        tick: Tick,
        agent_id: AgentId,
        action: ActionKind,
    },
    NeedChanged {
        tick: Tick,
        agent_id: AgentId,
        need_kind: NeedKind,
        old_val: u8,
        new_val: u8,
    },
    InteractionOccurred {
        tick: Tick,
        interaction: Interaction,
    },
    AgentMoved {
        tick: Tick,
        agent_id: AgentId,
        from_room: RoomId,
        to_room: RoomId,
    },
    SimulationStarted {
        tick: Tick,
    },
    SimulationEnded {
        tick: Tick,
    },
}
