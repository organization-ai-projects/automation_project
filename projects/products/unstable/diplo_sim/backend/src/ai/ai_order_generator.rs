use super::ai_profile::AiProfile;
use crate::map::territory_id::TerritoryId;
use crate::model::faction_id::FactionId;
use crate::model::game_state::GameState;
use crate::model::unit_id::UnitId;
use crate::orders::order_kind::OrderKind;
use crate::orders::order_set::OrderSet;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

/// Generates orders for a single faction and turn deterministically.
pub fn generate_orders_for_faction(
    base_seed: u64,
    turn_number: u32,
    faction_id: FactionId,
    state: &GameState,
    profile: &AiProfile,
    next_order_id: &mut u32,
) -> OrderSet {
    // Derive a per-turn-per-faction seed
    let turn_seed = base_seed
        .wrapping_add((turn_number as u64).wrapping_mul(0x9e3779b97f4a7c15))
        .wrapping_add((faction_id.0 as u64).wrapping_mul(0x6c62272e07bb0142));
    let mut rng = StdRng::seed_from_u64(turn_seed);

    // Find all units belonging to this faction, sorted by id for determinism
    let mut units: Vec<_> = state
        .units
        .iter()
        .filter(|u| u.faction_id == faction_id)
        .collect();
    units.sort_by_key(|u| u.id);

    let raw: Vec<(UnitId, OrderKind)> = units
        .into_iter()
        .map(|unit| {
            let neighbors = state.map_graph.neighbors(unit.territory_id);
            let should_move =
                !neighbors.is_empty() && rng.random_range(0..100u32) < profile.move_probability;

            let kind = if should_move {
                let idx = rng.random_range(0..neighbors.len());
                OrderKind::Move {
                    target: neighbors[idx],
                }
            } else {
                OrderKind::Hold
            };
            (unit.id, kind)
        })
        .collect();

    OrderSet::from_raw(faction_id, raw, next_order_id)
}

/// Generates a deterministic Move order to a specific territory (used for testing tie-breaking).
pub fn generate_move_order(
    unit_id: UnitId,
    target: TerritoryId,
    next_order_id: &mut u32,
) -> crate::orders::order::Order {
    let id = crate::orders::order_id::OrderId(*next_order_id);
    *next_order_id += 1;
    crate::orders::order::Order {
        id,
        unit_id,
        kind: OrderKind::Move { target },
    }
}
