// projects/libraries/neural/tests/integration_softmax.rs
use ndarray::Array1;
use neural::generation::softmax;

#[test]
fn test_integration_softmax() {
    let logits = Array1::from_vec(vec![1.0, 2.0, 3.0]);
    let probs = softmax(&logits);

    // Probabilities should sum to 1.0
    let sum = probs.sum();
    assert!(
        (sum - 1.0).abs() < 1e-6,
        "softmax output should sum to 1.0, got {}",
        sum
    );

    // Higher logits should produce higher probabilities
    assert!(
        probs[2] > probs[1],
        "logit 3.0 should produce higher probability than 2.0"
    );
    assert!(
        probs[1] > probs[0],
        "logit 2.0 should produce higher probability than 1.0"
    );

    // All probabilities should be non-negative and <= 1.0
    for (i, &prob) in probs.iter().enumerate() {
        assert!(
            (0.0..=1.0).contains(&prob),
            "probability at index {} should be in [0, 1], got {}",
            i,
            prob
        );
    }
}
