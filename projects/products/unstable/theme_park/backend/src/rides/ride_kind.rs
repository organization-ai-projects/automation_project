#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// The type/category of a ride. Used for visitor preference matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RideKind {
    Coaster,
    WaterRide,
    CarouselRide,
    FerrisWheel,
    HauntedHouse,
}

impl RideKind {
    pub fn all() -> &'static [RideKind] {
        &[
            RideKind::Coaster,
            RideKind::WaterRide,
            RideKind::CarouselRide,
            RideKind::FerrisWheel,
            RideKind::HauntedHouse,
        ]
    }

    /// Deterministic index — used to cycle preferences from seed.
    pub fn from_index(i: usize) -> RideKind {
        match i % 5 {
            0 => RideKind::Coaster,
            1 => RideKind::WaterRide,
            2 => RideKind::CarouselRide,
            3 => RideKind::FerrisWheel,
            _ => RideKind::HauntedHouse,
        }
    }
}
