use crate::ships::ShipKind;

pub struct ShipStats {
    pub attack: u64,
    pub shield: u64,
    pub hull: u64,
    pub speed: u64,
    pub cargo: u64,
}

pub fn base_stats(kind: ShipKind) -> ShipStats {
    match kind {
        ShipKind::LightFighter => ShipStats {
            attack: 50,
            shield: 10,
            hull: 400,
            speed: 12500,
            cargo: 50,
        },
        ShipKind::HeavyFighter => ShipStats {
            attack: 150,
            shield: 25,
            hull: 1000,
            speed: 10000,
            cargo: 100,
        },
        ShipKind::Cruiser => ShipStats {
            attack: 400,
            shield: 50,
            hull: 2700,
            speed: 15000,
            cargo: 800,
        },
        ShipKind::Battleship => ShipStats {
            attack: 1000,
            shield: 200,
            hull: 6000,
            speed: 10000,
            cargo: 1500,
        },
        ShipKind::Battlecruiser => ShipStats {
            attack: 700,
            shield: 400,
            hull: 7000,
            speed: 10000,
            cargo: 750,
        },
        ShipKind::Bomber => ShipStats {
            attack: 1000,
            shield: 500,
            hull: 7500,
            speed: 4000,
            cargo: 500,
        },
        ShipKind::Destroyer => ShipStats {
            attack: 2000,
            shield: 500,
            hull: 11000,
            speed: 5000,
            cargo: 2000,
        },
        ShipKind::Deathstar => ShipStats {
            attack: 200000,
            shield: 50000,
            hull: 900000,
            speed: 100,
            cargo: 1000000,
        },
        ShipKind::SmallCargo => ShipStats {
            attack: 5,
            shield: 10,
            hull: 400,
            speed: 5000,
            cargo: 5000,
        },
        ShipKind::LargeCargo => ShipStats {
            attack: 5,
            shield: 25,
            hull: 1200,
            speed: 7500,
            cargo: 25000,
        },
        ShipKind::ColonyShip => ShipStats {
            attack: 50,
            shield: 100,
            hull: 3000,
            speed: 2500,
            cargo: 7500,
        },
        ShipKind::Recycler => ShipStats {
            attack: 1,
            shield: 10,
            hull: 1600,
            speed: 2000,
            cargo: 20000,
        },
        ShipKind::EspionageProbe => ShipStats {
            attack: 0,
            shield: 0,
            hull: 100,
            speed: 100000000,
            cargo: 5,
        },
    }
}
