use crate::model::empire_id::EmpireId;
use crate::model::sim_state::SimState;
use crate::orders::order::Order;

use super::ai_profile::AiProfile;

pub struct AiEngine;

impl AiEngine {
    /// Generate orders for a given empire based on AiProfile and current state.
    /// Deterministic: no RNG, only state-driven decisions sorted by IDs.
    pub fn generate_orders(
        empire_id: &EmpireId,
        _profile: &AiProfile,
        _state: &SimState,
        _turn: u64,
    ) -> Vec<Order> {
        // Base AI generates no orders; concrete scenarios use scripted orders.
        let _ = empire_id;
        Vec::new()
    }
}
