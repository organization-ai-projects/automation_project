// projects/products/unstable/digital_pet/ui/src/screens/pet_screen.rs
use crate::app::app_state::AppState;
use crate::widgets::stat_widget::StatWidget;

pub struct PetScreen {
    state: AppState,
}

impl PetScreen {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub fn render(&self) {
        println!("=== Pet Status (tick {}) ===", self.state.current_tick);
        println!(
            "  Species:   {} (stage {})",
            self.state.species, self.state.evolution_stage
        );
        StatWidget::new("HP", self.state.hp, self.state.max_hp).render();
        StatWidget::new("Hunger", self.state.hunger, 100).render();
        StatWidget::new("Fatigue", self.state.fatigue, 100).render();
        StatWidget::new("Happiness", self.state.happiness, 100).render();
        StatWidget::new("Discipline", self.state.discipline, 100).render();
        if self.state.sick {
            println!("  [SICK]");
        }
    }
}
