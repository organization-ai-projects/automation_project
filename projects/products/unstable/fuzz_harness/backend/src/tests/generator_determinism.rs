use crate::generator::InputGenerator;

#[test]
fn same_seed_produces_identical_sequence() {
    let mut gen_a = InputGenerator::new(42);
    let mut gen_b = InputGenerator::new(42);

    for _ in 0..100 {
        let a = gen_a.next();
        let b = gen_b.next();
        assert_eq!(a.data, b.data, "mismatch at index {}", a.index);
        assert_eq!(a.index, b.index);
    }
}

#[test]
fn different_seeds_produce_different_sequences() {
    let mut gen_a = InputGenerator::new(1);
    let mut gen_b = InputGenerator::new(2);

    let a = gen_a.next();
    let b = gen_b.next();
    assert_ne!(a.data, b.data);
}

#[test]
fn generator_index_increments() {
    let mut generator = InputGenerator::new(99);
    for i in 0..50 {
        let input = generator.next();
        assert_eq!(input.index, i);
    }
}

#[test]
fn seed_zero_is_handled() {
    let mut generator = InputGenerator::new(0);
    let input = generator.next();
    assert!(!input.data.is_empty());
}
