// projects/libraries/neural/src/generation/probabilities.rs
use ndarray::Array1;

/// Applies the softmax function to an array of logits.
pub fn softmax(logits: &Array1<f64>) -> Array1<f64> {
    let max_logit = logits.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let exp_logits: Array1<f64> = logits.mapv(|x| (x - max_logit).exp());
    let sum_exp = exp_logits.sum();
    exp_logits / sum_exp
}
