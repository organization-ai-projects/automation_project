use ndarray::Array1;

use crate::{
    generation::{GenerationConfig, GenerationError, apply_top_k, sample_categorical, softmax},
    network::neural_net::NeuralNetwork,
    tokenization::RustTokenizer,
};

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
            let logits = self
                .model
                .forward(&input)
                .map_err(GenerationError::NetworkError)?;
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

        // Apply top-k filtering if configured
        if let Some(k) = self.config.top_k {
            probs = apply_top_k(&probs, k);
        }

        sample_categorical(&probs)
    }

    /// Serialize the neural network model to disk
    pub fn save(&self, model_path: &std::path::Path) -> Result<(), std::io::Error> {
        println!("Saving CodeGenerator model to {:?}", model_path);

        // Serialize the neural network model
        let serialized_model =
            serde_json::to_string(&self.model).map_err(|e| std::io::Error::other(e.to_string()))?;

        // Write the serialized data to the specified file
        std::fs::write(model_path, serialized_model)
    }

    pub fn train(&mut self, training_data: Vec<String>) -> Result<(), GenerationError> {
        println!(
            "Training CodeGenerator with {} examples",
            training_data.len()
        );

        for example in training_data {
            // Tokenize the input example
            let tokens = self.tokenizer.encode(&example);

            // Convert tokens to input vector
            let input = self.tokens_to_input(&tokens)?;

            // Forward pass
            let output = self
                .model
                .forward(&input)
                .map_err(GenerationError::NetworkError)?;

            // Generate target vector (one-hot encoding for simplicity)
            let target = self.tokens_to_input(&tokens)?;

            // Calculate error (difference between output and target)
            let error = &target - &output;

            // Backward pass to update weights using the error
            self.model
                .backward(&error, 0.01)
                .map_err(GenerationError::NetworkError)?;
        }

        Ok(())
    }

    pub fn get_model(&mut self) -> &mut NeuralNetwork {
        &mut self.model
    }
}
