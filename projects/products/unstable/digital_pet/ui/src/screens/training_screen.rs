// projects/products/unstable/digital_pet/ui/src/screens/training_screen.rs
use crate::widgets::log_widget::LogWidget;

pub struct TrainingScreen {
    pub result: String,
}

impl TrainingScreen {
    pub fn new(result: String) -> Self {
        Self { result }
    }

    pub fn render(&self) {
        println!("=== Training ===");
        LogWidget::new(vec![self.result.clone()]).render();
    }
}
