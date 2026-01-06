use crate::feedback::{FeedbackAdjuster, FeedbackConfig, UserFeedback};
use crate::generation::CodeGenerator;
use crate::network::neural_net::NeuralNetwork;
use crate::tokenization::rust_tokenizer::RustTokenizer;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NeuralError {
    #[error("Generation error: {0}")]
    GenerationError(String),
    #[error("Training error: {0}")]
    TrainingError(String),
    #[error("Model not loaded")]
    ModelNotLoaded,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct SolverResult {
    pub output: String,
    pub confidence: f64,
    pub metadata: Option<String>,
}

/// Solver neural - orchestration interne de neural
pub struct NeuralSolver {
    generator: CodeGenerator,
    feedback_adjuster: FeedbackAdjuster,
    tokenizer: RustTokenizer,
}

impl NeuralSolver {
    pub fn load(
        model_path: &std::path::Path,
        tokenizer_path: &std::path::Path,
    ) -> Result<Self, NeuralError> {
        let tokenizer = RustTokenizer::load(tokenizer_path)
            .map_err(|e| NeuralError::GenerationError(e.to_string()))?;

        let model = NeuralNetwork::load(model_path).map_err(|_| NeuralError::ModelNotLoaded)?;

        let generator = CodeGenerator::new(
            model,
            tokenizer.clone(),
            crate::generation::GenerationConfig::default(),
        );

        let feedback_adjuster = FeedbackAdjuster::new(FeedbackConfig::default());

        Ok(Self {
            generator,
            feedback_adjuster,
            tokenizer,
        })
    }

    pub fn solve(&mut self, input: &str) -> Result<SolverResult, NeuralError> {
        // Tokenize the input
        let input_tokens = self.tokenizer.encode(input);
        let tokenized_input = input_tokens
            .iter()
            .map(|token| token.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        let output = self
            .generator
            .generate(&tokenized_input)
            .map_err(|e| NeuralError::GenerationError(e.to_string()))?;

        // Estimer la confiance (heuristique simple)
        let confidence = self.estimate_confidence(&output);

        Ok(SolverResult {
            output,
            confidence,
            metadata: Some("Neural generation".to_string()),
        })
    }

    pub fn train(&mut self, training_data: Vec<String>) -> Result<(), NeuralError> {
        // TODO: Implémenter training pipeline
        println!("Training on {} examples", training_data.len());
        Ok(())
    }

    pub fn record_feedback(&mut self, feedback: UserFeedback) -> Result<(), NeuralError> {
        self.feedback_adjuster
            .record_feedback(feedback)
            .map_err(|e| NeuralError::TrainingError(e.to_string()))
    }

    pub fn adjust_from_feedback(&mut self) -> Result<(), NeuralError> {
        // TODO: Appliquer les feedbacks au modèle
        let stats = self.feedback_adjuster.feedback_stats();
        println!("Feedback stats: {:?}", stats);
        Ok(())
    }

    fn estimate_confidence(&self, output: &str) -> f64 {
        // Heuristiques simples pour estimer la confiance
        let mut confidence: f64 = 0.75;

        // Pénaliser si trop court ou trop long
        if output.len() < 10 {
            confidence -= 0.2;
        } else if output.len() > 5000 {
            confidence -= 0.1;
        }

        // Bonus si contient des mots-clés Rust
        if output.contains("fn ") || output.contains("struct ") {
            confidence += 0.1;
        }

        confidence.clamp(0.0, 1.0)
    }
}
