// projects/products/unstable/digital_pet/ui/src/screens/pet_screen.rs
use crate::app::app_state::AppState;

pub struct PetScreen {
    state: AppState,
}

impl PetScreen {
    pub fn new(state: AppState) -> Self { Self { state } }

    pub fn render(&self) {
        println!("=== Pet Status (tick {}) ===", self.state.current_tick);
        println!("  Species:   {} (stage {})", self.state.species, self.state.evolution_stage);
        println!("  HP:        {}/{}", self.state.hp, self.state.max_hp);
        println!("  Hunger:    {}", self.state.hunger);
        println!("  Fatigue:   {}", self.state.fatigue);
        println!("  Happiness: {}", self.state.happiness);
        println!("  Discipline:{}", self.state.discipline);
        if self.state.sick { println!("  [SICK]"); }
    }
}
