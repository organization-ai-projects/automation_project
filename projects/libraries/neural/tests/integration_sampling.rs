// projects/libraries/neural/tests/integration_sampling.rs
use ndarray::Array1;
use neural::generation::{apply_top_k, sample_categorical};

#[test]
fn test_integration_apply_top_k() {
    let probs = Array1::from_vec(vec![0.1, 0.2, 0.3, 0.4]);
    let filtered = apply_top_k(&probs, 2);

    // Top 2 probabilities should be preserved
    assert!(
        filtered[3] > 0.0,
        "highest probability (0.4) should be preserved"
    );
    assert!(
        filtered[2] > 0.0,
        "second highest probability (0.3) should be preserved"
    );

    // Lower probabilities should be zeroed out
    assert_eq!(
        filtered[1], 0.0,
        "third probability should be filtered to 0.0"
    );
    assert_eq!(
        filtered[0], 0.0,
        "lowest probability should be filtered to 0.0"
    );

    // Filtered result should still be valid probabilities
    for &val in filtered.iter() {
        assert!(val >= 0.0, "filtered probability should be non-negative");
    }
}

#[test]
fn test_integration_sample_categorical() {
    let probs = Array1::from_vec(vec![0.0, 1.0, 0.0]);
    let token = sample_categorical(&probs).expect("sampling should succeed");

    // With probability 1.0 at index 1, should always sample index 1
    assert_eq!(token, 1, "should sample the token with probability 1.0");

    // Test with uniform distribution
    let uniform_probs = Array1::from_vec(vec![0.33, 0.34, 0.33]);
    let sampled_token = sample_categorical(&uniform_probs)
        .expect("sampling from uniform distribution should succeed");
    assert!(
        sampled_token < 3,
        "sampled token should be within valid range"
    );
}
