// projects/libraries/common_json/src/json_error.rs
use std::fmt;

pub use crate::json_error_code::JsonErrorCode;

#[derive(Debug, serde::Serialize)]
pub struct JsonError {
    pub code: JsonErrorCode,
    pub context: Option<String>,
    #[serde(skip_serializing)]
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl JsonError {
    pub fn new(code: JsonErrorCode) -> Self {
        Self {
            code,
            context: None,
            source: None,
        }
    }

    pub fn context(mut self, ctx: impl Into<String>) -> Self {
        self.context = Some(ctx.into());
        self
    }

    pub fn source(mut self, src: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> Self {
        self.source = Some(src.into());
        self
    }

    pub fn message(&self) -> &'static str {
        match self.code {
            JsonErrorCode::Serialize => "serialization error",
            JsonErrorCode::TypeMismatch => "type mismatch",
            JsonErrorCode::MissingField => "missing field",
            JsonErrorCode::IndexOutOfBounds => "index out of bounds",
            JsonErrorCode::InvalidPath => "invalid path",
            JsonErrorCode::UnexpectedNull => "unexpected null value",
            JsonErrorCode::ParseError => "parse error",
            JsonErrorCode::UnsupportedOperation => "unsupported operation",
            JsonErrorCode::Io => "I/O error",
            JsonErrorCode::Custom => "custom error",
            JsonErrorCode::InvalidInteger => "invalid integer",
            JsonErrorCode::InvalidByteValue => "invalid byte value",
            JsonErrorCode::MissingEnumValueError => "missing enum value",
            JsonErrorCode::ValueSerializedBeforeKey => "value serialized before key",
            JsonErrorCode::FieldNotFound => "field not found",
            JsonErrorCode::ExpectedSingleCharacter => "expected single character",
            JsonErrorCode::ValueIsMissing => "value is missing",
        }
    }
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.context {
            Some(ctx) => write!(f, "{}: {}", self.message(), ctx),
            None => write!(f, "{}", self.message()),
        }
    }
}

impl std::error::Error for JsonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_deref()
            .map(|e| e as &(dyn std::error::Error + 'static))
    }
}

pub type JsonResult<T> = Result<T, JsonError>;

impl serde::de::Error for JsonError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        JsonError::new(JsonErrorCode::ParseError).context(msg.to_string())
    }
}

impl serde::ser::Error for JsonError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        JsonError::new(JsonErrorCode::Serialize).context(msg.to_string())
    }
}

impl From<std::io::Error> for JsonError {
    fn from(e: std::io::Error) -> Self {
        JsonError::new(JsonErrorCode::Io)
            .context(e.to_string())
            .source(e)
    }
}
