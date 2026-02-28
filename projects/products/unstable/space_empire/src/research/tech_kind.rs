use crate::economy::ResourceKind;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TechKind {
    EspionageTech,
    ComputerTech,
    WeaponsTech,
    ShieldingTech,
    ArmourTech,
    EnergyTech,
    HyperspaceTech,
    CombustionDrive,
    ImpulseDrive,
    HyperspaceDrive,
}

impl TechKind {
    pub fn research_ticks(&self, level: u32) -> u64 {
        let base: u64 = match self {
            TechKind::EspionageTech => 20,
            TechKind::ComputerTech => 25,
            TechKind::WeaponsTech => 30,
            TechKind::ShieldingTech => 35,
            TechKind::ArmourTech => 30,
            TechKind::EnergyTech => 25,
            TechKind::HyperspaceTech => 50,
            TechKind::CombustionDrive => 20,
            TechKind::ImpulseDrive => 40,
            TechKind::HyperspaceDrive => 60,
        };
        base * level as u64
    }

    pub fn costs(&self, level: u32) -> BTreeMap<ResourceKind, u64> {
        let mut m = BTreeMap::new();
        let level = level.max(1);
        match self {
            TechKind::EspionageTech => {
                m.insert(
                    ResourceKind::Metal,
                    200u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Crystal,
                    1000u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Deuterium,
                    200u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
            }
            TechKind::ComputerTech => {
                m.insert(
                    ResourceKind::Crystal,
                    400u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Deuterium,
                    600u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
            }
            TechKind::WeaponsTech => {
                m.insert(
                    ResourceKind::Metal,
                    800u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Crystal,
                    200u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
            }
            TechKind::ShieldingTech => {
                m.insert(
                    ResourceKind::Metal,
                    200u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Crystal,
                    600u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
            }
            TechKind::ArmourTech => {
                m.insert(
                    ResourceKind::Metal,
                    1000u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
            }
            TechKind::EnergyTech => {
                m.insert(
                    ResourceKind::Crystal,
                    800u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Deuterium,
                    400u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
            }
            TechKind::HyperspaceTech => {
                m.insert(
                    ResourceKind::Crystal,
                    4000u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Deuterium,
                    2000u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
            }
            TechKind::CombustionDrive => {
                m.insert(
                    ResourceKind::Metal,
                    400u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Deuterium,
                    600u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
            }
            TechKind::ImpulseDrive => {
                m.insert(
                    ResourceKind::Metal,
                    2000u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Crystal,
                    4000u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Deuterium,
                    600u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
            }
            TechKind::HyperspaceDrive => {
                m.insert(
                    ResourceKind::Metal,
                    10000u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Crystal,
                    20000u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
                m.insert(
                    ResourceKind::Deuterium,
                    6000u64.saturating_mul(2u64.saturating_pow(level - 1)),
                );
            }
        }
        m
    }
}
