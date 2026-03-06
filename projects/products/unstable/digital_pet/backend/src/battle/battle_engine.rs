// projects/products/unstable/digital_pet/backend/src/battle/battle_engine.rs
use crate::battle::battle_report::BattleReport;
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
        if self.finished {
            let winner = self.winner.clone().unwrap_or_else(|| "none".to_string());
            let report = BattleReport {
                winner,
                turns: self.turn,
                pet_hp_remaining: self.pet.hp,
                opponent_hp_remaining: self.opponent.hp,
            };
            self.log.push(format!(
                "Battle result: winner={}, turns={}, pet_hp={}, opponent_hp={}",
                report.winner, report.turns, report.pet_hp_remaining, report.opponent_hp_remaining
            ));
        }
        self.state()
    }

    fn state(&self) -> BattleState {
        let mut log = self.log.clone();
        if self.turn == 0 {
            log.push(format!(
                "Battle started at tick {}",
                self.start_tick.value()
            ));
        }
        BattleState {
            turn: self.turn,
            pet_hp: self.pet.hp,
            opponent_hp: self.opponent.hp,
            finished: self.finished,
            winner: self.winner.clone(),
            log,
        }
    }
}
