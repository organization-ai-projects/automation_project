// projects/libraries/neural/tests/integration_sampling.rs
use ndarray::Array1;
use neural::generation::{apply_top_k, sample_categorical};

#[test]
fn test_integration_apply_top_k() {
    let probs = Array1::from_vec(vec![0.1, 0.2, 0.3, 0.4]);
    let filtered = apply_top_k(&probs, 2);

    assert!(filtered[3] > 0.0); // 0.4
    assert!(filtered[2] > 0.0); // 0.3
    assert_eq!(filtered[1], 0.0);
    assert_eq!(filtered[0], 0.0);
}

#[test]
fn test_integration_sample_categorical() {
    let probs = Array1::from_vec(vec![0.0, 1.0, 0.0]);
    let token = sample_categorical(&probs).expect("sample succeeds");

    assert_eq!(token, 1);
}
