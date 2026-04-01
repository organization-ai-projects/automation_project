use crate::math::constants::SOLAR_MASS;
use crate::math::vec3::Vec3;
use crate::structures::structure_id::StructureId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StarClass {
    O,
    B,
    A,
    F,
    G,
    K,
    M,
    WhiteDwarf,
    NeutronStar,
    BlackHole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Star {
    pub id: StructureId,
    pub position: Vec3,
    pub mass: f64,
    pub luminosity: f64,
    pub temperature: f64,
    pub radius: f64,
    pub age_ticks: u64,
    pub class: StarClass,
    pub alive: bool,
}

impl Star {
    pub fn new(id: StructureId, position: Vec3, mass: f64) -> Self {
        let class = Self::classify(mass);
        let luminosity = (mass / SOLAR_MASS).powf(3.5) * 3.828e26;
        let temperature = 5778.0 * (mass / SOLAR_MASS).powf(0.505);
        let radius = (mass / SOLAR_MASS).powf(0.8) * 6.957e8;
        Self {
            id,
            position,
            mass,
            luminosity,
            temperature,
            radius,
            age_ticks: 0,
            class,
            alive: true,
        }
    }

    fn classify(mass: f64) -> StarClass {
        let solar_masses = mass / SOLAR_MASS;
        if solar_masses >= 16.0 {
            StarClass::O
        } else if solar_masses >= 2.1 {
            StarClass::B
        } else if solar_masses >= 1.4 {
            StarClass::A
        } else if solar_masses >= 1.04 {
            StarClass::F
        } else if solar_masses >= 0.8 {
            StarClass::G
        } else if solar_masses >= 0.45 {
            StarClass::K
        } else {
            StarClass::M
        }
    }

    pub fn lifetime_ticks(&self) -> u64 {
        let solar_masses = self.mass / SOLAR_MASS;
        if solar_masses < 0.1 {
            return 1_000_000;
        }
        (10000.0 / solar_masses.powf(2.5)).max(1.0) as u64
    }

    pub fn evolve(&mut self) {
        if !self.alive {
            return;
        }
        self.age_ticks += 1;
        if self.age_ticks >= self.lifetime_ticks() {
            let solar_masses = self.mass / SOLAR_MASS;
            self.alive = false;
            if solar_masses >= 25.0 {
                self.class = StarClass::BlackHole;
            } else if solar_masses >= 8.0 {
                self.class = StarClass::NeutronStar;
            } else {
                self.class = StarClass::WhiteDwarf;
            }
        }
    }
}
