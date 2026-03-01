// projects/products/unstable/hospital_tycoon/ui/src/screens/patient_screen.rs

pub struct PatientScreen {
    pub patient_count: usize,
    pub treated_count: u32,
    pub current_tick: u64,
}

impl PatientScreen {
    pub fn new(patient_count: usize, treated_count: u32, current_tick: u64) -> Self {
        Self { patient_count, treated_count, current_tick }
    }

    pub fn render(&self) {
        println!("=== Patients (tick {}) ===", self.current_tick);
        println!("  Active Patients: {}", self.patient_count);
        println!("  Total Treated:   {}", self.treated_count);
    }
}
