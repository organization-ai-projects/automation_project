// projects/products/stable/varina/backend/src/handler_error.rs

/// Handler error that preserves HTTP status and error code information
#[derive(Debug, Clone)]
pub struct HandlerError {
    pub http_code: u16,
    pub error_code: i32,
    pub message: String,
}

impl HandlerError {
    #[allow(dead_code)]
    pub fn new(http_code: u16, error_code: i32, message: String) -> Self {
        Self {
            http_code,
            error_code,
            message,
        }
    }

    pub fn validation_error(error_code: i32, message: String) -> Self {
        Self {
            http_code: 400,
            error_code,
            message,
        }
    }

    pub fn internal_error(error_code: i32, message: String) -> Self {
        Self {
            http_code: 500,
            error_code,
            message,
        }
    }
}

impl From<String> for HandlerError {
    fn from(message: String) -> Self {
        // Use E_HANDLER_FAILED for generic internal errors
        Self::internal_error(crate::router::E_HANDLER_FAILED, message)
    }
}
