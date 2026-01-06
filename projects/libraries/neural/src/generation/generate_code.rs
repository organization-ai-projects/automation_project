use crate::network::neural_net::{NetworkError, NeuralNetwork};
use crate::tokenization::rust_tokenizer::RustTokenizer;
use ndarray::Array1;
use thiserror::Error; // Import du trait pour `gen`

#[derive(Debug, Error)]
pub enum GenerationError {
    #[error("Model not loaded")]
    ModelNotLoaded,
    #[error("Tokenization error: {0}")]
    TokenizationError(String),
    #[error("Generation failed: {0}")]
    GenerationFailed(String),
    #[error("Network error: {0}")]
    NetworkError(#[from] NetworkError),
}

#[derive(Debug, Clone)]
pub struct GenerationConfig {
    pub max_new_tokens: usize,
    pub temperature: f32,
    pub top_k: Option<usize>,
    pub top_p: Option<f32>,
    pub stop_token_id: usize,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            max_new_tokens: 512,
            temperature: 0.8,
            top_k: Some(50),
            top_p: Some(0.95),
            stop_token_id: 0,
        }
    }
}

pub struct CodeGenerator {
    model: NeuralNetwork,
    tokenizer: RustTokenizer,
    config: GenerationConfig,
}

impl CodeGenerator {
    pub fn new(model: NeuralNetwork, tokenizer: RustTokenizer, config: GenerationConfig) -> Self {
        Self {
            model,
            tokenizer,
            config,
        }
    }

    pub fn generate(&mut self, prompt: &str) -> Result<String, GenerationError> {
        let prompt_tokens = self.tokenizer.encode(prompt);
        let generated_tokens = self.generate_tokens(prompt_tokens)?;
        let code = self
            .tokenizer
            .decode(&generated_tokens)
            .map_err(|e| GenerationError::TokenizationError(e.to_string()))?;
        Ok(code)
    }

    fn generate_tokens(&mut self, mut tokens: Vec<usize>) -> Result<Vec<usize>, GenerationError> {
        for _ in 0..self.config.max_new_tokens {
            let input = self.tokens_to_input(&tokens)?;
            let logits = self.model.forward(&input)?;
            let next_token = self.sample_token(&logits)?;
            tokens.push(next_token);
            if next_token == self.config.stop_token_id {
                break;
            }
        }
        Ok(tokens)
    }

    fn tokens_to_input(&self, tokens: &[usize]) -> Result<Array1<f64>, GenerationError> {
        let context_size = 10.min(tokens.len());
        let recent_tokens = &tokens[tokens.len().saturating_sub(context_size)..];
        let vocab_size = self.tokenizer.vocab_size();
        let mut input = Array1::<f64>::zeros(vocab_size);
        for &token in recent_tokens {
            if token < vocab_size {
                input[token] += 1.0;
            }
        }
        if context_size > 0 {
            input /= context_size as f64;
        }
        Ok(input)
    }

    fn sample_token(&self, logits: &Array1<f64>) -> Result<usize, GenerationError> {
        let scaled_logits = logits / self.config.temperature as f64;
        let mut probs = softmax(&scaled_logits);

        // Appliquer top-k si configuré
        if let Some(k) = self.config.top_k {
            probs = apply_top_k(&probs, k);
        }

        sample_categorical(&probs)
    }
}

fn softmax(logits: &Array1<f64>) -> Array1<f64> {
    let max_logit = logits.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let exp_logits: Array1<f64> = logits.mapv(|x| (x - max_logit).exp());
    let sum_exp = exp_logits.sum();
    exp_logits / sum_exp
}

fn apply_top_k(probs: &Array1<f64>, k: usize) -> Array1<f64> {
    let mut sorted_probs: Vec<(usize, f64)> = probs.iter().cloned().enumerate().collect();
    sorted_probs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut filtered = Array1::<f64>::zeros(probs.len());
    for &(idx, prob) in sorted_probs.iter().take(k) {
        filtered[idx] = prob;
    }

    filtered
}

fn sample_categorical(probs: &Array1<f64>) -> Result<usize, GenerationError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_softmax() {
        let logits = Array1::from_vec(vec![1.0, 2.0, 3.0]);
        let probs = softmax(&logits);

        // Somme doit être 1
        assert!((probs.sum() - 1.0).abs() < 1e-6);

        // Le plus grand logit doit avoir la plus grande prob
        assert!(probs[2] > probs[1]);
        assert!(probs[1] > probs[0]);
    }

    #[test]
    fn test_top_k() {
        let probs = Array1::from_vec(vec![0.1, 0.2, 0.3, 0.4]);
        let filtered = apply_top_k(&probs, 2);

        // Seulement les 2 meilleurs doivent être non-zero
        assert!(filtered[3] > 0.0); // 0.4
        assert!(filtered[2] > 0.0); // 0.3
        assert_eq!(filtered[1], 0.0);
        assert_eq!(filtered[0], 0.0);
    }

    #[test]
    fn test_sample_categorical() {
        let probs = Array1::from_vec(vec![0.0, 1.0, 0.0]);
        let token = sample_categorical(&probs).unwrap();

        // Doit toujours sampler l'index 1
        assert_eq!(token, 1);
    }
}
