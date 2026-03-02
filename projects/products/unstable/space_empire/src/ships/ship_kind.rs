use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ShipKind {
    LightFighter,
    HeavyFighter,
    Cruiser,
    Battleship,
    Battlecruiser,
    Bomber,
    Destroyer,
    Deathstar,
    SmallCargo,
    LargeCargo,
    ColonyShip,
    Recycler,
    EspionageProbe,
}
