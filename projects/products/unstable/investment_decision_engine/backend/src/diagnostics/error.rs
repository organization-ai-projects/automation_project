use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("input error: {0}")]
    Input(String),

    #[error("parse error: {0}")]
    Parse(String),

    #[error("io error: {0}")]
    Io(String),

    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("decision error: {0}")]
    Decision(String),

    #[error("replay error: {0}")]
    Replay(String),

    #[error("scenario error: {0}")]
    Scenario(String),

    #[error("feature disabled: {0}")]
    FeatureDisabled(String),
}
