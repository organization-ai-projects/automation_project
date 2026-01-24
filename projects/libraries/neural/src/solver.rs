// projects/libraries/neural/src/solver.rs
use crate::feedback::{FeedbackAdjuster, FeedbackConfig, UserFeedback};
use crate::generation::code_generator::CodeGenerator;
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
    #[error("Save error: {0}")]
    SaveError(String),
}

#[derive(Debug, Clone)]
pub struct SolverResult {
    pub output: String,
    pub confidence: f64,
    pub metadata: Option<String>,
}

/// Neural solver - internal orchestration of neural operations
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

        // Estimate confidence (simple heuristic)
        let confidence = self.estimate_confidence(&output);

        Ok(SolverResult {
            output,
            confidence,
            metadata: Some("Neural generation".to_string()),
        })
    }

    /// Saves the neural model and, if provided, the tokenizer to disk
    pub fn save_model(
        &self,
        model_path: &std::path::Path,
        tokenizer_path: Option<&std::path::Path>,
    ) -> Result<(), NeuralError> {
        println!("Saving model to {:?}", model_path);

        // Save the model
        self.generator
            .save(model_path)
            .map_err(|e| NeuralError::SaveError(e.to_string()))?;

        // Save the tokenizer if a path is provided
        if let Some(tokenizer_path) = tokenizer_path {
            println!("Saving tokenizer to {:?}", tokenizer_path);
            // Replace with a real implementation if necessary
            std::fs::write(tokenizer_path, b"tokenizer data")
                .map_err(|e| NeuralError::SaveError(e.to_string()))?;
        }

        Ok(())
    }

    pub fn train(
        &mut self,
        training_data: Vec<String>,
        model_path: &std::path::Path,
    ) -> Result<(), NeuralError> {
        println!("Training on {} examples", training_data.len());

        // Train the model
        self.generator
            .train(training_data)
            .map_err(|e| NeuralError::TrainingError(e.to_string()))?;

        // Save the model after training
        self.save_model(model_path, None)
    }

    pub fn record_feedback(&mut self, feedback: &UserFeedback) -> Result<(), NeuralError> {
        self.feedback_adjuster
            .record_feedback(feedback)
            .map_err(|e| NeuralError::TrainingError(e.to_string()))
    }

    pub fn record_feedback_if_new(
        &mut self,
        feedback_hash: &str,
        input: &str,
        generated_output: &str,
        feedback_type: crate::feedback::FeedbackType,
    ) -> Result<bool, NeuralError> {
        if self.has_seen_feedback(feedback_hash) {
            return Ok(false);
        }

        let user_feedback = UserFeedback::from_parts(input, generated_output, feedback_type);
        self.record_feedback(&user_feedback)?;
        Ok(true)
    }

    pub fn pending_since_last_adjust(&self) -> usize {
        self.feedback_adjuster.pending_feedback_count()
    }

    pub fn adjust_from_feedback(
        &mut self,
        model_path: &std::path::Path,
    ) -> Result<(), NeuralError> {
        let stats = self.feedback_adjuster.feedback_stats();
        println!("Feedback stats: {:?}", stats);

        // Apply adjustments to the model
        self.feedback_adjuster
            .apply_feedback()
            .map_err(|e| NeuralError::TrainingError(e.to_string()))?;

        // Save the model after adjustment
        self.save_model(model_path, None)
    }

    pub fn has_seen_feedback(&self, feedback_hash: &str) -> bool {
        self.feedback_adjuster.has_feedback(feedback_hash)
    }

    pub fn feedback_count(&self) -> usize {
        self.feedback_adjuster.feedback_count()
    }

    pub fn min_feedback_for_adjustment(&self) -> usize {
        self.feedback_adjuster.min_feedback_for_adjustment()
    }

    pub fn adjust_model(&mut self) -> Result<(), NeuralError> {
        let model = self.generator.get_model();
        let tokenizer = &self.tokenizer;

        self.feedback_adjuster
            .adjust_model(model, tokenizer)
            .map(|_| ())
            .map_err(|e| NeuralError::TrainingError(e.to_string()))
    }

    pub fn evaluate_model(&mut self, test_data: Vec<String>) -> Result<f64, NeuralError> {
        if test_data.is_empty() {
            return Err(NeuralError::GenerationError("Test data is empty".into()));
        }

        let mut total_confidence = 0.0;
        for input in &test_data {
            let result = self.solve(input)?;
            total_confidence += result.confidence;
        }

        Ok(total_confidence / test_data.len() as f64)
    }

    fn estimate_confidence(&self, output: &str) -> f64 {
        // Simple heuristics to estimate confidence
        let mut confidence: f64 = 0.75;

        // Penalize if too short or too long
        if output.len() < 10 {
            confidence -= 0.2;
        } else if output.len() > 5000 {
            confidence -= 0.1;
        }

        // Bonus if it contains Rust keywords
        if output.contains("fn ") || output.contains("struct ") {
            confidence += 0.1;
        }

        confidence.clamp(0.0, 1.0)
    }
}
