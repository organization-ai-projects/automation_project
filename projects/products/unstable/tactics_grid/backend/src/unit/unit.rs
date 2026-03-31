use super::unit_id::UnitId;
use super::team::Team;
use crate::grid::position::Position;
use crate::ability::ability_id::AbilityId;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Unit {
    pub id: UnitId,
    pub name: String,
    pub team: Team,
    pub position: Position,
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub move_range: u32,
    pub abilities: Vec<AbilityId>,
    pub alive: bool,
}

impl Unit {
    pub fn take_damage(&mut self, amount: i32) {
        let effective = (amount - self.defense).max(0);
        self.hp = (self.hp - effective).max(0);
        if self.hp == 0 {
            self.alive = false;
        }
    }

    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }
}
