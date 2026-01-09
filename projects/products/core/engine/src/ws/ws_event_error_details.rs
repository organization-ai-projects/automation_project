// projects/products/core/engine/src/ws/ws_event_error_details.rs
// Define a new struct to encapsulate error details
pub struct WsEventErrorDetails {
    pub http_code: u16,
    pub code: i32,
    pub message: String,
}
