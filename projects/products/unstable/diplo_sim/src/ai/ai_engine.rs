use super::ai_order_generator::generate_orders_for_faction;
use super::ai_profile::AiProfile;
use crate::model::faction_id::FactionId;
use crate::model::game_state::GameState;
use crate::orders::order_set::OrderSet;

/// AI engine that generates orders for all AI-controlled factions.
pub struct AiEngine {
    pub base_seed: u64,
    pub profile: AiProfile,
}

impl AiEngine {
    pub fn new(base_seed: u64, profile: AiProfile) -> Self {
        Self { base_seed, profile }
    }

    /// Generate orders for all factions in the current game state.
    pub fn generate_all_orders(&self, state: &GameState, next_order_id: &mut u32) -> Vec<OrderSet> {
        let mut faction_ids: Vec<FactionId> = state.factions.iter().map(|f| f.id).collect();
        faction_ids.sort();

        let turn_number = state.current_turn.number;
        faction_ids
            .into_iter()
            .map(|faction_id| {
                generate_orders_for_faction(
                    self.base_seed,
                    turn_number,
                    faction_id,
                    state,
                    &self.profile,
                    next_order_id,
                )
            })
            .collect()
    }
}
