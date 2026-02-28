use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::economy::resource_kind::ResourceKind;
use crate::economy::resource_wallet::ResourceWallet;
use crate::fleets::fleet::Fleet;
use crate::fleets::ship_kind::ShipKind;
use crate::map::route::Route;
use crate::map::star_map::StarMap;
use crate::map::star_system::StarSystem;
use crate::map::star_system_id::StarSystemId;
use crate::model::empire_id::EmpireId;
use crate::model::fleet_id::FleetId;
use crate::model::game_id::GameId;
use crate::model::sim_state::{Empire, SimState};
use crate::orders::order::Order;
use crate::orders::order_id::OrderId;
use crate::orders::order_kind::OrderKind;
use crate::tech::tech_tree::TechTree;

/// Raw empire data as loaded from scenario JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioEmpire {
    pub id: String,
    pub name: String,
    pub home_system: String,
    pub resources: BTreeMap<String, i64>,
    pub tech_levels: BTreeMap<String, u32>,
}

/// Raw fleet data as loaded from scenario JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioFleet {
    pub id: String,
    pub empire_id: String,
    pub location: String,
    pub ships: BTreeMap<String, u32>,
}

/// Raw star system as loaded from scenario JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioSystem {
    pub id: String,
    pub name: String,
    pub planets: Vec<String>,
    pub owner: Option<String>,
}

/// Raw route as loaded from scenario JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioRoute {
    pub from: String,
    pub to: String,
    pub distance: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioStarMap {
    pub systems: Vec<ScenarioSystem>,
    pub routes: Vec<ScenarioRoute>,
}

/// Raw scripted order as loaded from scenario JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioOrder {
    pub id: String,
    pub empire_id: String,
    pub kind: String,
    pub params: BTreeMap<String, String>,
}

/// A checkpoint to verify the snapshot hash at a given turn.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub turn: u64,
    pub expected_snapshot_hash: Option<String>,
}

/// Full scenario definition loaded from JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub game_id: String,
    pub empires: Vec<ScenarioEmpire>,
    pub star_map: ScenarioStarMap,
    pub initial_fleets: Vec<ScenarioFleet>,
    /// Orders keyed by turn number as string.
    pub scripted_orders: BTreeMap<String, Vec<ScenarioOrder>>,
    pub scripted_treaty_choices: BTreeMap<String, BTreeMap<String, String>>,
    pub checkpoints: Vec<Checkpoint>,
}

impl Scenario {
    /// Build the initial SimState from this scenario.
    pub fn build_initial_state(&self) -> SimState {
        // Build star map
        let mut star_map = StarMap::new();
        for sys in &self.star_map.systems {
            let system = StarSystem {
                id: StarSystemId(sys.id.clone()),
                name: sys.name.clone(),
                planets: sys.planets.clone(),
                owner: sys.owner.clone(),
            };
            star_map
                .systems
                .insert(StarSystemId(sys.id.clone()), system);
        }
        for route in &self.star_map.routes {
            star_map.routes.push(Route {
                from: StarSystemId(route.from.clone()),
                to: StarSystemId(route.to.clone()),
                distance: route.distance,
            });
        }

        let mut state = SimState::new(GameId(self.game_id.clone()), star_map);

        // Build empires
        for se in &self.empires {
            let mut wallet = ResourceWallet::new();
            for (k, v) in &se.resources {
                let rk = parse_resource_kind(k);
                wallet.add(rk, *v);
            }
            let empire = Empire {
                id: EmpireId(se.id.clone()),
                name: se.name.clone(),
                home_system: se.home_system.clone(),
                resources: wallet,
                tech_tree: TechTree::new(),
            };
            state.empires.insert(EmpireId(se.id.clone()), empire);
        }

        // Build fleets
        for sf in &self.initial_fleets {
            let mut ships: BTreeMap<ShipKind, u32> = BTreeMap::new();
            for (k, v) in &sf.ships {
                let sk = parse_ship_kind(k);
                *ships.entry(sk).or_insert(0) += v;
            }
            let fleet = Fleet {
                id: FleetId(sf.id.clone()),
                empire_id: EmpireId(sf.empire_id.clone()),
                ships,
                location: sf.location.clone(),
                destination: None,
            };
            state.fleets.insert(FleetId(sf.id.clone()), fleet);
        }

        state
    }

    /// Get orders for a given turn (1-based), or empty slice.
    pub fn orders_for_turn(&self, turn: u64) -> Vec<Order> {
        let key = turn.to_string();
        let scenario_orders = match self.scripted_orders.get(&key) {
            Some(v) => v,
            None => return Vec::new(),
        };
        scenario_orders
            .iter()
            .map(|so| Order {
                id: OrderId(so.id.clone()),
                empire_id: EmpireId(so.empire_id.clone()),
                kind: parse_order_kind(&so.kind),
                params: so.params.clone(),
            })
            .collect()
    }
}

fn parse_resource_kind(s: &str) -> ResourceKind {
    match s {
        "Metal" => ResourceKind::Metal,
        "Energy" => ResourceKind::Energy,
        "Food" => ResourceKind::Food,
        "Research" => ResourceKind::Research,
        _ => ResourceKind::Metal,
    }
}

fn parse_ship_kind(s: &str) -> ShipKind {
    match s {
        "Fighter" => ShipKind::Fighter,
        "Cruiser" => ShipKind::Cruiser,
        "Battleship" => ShipKind::Battleship,
        "Transport" => ShipKind::Transport,
        "Scout" => ShipKind::Scout,
        _ => ShipKind::Fighter,
    }
}

fn parse_order_kind(s: &str) -> OrderKind {
    match s {
        "MoveFleet" => OrderKind::MoveFleet,
        "AttackFleet" => OrderKind::AttackFleet,
        "DefendSystem" => OrderKind::DefendSystem,
        "OfferTreaty" => OrderKind::OfferTreaty,
        "AcceptTreaty" => OrderKind::AcceptTreaty,
        "RejectTreaty" => OrderKind::RejectTreaty,
        "Embargo" => OrderKind::Embargo,
        "Invest" => OrderKind::Invest,
        _ => OrderKind::Invest,
    }
}
