use ndarray::Array1;
use neural::generation::softmax;

#[test]
fn test_integration_softmax() {
    let logits = Array1::from_vec(vec![1.0, 2.0, 3.0]);
    let probs = softmax(&logits);

    assert!((probs.sum() - 1.0).abs() < 1e-6);
    assert!(probs[2] > probs[1]);
    assert!(probs[1] > probs[0]);
}
