use super::ai_profile::AiProfile;
use crate::map::territory_id::TerritoryId;
use crate::model::faction_id::FactionId;
use crate::model::game_state::GameState;
use crate::model::unit_id::UnitId;
use crate::orders::order::Order;
use crate::orders::order_kind::OrderKind;
use crate::orders::order_set::OrderSet;

/// Generates orders for a single faction and turn deterministically.
pub fn generate_orders_for_faction(
    base_seed: u64,
    turn_number: u32,
    faction_id: FactionId,
    state: &GameState,
    profile: &AiProfile,
    next_order_id: &mut u32,
) -> OrderSet {
    let mut turn_seed = base_seed
        .wrapping_add((turn_number as u64).wrapping_mul(0x9e3779b97f4a7c15))
        .wrapping_add((faction_id.0 as u64).wrapping_mul(0x6c62272e07bb0142));

    // Find all units belonging to this faction, sorted by id for determinism
    let mut units: Vec<_> = state
        .units
        .iter()
        .filter(|u| u.faction_id == faction_id)
        .collect();
    units.sort_by_key(|u| u.id);

    let orders: Vec<Order> = units
        .into_iter()
        .map(|unit| {
            let neighbors = state.map_graph.neighbors(unit.territory_id);
            let should_move = !neighbors.is_empty()
                && deterministic_index(&mut turn_seed, 100) < profile.move_probability as usize;

            if should_move {
                let idx = deterministic_index(&mut turn_seed, neighbors.len());
                generate_move_order(unit.id, neighbors[idx], next_order_id)
            } else {
                let id = crate::orders::order_id::OrderId(*next_order_id);
                *next_order_id += 1;
                Order {
                    id,
                    unit_id: unit.id,
                    kind: OrderKind::Hold,
                }
            }
        })
        .collect();

    OrderSet::new(faction_id, orders)
}

fn deterministic_index(seed: &mut u64, bound: usize) -> usize {
    *seed = splitmix64_next(*seed);
    (*seed as usize) % bound
}

fn splitmix64_next(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
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
