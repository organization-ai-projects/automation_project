use super::battle_input::BattleInput;
use super::battle_report::BattleReport;

/// Purely deterministic battle resolution - no RNG.
/// Rule: attacker wins iff attack_power > defense_power; tie goes to defender.
pub struct BattleResolver;

impl BattleResolver {
    pub fn resolve(input: BattleInput) -> BattleReport {
        let attacker_power = input.attacker.attack_power();
        let defender_power = input.defender.defense_power();
        // Attacker wins only if strictly greater
        let attacker_wins = attacker_power > defender_power;
        BattleReport {
            location: input.location,
            attacker_empire: input.attacker.empire_id.clone(),
            attacker_fleet: input.attacker.id.clone(),
            attacker_power,
            defender_empire: input.defender.empire_id.clone(),
            defender_fleet: input.defender.id.clone(),
            defender_power,
            attacker_wins,
        }
    }
}
