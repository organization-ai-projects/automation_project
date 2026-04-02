//! projects/products/unstable/rust_language/backend/src/app_error.rs
use ai::AiError;

use crate::ai_assist::{AiErrors, CodeImproverError, ErrorAnalyzerError, TranspileValidatorError};
use crate::engine::EngineErrors;
use std::error::Error;
use std::{fmt, io};

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Engine(EngineErrors),
    Ai(AiErrors),
    Generic(Box<dyn Error + Send + Sync>),
    Usage(&'static str),
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "I/O error: {}", e),
            AppError::Engine(e) => write!(f, "Engine error: {}", e),
            AppError::Ai(e) => write!(f, "AI error: {}", e),
            AppError::Generic(e) => write!(f, "Generic error: {}", e),
            AppError::Usage(msg) => write!(f, "Usage error: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(e) => Some(e),
            AppError::Engine(e) => Some(e),
            AppError::Ai(e) => Some(e),
            AppError::Generic(e) => Some(e.as_ref()),
            AppError::Usage(_) => None,
            AppError::Internal(_) => None,
        }
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::Io(error)
    }
}

impl From<EngineErrors> for AppError {
    fn from(error: EngineErrors) -> Self {
        AppError::Engine(error)
    }
}

impl From<AiErrors> for AppError {
    fn from(error: AiErrors) -> Self {
        AppError::Ai(error)
    }
}

impl From<Box<dyn Error + Send + Sync>> for AppError {
    fn from(error: Box<dyn Error + Send + Sync>) -> Self {
        AppError::Generic(error)
    }
}

impl From<TranspileValidatorError> for AppError {
    fn from(error: TranspileValidatorError) -> Self {
        AppError::Generic(Box::new(error))
    }
}

impl From<ErrorAnalyzerError> for AppError {
    fn from(error: ErrorAnalyzerError) -> Self {
        AppError::Generic(Box::new(error))
    }
}

impl From<CodeImproverError> for AppError {
    fn from(error: CodeImproverError) -> Self {
        AppError::Generic(Box::new(error))
    }
}

impl From<AiError> for AppError {
    fn from(error: AiError) -> Self {
        AppError::Generic(Box::new(error))
    }
}
