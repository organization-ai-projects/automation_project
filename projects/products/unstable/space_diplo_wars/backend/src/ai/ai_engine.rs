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
        profile: &AiProfile,
        state: &SimState,
        turn: u64,
    ) -> Vec<Order> {
        // Deterministic baseline behavior while the full AI strategy matures.
        let empire_exists = state.empires.contains_key(empire_id);
        let same_or_future_turn = turn >= state.current_turn.0;
        let profile_is_valid = profile.aggression >= 0.0
            && profile.diplomacy_bias >= 0.0
            && profile.economic_focus >= 0.0;

        if !empire_exists || !same_or_future_turn || !profile_is_valid {
            return Vec::new();
        }

        Vec::new()
    }
}
