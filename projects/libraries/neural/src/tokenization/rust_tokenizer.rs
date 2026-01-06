use ndarray::Array1;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenizationError {
    #[error("Unknown token: {0}")]
    UnknownToken(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustTokenizer {
    token_to_id: HashMap<String, usize>,
    id_to_token: HashMap<usize, String>,

    // Special tokens
    pad_token: usize,
    eos_token: usize,
    bos_token: usize,
    unk_token: usize,
}

impl RustTokenizer {
    pub fn new(vocab: Vec<String>) -> Self {
        let mut token_to_id = HashMap::new();
        let mut id_to_token = HashMap::new();

        // Special tokens
        let pad_token = 0;
        let eos_token = 1;
        let bos_token = 2;
        let unk_token = 3;

        token_to_id.insert("<PAD>".to_string(), pad_token);
        token_to_id.insert("<EOS>".to_string(), eos_token);
        token_to_id.insert("<BOS>".to_string(), bos_token);
        token_to_id.insert("<UNK>".to_string(), unk_token);

        id_to_token.insert(pad_token, "<PAD>".to_string());
        id_to_token.insert(eos_token, "<EOS>".to_string());
        id_to_token.insert(bos_token, "<BOS>".to_string());
        id_to_token.insert(unk_token, "<UNK>".to_string());

        // Add vocab
        for (idx, token) in vocab.into_iter().enumerate() {
            let id = idx + 4;
            token_to_id.insert(token.clone(), id);
            id_to_token.insert(id, token);
        }

        Self {
            token_to_id,
            id_to_token,
            pad_token,
            eos_token,
            bos_token,
            unk_token,
        }
    }

    /// Encode texte → tokens IDs
    pub fn encode(&self, text: &str) -> Vec<usize> {
        let tokens = Self::tokenize(text);
        let mut ids = vec![self.bos_token];

        for token in tokens {
            let id = self
                .token_to_id
                .get(&token)
                .copied()
                .unwrap_or(self.unk_token);
            ids.push(id);
        }

        ids.push(self.eos_token);
        ids
    }

    /// Decode tokens IDs → texte
    pub fn decode(&self, ids: &[usize]) -> Result<String, TokenizationError> {
        let mut tokens = Vec::new();

        for &id in ids {
            if id == self.bos_token || id == self.eos_token || id == self.pad_token {
                continue;
            }

            let token = self
                .id_to_token
                .get(&id)
                .ok_or(TokenizationError::UnknownToken(id.to_string()))?;

            tokens.push(token.as_str());
        }

        Ok(tokens.join(" "))
    }

    /// Tokenize simple (word-level + symbols)
    fn tokenize(text: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();

        for ch in text.chars() {
            if ch.is_alphanumeric() || ch == '_' {
                current.push(ch);
            } else {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                if !ch.is_whitespace() {
                    tokens.push(ch.to_string());
                }
            }
        }

        if !current.is_empty() {
            tokens.push(current);
        }

        tokens
    }

    pub fn vocab_size(&self) -> usize {
        self.token_to_id.len()
    }

    pub fn save(&self, path: &std::path::Path) -> Result<(), TokenizationError> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load(path: &std::path::Path) -> Result<Self, TokenizationError> {
        let json = std::fs::read_to_string(path)?;
        let tokenizer = serde_json::from_str(&json)?;
        Ok(tokenizer)
    }
}
