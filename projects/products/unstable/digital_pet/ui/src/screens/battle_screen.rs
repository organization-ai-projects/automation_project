// projects/products/unstable/digital_pet/ui/src/screens/battle_screen.rs

pub struct BattleScreen {
    pub turn: u32,
    pub pet_hp: u32,
    pub opponent_hp: u32,
    pub finished: bool,
    pub winner: Option<String>,
}

impl BattleScreen {
    pub fn render(&self) {
        println!("=== Battle (turn {}) ===", self.turn);
        println!(
            "  Pet HP: {}  Opponent HP: {}",
            self.pet_hp, self.opponent_hp
        );
        if self.finished {
            println!("  Winner: {:?}", self.winner);
        }
    }
}
