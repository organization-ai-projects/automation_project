use crate::economy::ResourceKind;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum BuildingKind {
    MetalMine,
    CrystalMine,
    DeuteriumSynthesizer,
    SolarPlant,
    RoboticsFactory,
    Shipyard,
    ResearchLab,
    NaniteFactory,
}

impl BuildingKind {
    pub fn costs(&self, level: u32) -> BTreeMap<ResourceKind, u64> {
        let mut m = BTreeMap::new();
        let level = level.max(1);
        match self {
            BuildingKind::MetalMine => {
                m.insert(
                    ResourceKind::Metal,
                    60u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Crystal,
                    15u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
            }
            BuildingKind::CrystalMine => {
                m.insert(
                    ResourceKind::Metal,
                    48u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Crystal,
                    24u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
            }
            BuildingKind::DeuteriumSynthesizer => {
                m.insert(
                    ResourceKind::Metal,
                    225u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Crystal,
                    75u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
            }
            BuildingKind::SolarPlant => {
                m.insert(
                    ResourceKind::Metal,
                    75u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Crystal,
                    30u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
            }
            BuildingKind::RoboticsFactory => {
                m.insert(
                    ResourceKind::Metal,
                    400u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Crystal,
                    120u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Deuterium,
                    200u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
            }
            BuildingKind::Shipyard => {
                m.insert(
                    ResourceKind::Metal,
                    400u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Crystal,
                    200u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Deuterium,
                    100u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
            }
            BuildingKind::ResearchLab => {
                m.insert(
                    ResourceKind::Metal,
                    200u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Crystal,
                    400u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Deuterium,
                    200u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
            }
            BuildingKind::NaniteFactory => {
                m.insert(
                    ResourceKind::Metal,
                    1_000_000u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Crystal,
                    500_000u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Deuterium,
                    100_000u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
            }
        }
        m
    }

    pub fn build_ticks(&self, level: u32) -> u64 {
        let base: u64 = match self {
            BuildingKind::MetalMine => 10,
            BuildingKind::CrystalMine => 10,
            BuildingKind::DeuteriumSynthesizer => 15,
            BuildingKind::SolarPlant => 10,
            BuildingKind::RoboticsFactory => 25,
            BuildingKind::Shipyard => 20,
            BuildingKind::ResearchLab => 20,
            BuildingKind::NaniteFactory => 50,
        };
        base * level as u64
    }
}
