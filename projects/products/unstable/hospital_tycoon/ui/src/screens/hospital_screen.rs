// projects/products/unstable/hospital_tycoon/ui/src/screens/hospital_screen.rs

pub struct HospitalScreen {
    pub room_count: usize,
    pub staff_count: usize,
    pub current_tick: u64,
}

impl HospitalScreen {
    pub fn new(room_count: usize, staff_count: usize, current_tick: u64) -> Self {
        Self {
            room_count,
            staff_count,
            current_tick,
        }
    }

    pub fn render(&self) {
        println!("=== Hospital Overview (tick {}) ===", self.current_tick);
        println!("  Rooms: {}", self.room_count);
        println!("  Staff: {}", self.staff_count);
    }
}
