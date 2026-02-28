use crate::combat::{BattleReport, CombatInput, CombatRound};
use crate::model::EmpireId;
use crate::ships::{ShipKind, base_stats};
use std::collections::BTreeMap;

#[allow(dead_code)]
pub struct CombatResolver;

#[allow(dead_code)]
impl CombatResolver {
    pub fn resolve(input: &CombatInput) -> BattleReport {
        let mut attacker_ships = input.attacker_fleet.ships.clone();
        let mut defender_ships = input.defender_fleet.ships.clone();
        let mut rounds = Vec::new();

        for round_number in 1..=6u32 {
            let attacker_power: u64 = attacker_ships
                .iter()
                .map(|(&kind, &count)| base_stats(kind).attack * count as u64)
                .sum();
            let defender_power: u64 = defender_ships
                .iter()
                .map(|(&kind, &count)| base_stats(kind).attack * count as u64)
                .sum();

            let mut attacker_losses: BTreeMap<ShipKind, u32> = BTreeMap::new();
            let mut defender_losses: BTreeMap<ShipKind, u32> = BTreeMap::new();

            for (&kind, count) in &mut defender_ships {
                if *count == 0 {
                    continue;
                }
                let hull = base_stats(kind).hull.max(1);
                let loss = ((attacker_power / hull) as u32).min(*count);
                if loss > 0 {
                    defender_losses.insert(kind, loss);
                    *count -= loss;
                }
            }

            for (&kind, count) in &mut attacker_ships {
                if *count == 0 {
                    continue;
                }
                let hull = base_stats(kind).hull.max(1);
                let loss = ((defender_power / hull) as u32).min(*count);
                if loss > 0 {
                    attacker_losses.insert(kind, loss);
                    *count -= loss;
                }
            }

            rounds.push(CombatRound {
                round_number,
                attacker_losses,
                defender_losses,
            });

            let attacker_alive: u32 = attacker_ships.values().sum();
            let defender_alive: u32 = defender_ships.values().sum();
            if attacker_alive == 0 || defender_alive == 0 {
                break;
            }
        }

        let attacker_alive: u32 = attacker_ships.values().sum();
        let defender_alive: u32 = defender_ships.values().sum();

        let winner: Option<EmpireId> = if attacker_alive > 0 && defender_alive == 0 {
            Some(input.attacker_empire)
        } else if defender_alive > 0 && attacker_alive == 0 {
            Some(input.defender_empire)
        } else if attacker_alive > 0 {
            Some(input.attacker_empire)
        } else {
            None
        };

        BattleReport {
            input: input.clone(),
            rounds,
            winner,
            tick: input.tick,
        }
    }
}
