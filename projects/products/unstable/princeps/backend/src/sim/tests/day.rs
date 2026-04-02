use crate::sim::day::Day;

#[test]
fn day_new_starts_empty() {
    let day = Day::new(4);
    assert_eq!(day.number, 4);
    assert!(day.events.is_empty());
    assert!(day.poll.is_none());
    assert!(day.debate.is_none());
}
