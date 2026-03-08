use crate::time::turn::Turn;

#[test]
fn turn_next_increments_number() {
    assert_eq!(Turn::new(3).next(), Turn::new(4));
    assert_eq!(Turn::new(3).to_string(), "Turn(3)");
}
