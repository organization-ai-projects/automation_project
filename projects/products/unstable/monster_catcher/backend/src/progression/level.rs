pub struct LevelTable;

impl LevelTable {
    pub fn xp_for_level(level: u32) -> u64 {
        (level as u64).pow(3)
    }

    pub fn check_level_up(current_level: u32, total_xp: u64) -> Option<u32> {
        let next = current_level + 1;
        if total_xp >= Self::xp_for_level(next) {
            Some(next)
        } else {
            None
        }
    }
}
