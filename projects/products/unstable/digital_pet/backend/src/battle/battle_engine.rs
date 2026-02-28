// projects/products/unstable/digital_pet/backend/src/battle/battle_engine.rs
use crate::battle::battle_state::BattleState;
use crate::battle::opponent::Opponent;
use crate::model::pet::Pet;
use crate::time::tick::Tick;

pub struct BattleEngine {
    pet: Pet,
    opponent: Opponent,
    turn: u32,
    finished: bool,
    winner: Option<String>,
    log: Vec<String>,
    start_tick: Tick,
}

impl BattleEngine {
    pub fn new(pet: Pet, start_tick: Tick) -> Self {
        Self {
            opponent: Opponent::default_opponent(),
            turn: 0,
            finished: false,
            winner: None,
            log: vec![],
            start_tick,
            pet,
        }
    }

    pub fn step(&mut self) -> BattleState {
        if self.finished {
            return self.state();
        }
        self.turn += 1;
        let pet_dmg = self.pet.attack.saturating_sub(self.opponent.defense / 2);
        let opp_dmg = self.opponent.attack.saturating_sub(self.pet.defense / 2);
        self.opponent.hp = self.opponent.hp.saturating_sub(pet_dmg);
        self.pet.hp = self.pet.hp.saturating_sub(opp_dmg);
        self.log.push(format!(
            "Turn {}: pet deals {}, opponent deals {}",
            self.turn, pet_dmg, opp_dmg
        ));
        if self.opponent.hp == 0 {
            self.finished = true;
            self.winner = Some("pet".into());
        } else if self.pet.hp == 0 {
            self.finished = true;
            self.winner = Some("opponent".into());
        }
        self.state()
    }

    fn state(&self) -> BattleState {
        BattleState {
            turn: self.turn,
            pet_hp: self.pet.hp,
            opponent_hp: self.opponent.hp,
            finished: self.finished,
            winner: self.winner.clone(),
            log: self.log.clone(),
        }
    }
}
