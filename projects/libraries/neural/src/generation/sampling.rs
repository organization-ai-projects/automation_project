use crate::generation::GenerationError;
use ndarray::Array1;

/// Applies top-k filtering to an array of probabilities.
pub fn apply_top_k(probs: &Array1<f64>, k: usize) -> Array1<f64> {
    let mut sorted_probs: Vec<(usize, f64)> = probs.iter().cloned().enumerate().collect();
    sorted_probs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut filtered = Array1::<f64>::zeros(probs.len());
    for &(idx, prob) in sorted_probs.iter().take(k) {
        filtered[idx] = prob;
    }

    filtered
}

/// Samples a token index from an array of probabilities.
pub fn sample_categorical(probs: &Array1<f64>) -> Result<usize, GenerationError> {
    let sample: f64 = rand::random::<f64>();
    let mut cumsum = 0.0;
    for (idx, &prob) in probs.iter().enumerate() {
        cumsum += prob;
        if sample < cumsum {
            return Ok(idx);
        }
    }
    Ok(probs.len() - 1)
}
